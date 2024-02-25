use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::error::LighthouseError;
use lighthouse::types::{Assertion, AssertionConfigV1, EquatableOperator, Operator};
use lighthouse_sdk::LighthouseProgram;
use solana_program::keccak;
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

use crate::utils::context::TestContext;
use crate::utils::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use crate::utils::{create_mint, create_test_account, create_user, mint_to, CreateMintParameters};

#[tokio::test]
async fn test_basic() {
    let ctx = &mut TestContext::new().await.unwrap();
    let program = LighthouseProgram {};
    let user = create_user(ctx).await.unwrap();

    let test_account = create_test_account(ctx, &user, false).await.unwrap();

    let test_account_data = ctx
        .client()
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .unwrap();

    let account_hash = keccak::hashv(&[&test_account_data.data]).0;

    process_transaction_assert_success(
        ctx,
        program
            .create_assert(
                &user,
                test_account.encodable_pubkey(),
                Assertion::AccountDataHash(account_hash, EquatableOperator::Equal, None, None),
                Some(AssertionConfigV1 { verbose: true }),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();

    let (tx, mint) = create_mint(
        ctx,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: None,
            freeze_authority: None,
            decimals: 9,
            mint_to: Some((user.encodable_pubkey(), 100)),
        },
    )
    .await
    .unwrap();
    process_transaction_assert_success(ctx, tx).await.unwrap();

    let token_account =
        get_associated_token_address(&user.encodable_pubkey(), &mint.encodable_pubkey());

    let token_account_data = ctx
        .client()
        .get_account(token_account)
        .await
        .unwrap()
        .unwrap();

    let account_hash = keccak::hashv(&[&token_account_data.data]).0;

    process_transaction_assert_success(
        ctx,
        program
            .create_assert(
                &user,
                token_account,
                Assertion::AccountDataHash(account_hash, EquatableOperator::Equal, None, None),
                Some(AssertionConfigV1 { verbose: true }),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();

    let account_hash = keccak::hashv(&[&token_account_data.data[30..]]).0;

    process_transaction_assert_success(
        ctx,
        program
            .create_assert(
                &user,
                token_account,
                Assertion::AccountDataHash(account_hash, EquatableOperator::Equal, Some(30), None),
                Some(AssertionConfigV1 { verbose: true }),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();

    let account_hash = keccak::hashv(&[&token_account_data.data[29..]]).0;

    process_transaction_assert_failure(
        ctx,
        program
            .create_assert(
                &user,
                token_account,
                Assertion::AccountDataHash(account_hash, EquatableOperator::Equal, Some(30), None),
                Some(AssertionConfigV1 { verbose: true }),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}
