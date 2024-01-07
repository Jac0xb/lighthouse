use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::error::LighthouseError;
use lighthouse::structs::{Assertion, Operator};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

use crate::utils::program::{create_mint, mint_to};
use crate::utils::utils::to_transaction_error;
use crate::utils::{
    context::TestContext,
    program::{create_user, Program},
};
use crate::utils::{process_transaction_assert_failure, process_transaction_assert_success};

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    let mut tx_builder = program.create_assert_compact(
        &user,
        user.encodable_pubkey(),
        Assertion::AccountBalance(0, Operator::GreaterThan),
    );

    process_transaction_assert_success(context, tx_builder.to_transaction().await).await;
}

#[tokio::test]
async fn test_compact_token_account() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();
    let (tx, mint) = create_mint(context, &user).await.unwrap();
    process_transaction_assert_success(context, Ok(tx)).await;

    let tx = mint_to(
        context,
        &mint.encodable_pubkey(),
        &user,
        &user.encodable_pubkey(),
        100,
    )
    .await
    .unwrap();
    process_transaction_assert_success(context, Ok(tx)).await;

    let token_account =
        get_associated_token_address(&user.encodable_pubkey(), &mint.encodable_pubkey());

    let mut tx_builder = program.create_assert_compact(
        &user,
        token_account,
        Assertion::LegacyTokenAccountField(
            lighthouse::structs::LegacyTokenAccountDataField::Amount(100),
            Operator::Equal,
        ),
    );

    process_transaction_assert_success(context, tx_builder.to_transaction().await).await;

    let mut tx_builder = program.create_assert_compact(
        &user,
        token_account,
        Assertion::LegacyTokenAccountField(
            lighthouse::structs::LegacyTokenAccountDataField::Amount(100),
            Operator::NotEqual,
        ),
    );

    process_transaction_assert_failure(
        context,
        tx_builder.to_transaction().await,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await;
}
