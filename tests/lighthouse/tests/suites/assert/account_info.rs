use crate::utils::context::TestContext;
use crate::utils::create_user;
use crate::utils::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use lighthouse::error::LighthouseError;
use lighthouse::types::{
    AccountInfoFieldAssertion, AssertionConfigV1, ComparableOperator, EquatableOperator,
};
use rust_sdk::{blackhat_program, LighthouseProgram, TxBuilder};
use solana_program::system_program;
use solana_program_test::tokio;
use solana_sdk::signer::{EncodableKeypair, Signer};

#[tokio::test]
async fn test_hijack_account_ownership() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let mut blackhat_program = blackhat_program::BlackhatProgram {};
    let unprotected_user = create_user(context).await.unwrap();
    let bad_fee_payer = create_user(context).await.unwrap();

    // User loses control of their account to malicious actor.
    let tx = blackhat_program
        .hijack_account_ownership(unprotected_user.pubkey())
        .change_fee_payer(bad_fee_payer.pubkey())
        .to_transaction_and_sign(
            vec![&unprotected_user, &bad_fee_payer],
            context.get_blockhash(),
        )
        .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let user_account = context
        .client()
        .get_account(unprotected_user.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user_account.owner, blackhat::ID);

    // User asserts that their account is owned by the system program after the attack, which should fail.
    let protected_user = create_user(context).await.unwrap();
    let tx = TxBuilder {
        payer: protected_user.pubkey(),
        look_up_tables: None,
        ixs: vec![
            blackhat_program
                .hijack_account_ownership(protected_user.pubkey())
                .ix(),
            program
                .assert_account_info(
                    protected_user.pubkey(),
                    protected_user.pubkey(),
                    lighthouse::types::AccountInfoFieldAssertion::Owner(
                        system_program::id(),
                        EquatableOperator::Equal,
                    ),
                    None,
                )
                .ix(),
        ],
    }
    .to_transaction_and_sign(vec![&protected_user], context.get_blockhash())
    .unwrap();

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_account_balance() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let user_balance = context
        .client()
        .get_balance(user.encodable_pubkey())
        .await
        .unwrap();

    let mut tx_builder = program.assert_account_info(
        user.encodable_pubkey(),
        user.encodable_pubkey(),
        AccountInfoFieldAssertion::Lamports(user_balance - 5000, ComparableOperator::Equal),
        Some(AssertionConfigV1 { verbose: true }),
    );

    process_transaction_assert_success(
        context,
        tx_builder
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();
}
