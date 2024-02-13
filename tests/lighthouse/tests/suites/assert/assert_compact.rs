use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::error::LighthouseError;
use lighthouse::types::{AccountInfoDataField, Assertion, Operator};
use rust_sdk::LighthouseProgram;
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

use crate::utils::context::TestContext;
use crate::utils::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use crate::utils::{create_mint, create_user, mint_to};

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let mut tx_builder = program.create_assert_compact(
        &user,
        user.encodable_pubkey(),
        Assertion::AccountInfoField(AccountInfoDataField::Lamports(0), Operator::GreaterThan),
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

#[tokio::test]
async fn test_compact_token_account() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();
    let (tx, mint) = create_mint(context, &user).await.unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = mint_to(
        context,
        &mint.encodable_pubkey(),
        &user,
        &user.encodable_pubkey(),
        100,
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let token_account =
        get_associated_token_address(&user.encodable_pubkey(), &mint.encodable_pubkey());

    let mut tx_builder = program.create_assert_compact(
        &user,
        token_account,
        Assertion::TokenAccountField(
            lighthouse::types::TokenAccountDataField::Amount(100),
            Operator::Equal,
        ),
    );

    process_transaction_assert_success(
        context,
        tx_builder
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();

    let mut tx_builder = program.create_assert_compact(
        &user,
        token_account,
        Assertion::TokenAccountField(
            lighthouse::types::TokenAccountDataField::Amount(100),
            Operator::NotEqual,
        ),
    );

    process_transaction_assert_failure(
        context,
        tx_builder
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}
