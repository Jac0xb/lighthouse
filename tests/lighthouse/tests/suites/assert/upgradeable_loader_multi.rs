use crate::utils::context::TestContext;
use crate::utils::{create_user_with_balance, set_account_from_refs};
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error_u8,
};
use lighthouse_client::instructions::AssertUpgradeableLoaderAccountMultiBuilder;
use lighthouse_client::types::{
    EquatableOperator, UpgradableBufferAssertion, UpgradeableLoaderStateAssertion,
    UpgradeableLoaderStateType,
};
use solana_program_test::tokio;
use solana_sdk::bpf_loader_upgradeable::{self, UpgradeableLoaderState};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn simple() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    // Assert an uninitialized program account

    let program_pubkey = Keypair::new().encodable_pubkey();
    let authority_address = Keypair::new().encodable_pubkey();
    let program = UpgradeableLoaderState::Buffer {
        authority_address: Some(authority_address),
    };
    let serialized_program = bincode::serialize(&program).unwrap();
    set_account_from_refs(
        context,
        &program_pubkey,
        &serialized_program,
        &bpf_loader_upgradeable::ID,
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[AssertUpgradeableLoaderAccountMultiBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertions(vec![
                UpgradeableLoaderStateAssertion::State {
                    value: UpgradeableLoaderStateType::Buffer,
                    operator: EquatableOperator::Equal,
                },
                UpgradeableLoaderStateAssertion::Buffer(UpgradableBufferAssertion::Authority {
                    value: Some(authority_address),
                    operator: EquatableOperator::Equal,
                }),
                UpgradeableLoaderStateAssertion::Buffer(UpgradableBufferAssertion::Authority {
                    value: Some(user.encodable_pubkey()),
                    operator: EquatableOperator::NotEqual,
                }),
            ])
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let assertions = vec![
        UpgradeableLoaderStateAssertion::State {
            value: UpgradeableLoaderStateType::Buffer,
            operator: EquatableOperator::Equal,
        },
        UpgradeableLoaderStateAssertion::Buffer(UpgradableBufferAssertion::Authority {
            value: Some(authority_address),
            operator: EquatableOperator::Equal,
        }),
        UpgradeableLoaderStateAssertion::Buffer(UpgradableBufferAssertion::Authority {
            value: Some(user.encodable_pubkey()),
            operator: EquatableOperator::NotEqual,
        }),
    ];

    for i in 0..assertions.len() {
        let mut assertions = assertions.clone();
        assertions[i] =
            UpgradeableLoaderStateAssertion::Buffer(UpgradableBufferAssertion::Authority {
                value: Some(user.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            });

        let tx = Transaction::new_signed_with_payer(
            &[AssertUpgradeableLoaderAccountMultiBuilder::new()
                .target_account(program_pubkey)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertions(assertions)
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error_u8(0, 0x1900 + i as u32),
            None,
        )
        .await
        .unwrap();
    }
}
