use std::path::PathBuf;

use lighthouse_sdk::{
    instructions::AssertAccountDataBuilder,
    transaction::{decode_transaction, encode_transaction, into_instruction},
    types::{DataValueAssertion, EquatableOperator},
};
use litesvm::LiteSVM;
use solana_account::Account;
use solana_keypair::Keypair;
use solana_message::{v1, Address, VersionedMessage};
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::{versioned::VersionedTransaction, InstructionError, TransactionError};

#[test]
fn large_v1_assertion_executes_and_rolls_back_a_failed_transfer() {
    let mut svm = LiteSVM::new();
    let program_path = std::env::var_os("SBF_OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../configs/.programs")
        })
        .join("lighthouse.so");
    svm.add_program_from_file(Address::from(lighthouse_sdk::ID.to_bytes()), program_path)
        .expect("Build the Lighthouse SBF test binary first");
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    let target = [2; 32].into();
    let recipient = [3; 32].into();
    svm.set_account(
        target,
        Account {
            lamports: 10_000_000,
            data: vec![7; 1800],
            owner: lighthouse_sdk::ID.to_bytes().into(),
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    svm.airdrop(&recipient, 1_000_000).unwrap();

    for matches in [true, false] {
        let before = svm.get_account(&recipient).unwrap().lamports;
        let assertion = into_instruction(
            AssertAccountDataBuilder::new()
                .target_account([2; 32].into())
                .offset(0u8.into())
                .assertion(DataValueAssertion::Bytes {
                    value: vec![if matches { 7 } else { 8 }; 1800].into(),
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        );
        let message = v1::Message::try_compile_with_config(
            &payer.pubkey(),
            &[transfer(&payer.pubkey(), &recipient, 10_000), assertion],
            svm.latest_blockhash(),
            v1::TransactionConfig::empty()
                .with_compute_unit_limit(200_000)
                .with_loaded_accounts_data_size_limit(64 * 1024 * 1024)
                .with_priority_fee(5_000),
        )
        .unwrap();
        let transaction =
            VersionedTransaction::try_new(VersionedMessage::V1(message), &[&payer]).unwrap();
        let bytes = encode_transaction(&transaction).unwrap();
        assert!(bytes.len() > 1232 && bytes.len() <= 4096);
        let result = svm.send_transaction(decode_transaction(&bytes).unwrap());
        if matches {
            let metadata = result.unwrap();
            assert!(metadata.compute_units_consumed > 0);
            assert_eq!(
                svm.get_account(&recipient).unwrap().lamports,
                before + 10_000
            );
        } else {
            let failure = result.unwrap_err();
            assert_eq!(
                failure.err,
                TransactionError::InstructionError(1, InstructionError::Custom(6001))
            );
            assert_eq!(svm.get_account(&recipient).unwrap().lamports, before);
        }
    }
}
