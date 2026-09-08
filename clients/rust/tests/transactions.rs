use base64::{engine::general_purpose::STANDARD, Engine};
use lighthouse_sdk::{
    instructions::AssertAccountInfoBuilder,
    transaction::{
        append_instructions, decode_transaction, decompile_instructions, encode_transaction,
        into_instruction, TransactionError,
    },
    types::{AccountInfoAssertion, IntegerOperator},
};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::{legacy, v0, v1, VersionedMessage};
use solana_program::pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

fn assertion() -> Instruction {
    into_instruction(
        AssertAccountInfoBuilder::new()
            .target_account(Pubkey::new_from_array([2; 32]))
            .assertion(AccountInfoAssertion::Lamports {
                value: 42,
                operator: IntegerOperator::Equal,
            })
            .instruction(),
    )
}

fn transaction(version: i8, instructions: &[Instruction]) -> VersionedTransaction {
    let payer = [1; 32].into();
    let blockhash = [9; 32].into();
    let message = match version {
        -1 => VersionedMessage::Legacy(legacy::Message::new_with_blockhash(
            instructions,
            Some(&payer),
            &blockhash,
        )),
        0 => VersionedMessage::V0(
            v0::Message::try_compile(&payer, instructions, &[], blockhash).unwrap(),
        ),
        1 => VersionedMessage::V1(
            v1::Message::try_compile_with_config(
                &payer,
                instructions,
                blockhash,
                v1::TransactionConfig::empty()
                    .with_compute_unit_limit(200_000)
                    .with_loaded_accounts_data_size_limit(64 * 1024)
                    .with_heap_size(64 * 1024)
                    .with_priority_fee(5_000),
            )
            .unwrap(),
        ),
        _ => unreachable!(),
    };
    VersionedTransaction {
        signatures: vec![Default::default(); usize::from(message.header().num_required_signatures)],
        message,
    }
}

#[test]
fn append_preserves_version_lifetime_config_and_account_roles() {
    let instruction = Instruction {
        program_id: [3; 32].into(),
        accounts: vec![
            AccountMeta::new([2; 32].into(), false),
            AccountMeta::new_readonly([4; 32].into(), true),
            AccountMeta::new_readonly([5; 32].into(), false),
        ],
        data: vec![7, 8, 9],
    };
    for version in [-1, 0, 1] {
        let original = transaction(version, std::slice::from_ref(&instruction));
        let appended = append_instructions(&original, &[assertion()]).unwrap();
        assert_eq!(appended.version(), original.version());
        assert_eq!(
            appended.message.recent_blockhash(),
            original.message.recent_blockhash()
        );
        assert_eq!(
            appended.message.static_account_keys()[0],
            original.message.static_account_keys()[0]
        );
        assert_eq!(appended.signatures.len(), 2);
        let instructions = decompile_instructions(&appended).unwrap();
        assert_eq!(instructions[0], instruction);
        assert_eq!(instructions[1].data, assertion().data);
        assert!(instructions[1].accounts[0].is_writable);
        assert_eq!(original.message.instructions().len(), 1);
        if let (VersionedMessage::V1(before), VersionedMessage::V1(after)) =
            (&original.message, &appended.message)
        {
            assert_eq!(after.config, before.config);
        }
        let bytes = encode_transaction(&appended).unwrap();
        assert_eq!(decode_transaction(&bytes).unwrap(), appended);
    }
}

#[test]
fn v1_large_transaction_has_prefix_and_signatures_at_the_tail() {
    let mut large = assertion();
    large.data = vec![7; 1800];
    let mut tx = append_instructions(&transaction(1, &[large.clone()]), &[assertion()]).unwrap();
    tx.signatures[0] = [42; 64].into();
    let bytes = encode_transaction(&tx).unwrap();
    assert!(bytes.len() > 1232 && bytes.len() <= 4096);
    assert_eq!(bytes[0], 0x81);
    assert_eq!(&bytes[bytes.len() - 64..], &[42; 64]);
    assert_eq!(&bytes[..bytes.len() - 64], tx.message.serialize());
    assert_eq!(decode_transaction(&bytes).unwrap(), tx);
    assert_eq!(
        decompile_instructions(&tx).unwrap(),
        vec![large.clone(), assertion()]
    );
    assert!(matches!(
        append_instructions(&tx, &[assertion()]),
        Err(TransactionError::AlreadySigned)
    ));
    for version in [-1, 0] {
        assert!(matches!(
            encode_transaction(&transaction(version, &[large.clone()])),
            Err(TransactionError::TooLarge { limit: 1232, .. })
        ));
    }
}

#[test]
fn v1_checks_size_after_appending_including_signature_slots() {
    let mut instruction = assertion();
    let overhead = encode_transaction(&transaction(
        1,
        &[Instruction {
            data: vec![],
            ..instruction.clone()
        }],
    ))
    .unwrap()
    .len();
    instruction.data = vec![7; 4096 - overhead];
    let full = transaction(1, &[instruction.clone()]);
    assert_eq!(encode_transaction(&full).unwrap().len(), 4096);
    assert!(matches!(
        append_instructions(&full, &[assertion()]),
        Err(TransactionError::TooLarge { limit: 4096, .. })
    ));
    instruction.data.push(7);
    assert!(encode_transaction(&transaction(1, &[instruction])).is_err());
}

#[test]
fn rejects_malformed_wire_data_and_invalid_v1_accounts() {
    let valid = transaction(1, &[assertion()]);
    let bytes = encode_transaction(&valid).unwrap();
    for length in [0, 1, 8, 42, bytes.len() - 1] {
        assert!(decode_transaction(&bytes[..length]).is_err());
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_transaction(&trailing).is_err());
    let mut bad_version = bytes.clone();
    bad_version[0] = 0x82;
    assert!(decode_transaction(&bad_version).is_err());
    let mut bad_mask = bytes;
    bad_mask[4] &= !2;
    assert!(decode_transaction(&bad_mask).is_err());

    for invalid_index in [false, true] {
        let mut invalid = valid.clone();
        if let VersionedMessage::V1(message) = &mut invalid.message {
            if invalid_index {
                message.instructions[0].accounts[0] = 64;
            } else {
                message.account_keys[1] = message.account_keys[0];
            }
        }
        assert!(encode_transaction(&invalid).is_err());
        assert!(append_instructions(&invalid, &[assertion()]).is_err());
    }
}

#[test]
fn v1_checks_account_instruction_and_signer_limits() {
    let mut instruction = assertion();
    instruction.accounts = (2..64)
        .map(|index| AccountMeta::new_readonly([index; 32].into(), false))
        .collect();
    assert!(encode_transaction(&transaction(1, &[instruction.clone()])).is_ok());
    instruction
        .accounts
        .push(AccountMeta::new_readonly([64; 32].into(), false));
    assert!(encode_transaction(&transaction(1, &[instruction])).is_err());

    let full = transaction(1, &vec![assertion(); 64]);
    assert!(encode_transaction(&full).is_ok());
    assert!(append_instructions(&full, &[assertion()]).is_err());

    let mut instruction = assertion();
    instruction.accounts = (2..13)
        .map(|index| AccountMeta::new_readonly([index; 32].into(), true))
        .collect();
    assert!(encode_transaction(&transaction(1, &[instruction.clone()])).is_ok());
    instruction
        .accounts
        .push(AccountMeta::new_readonly([13; 32].into(), true));
    assert!(encode_transaction(&transaction(1, &[instruction])).is_err());
}

#[test]
fn v0_lookup_tables_are_rejected_without_losing_accounts() {
    let mut tx = transaction(0, &[assertion()]);
    if let VersionedMessage::V0(message) = &mut tx.message {
        message
            .address_table_lookups
            .push(v0::MessageAddressTableLookup {
                account_key: [6; 32].into(),
                writable_indexes: vec![0],
                readonly_indexes: vec![],
            });
    }
    assert!(matches!(
        append_instructions(&tx, &[assertion()]),
        Err(TransactionError::AddressTableLookupsNotSupported)
    ));
}

#[test]
fn existing_helpers_sign_all_versions_with_current_solana_keypairs() {
    let signer = Keypair::new();
    for version in [-1, 0, 1] {
        let mut tx = transaction(version, &[assertion()]);
        match &mut tx.message {
            VersionedMessage::Legacy(message) => message.account_keys[0] = signer.pubkey(),
            VersionedMessage::V0(message) => message.account_keys[0] = signer.pubkey(),
            VersionedMessage::V1(message) => message.account_keys[0] = signer.pubkey(),
        }
        let signed = lighthouse_sdk::utils::append_instructions_to_versioned_transaction(
            &tx,
            vec![assertion()],
            &[&signer],
        )
        .unwrap();
        assert_eq!(signed.verify_with_results(), vec![true]);
        assert_eq!(signed.version(), tx.version());
        assert_eq!(
            signed.message.recent_blockhash(),
            tx.message.recent_blockhash()
        );
        assert_eq!(
            lighthouse_sdk::utils::decompile_instruction_from_versioned_transaction(&signed)
                .unwrap()
                .len(),
            2
        );
        assert!(
            lighthouse_sdk::utils::append_instructions_to_versioned_transaction(
                &signed,
                vec![assertion()],
                &[&signer],
            )
            .is_err()
        );
        assert!(
            lighthouse_sdk::utils::append_instructions_to_versioned_transaction(
                &tx,
                vec![assertion()],
                &[],
            )
            .is_err()
        );
    }
}

#[test]
fn legacy_helper_keeps_the_blockhash_and_accepts_unsigned_signature_slots() {
    let tx = transaction(-1, &[assertion()])
        .into_legacy_transaction()
        .unwrap();
    let appended =
        lighthouse_sdk::utils::append_instructions_to_transaction(&tx, vec![assertion()]).unwrap();
    assert_eq!(
        appended.message.recent_blockhash,
        tx.message.recent_blockhash
    );
    assert_eq!(
        lighthouse_sdk::utils::decompile_instruction_from_transaction(&appended).unwrap(),
        vec![assertion(), assertion()]
    );
}

#[test]
fn v1_wire_bytes_match_the_kit_fixture() {
    let bytes = STANDARD
        .decode(include_str!("../../../tests/fixtures/v1-lighthouse.base64").trim())
        .unwrap();
    let expected = transaction(1, &[assertion()]);
    assert_eq!(decode_transaction(&bytes).unwrap(), expected);
    assert_eq!(encode_transaction(&expected).unwrap(), bytes);
}
