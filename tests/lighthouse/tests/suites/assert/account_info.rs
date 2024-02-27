use crate::utils::blackhat_program::BlackhatProgram;
use crate::utils::context::TestContext;
use crate::utils::create_user;
use crate::utils::tx_builder::TxBuilder;
use crate::utils::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use lighthouse_client::errors::LighthouseError;
use lighthouse_client::instructions::{
    AssertAccountDataBuilder, AssertAccountInfoBuilder, AssertAccountInfoCpiBuilder,
};
use lighthouse_client::types::{AccountInfoAssertion, ComparableOperator, EquatableOperator};
use solana_program::system_program;
use solana_program_test::tokio;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn test_hijack_account_ownership() {
    let context = &mut TestContext::new().await.unwrap();
    let blackhat_program = BlackhatProgram {};
    let unprotected_user = create_user(context).await.unwrap();
    let bad_fee_payer = create_user(context).await.unwrap();

    // User loses control of their account to malicious actor.
    let tx = blackhat_program
        .hijack_account_ownership(unprotected_user.pubkey())
        .to_transaction_and_sign(
            vec![&unprotected_user, &bad_fee_payer],
            bad_fee_payer.encodable_pubkey(),
            context.get_blockhash().await,
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
        look_up_tables: None,
        ixs: vec![
            blackhat_program
                .hijack_account_ownership(protected_user.pubkey())
                .ix(),
            AssertAccountInfoBuilder::new()
                .target_account(protected_user.pubkey())
                .account_info_assertion(AccountInfoAssertion::Owner(
                    system_program::id(),
                    EquatableOperator::Equal,
                ))
                .instruction(),
        ],
    }
    .to_transaction_and_sign(
        vec![&protected_user],
        protected_user.encodable_pubkey(),
        context.get_blockhash().await,
    )
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
    let user = create_user(context).await.unwrap();

    let user_balance = context
        .client()
        .get_balance(user.encodable_pubkey())
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .account_info_assertion(AccountInfoAssertion::Lamports(
                user_balance - 5000,
                ComparableOperator::Equal,
            ))
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}
