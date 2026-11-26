use base64::{engine::general_purpose::STANDARD, Engine as _};
use lighthouse_sdk::{
    instructions::AssertAccountInfoBuilder,
    transaction::{
        append_instructions, decode_transaction, decompile_instructions, encode_transaction,
        TransactionError, MAX_LEGACY_TRANSACTION_SIZE,
    },
    types::{AccountInfoAssertion, IntegerOperator},
    Address,
};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::{legacy, v0, v1, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

fn assertion() -> Instruction {
    AssertAccountInfoBuilder::new()
        .target_account(Address::new_from_array([2; 32]))
        .assertion(AccountInfoAssertion::Lamports {
            value: 42,
            operator: IntegerOperator::Equal,
        })
        .instruction()
}

fn transaction(version: i8, instructions: &[Instruction]) -> VersionedTransaction {
    let payer = Address::new_from_array([1; 32]);
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
        signatures: vec![
            Default::default();
            usize::from(message.header().num_required_signatures)
        ],
        message,
    }
}

#[test]
fn append_preserves_version_lifetime_config_and_account_roles() {
    let instruction = Instruction {
        program_id: Address::new_from_array([3; 32]),
        accounts: vec![
            AccountMeta::new(Address::new_from_array([2; 32]), false),
            AccountMeta::new_readonly(Address::new_from_array([4; 32]), true),
            AccountMeta::new_readonly(Address::new_from_array([5; 32]), false),
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
    let mut tx = append_instructions(&transaction(1, &[large]), &[assertion()]).unwrap();
    tx.signatures[0] = [42; 64].into();

    let bytes = encode_transaction(&tx).unwrap();
    assert!(bytes.len() > MAX_LEGACY_TRANSACTION_SIZE && bytes.len() <= v1::MAX_TRANSACTION_SIZE);
    assert_eq!(bytes[0], 0x81);
    assert_eq!(&bytes[bytes.len() - 64..], &[42; 64]);
    assert_eq!(&bytes[..bytes.len() - 64], tx.message.serialize());
    assert_eq!(decode_transaction(&bytes).unwrap(), tx);

    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(matches!(
        decode_transaction(&trailing),
        Err(TransactionError::InvalidTransaction(_))
    ));
}

#[test]
fn size_limits_are_enforced_for_legacy_v0_and_v1() {
    let mut empty = assertion();
    empty.data.clear();
    let v1_overhead = encode_transaction(&transaction(1, std::slice::from_ref(&empty)))
        .unwrap()
        .len();

    let mut full_v1 = empty.clone();
    full_v1.data = vec![1; v1::MAX_TRANSACTION_SIZE - v1_overhead];
    let at_limit = transaction(1, &[full_v1]);
    assert_eq!(
        encode_transaction(&at_limit).unwrap().len(),
        v1::MAX_TRANSACTION_SIZE
    );
    assert!(matches!(
        append_instructions(&at_limit, &[assertion()]),
        Err(TransactionError::TooLarge {
            limit: v1::MAX_TRANSACTION_SIZE,
            ..
        })
    ));

    let mut large = assertion();
    large.data = vec![7; 1800];
    for version in [-1, 0] {
        assert!(matches!(
            encode_transaction(&transaction(version, std::slice::from_ref(&large))),
            Err(TransactionError::TooLarge {
                limit: MAX_LEGACY_TRANSACTION_SIZE,
                ..
            })
        ));
    }

    assert!(matches!(
        decode_transaction(&vec![0u8; v1::MAX_TRANSACTION_SIZE + 1]),
        Err(TransactionError::TooLarge {
            limit: v1::MAX_TRANSACTION_SIZE,
            ..
        })
    ));
}

#[test]
fn v1_structural_limits_are_validated() {
    // v1 supports > 64 static accounts (up to wire size limit), unlike older v0/legacy limits.
    let mut instruction = assertion();
    instruction.accounts = (2..80u8)
        .map(|index| AccountMeta::new_readonly(Address::new_from_array([index; 32]), false))
        .collect();
    assert!(encode_transaction(&transaction(1, std::slice::from_ref(&instruction))).is_ok());

    // 130 static accounts (130 * 32 = 4160 bytes) exceeds the 4096-byte v1 transaction limit.
    instruction.accounts = (2..130u8)
        .map(|index| AccountMeta::new_readonly(Address::new_from_array([index; 32]), false))
        .collect();
    assert!(matches!(
        encode_transaction(&transaction(1, &[instruction])),
        Err(TransactionError::TooLarge {
            limit: v1::MAX_TRANSACTION_SIZE,
            ..
        })
    ));

    // > u8::MAX (255) instructions are rejected by v1::Message::validate().
    let mut too_many_ixs = transaction(1, &[assertion()]);
    if let VersionedMessage::V1(message) = &mut too_many_ixs.message {
        let compiled = message.instructions[0].clone();
        message.instructions = vec![compiled; usize::from(u8::MAX) + 1];
    }
    assert!(matches!(
        encode_transaction(&too_many_ixs),
        Err(TransactionError::InvalidTransaction(_))
    ));

    // Program ID index == 0 (fee payer) is rejected by v1::Message::validate().
    let mut invalid_program = transaction(1, &[assertion()]);
    if let VersionedMessage::V1(message) = &mut invalid_program.message {
        message.instructions[0].program_id_index = 0;
    }
    assert!(matches!(
        encode_transaction(&invalid_program),
        Err(TransactionError::InvalidTransaction(_))
    ));
}

#[test]
fn v0_lookup_tables_are_rejected_without_losing_accounts() {
    let mut tx = transaction(0, &[assertion()]);
    if let VersionedMessage::V0(message) = &mut tx.message {
        message
            .address_table_lookups
            .push(v0::MessageAddressTableLookup {
                account_key: Address::new_from_array([6; 32]),
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
fn utils_helpers_sign_all_versions_with_current_solana_keypairs() {
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
        assert_eq!(signed.signatures.len(), 1);
        assert!(signed.signatures[0].verify(signer.pubkey().as_ref(), &signed.message.serialize()));
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
fn legacy_helper_preserves_blockhash_and_accepts_unsigned_signature_slots() {
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
