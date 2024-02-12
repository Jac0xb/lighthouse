use crate::utils::{
    context::TestContext,
    create_mint, create_user, mint_to, process_transaction_assert_failure,
    process_transaction_assert_success,
    utils::{build_tx, to_transaction_error},
};
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::{
    error::LighthouseError,
    types::{AccountInfoDataField, Assertion, Operator, TokenAccountDataField},
};
use rust_sdk::{blackhat_program::BlackhatProgram, LighthouseProgram};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_drain_solana() {
    let context = &mut TestContext::new().await.unwrap();
    let mut lighthouse_program = LighthouseProgram {};
    let mut blackhat_program = BlackhatProgram {};
    let user = create_user(context).await.unwrap();

    let drainer = Keypair::new();
    let user_balance = context
        .client()
        .get_account(user.encodable_pubkey())
        .await
        .unwrap()
        .unwrap()
        .lamports;

    let drainer_ixs = blackhat_program
        .drain_solana(&user, &drainer.encodable_pubkey())
        .ixs;
    let assert_ix = lighthouse_program
        .create_assert_multi(
            &user,
            vec![Assertion::AccountInfoField(
                AccountInfoDataField::Lamports(user_balance - 10_000),
                Operator::GreaterThan,
            )],
            vec![user.encodable_pubkey()],
        )
        .ixs;

    let tx = build_tx(
        [drainer_ixs, assert_ix].concat(),
        vec![&user],
        &user.encodable_pubkey(),
        &mut context.client(),
    )
    .await
    .unwrap();

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await;
}

#[tokio::test]
async fn test_drain_token_account() {
    let context = &mut TestContext::new().await.unwrap();
    let mut lighthouse_program = LighthouseProgram {};
    let mut blackhat_program = BlackhatProgram {};

    let drainer = Keypair::new();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(context, &user).await.unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = mint_to(context, &mint.pubkey(), &user, &user.pubkey(), 69_000)
        .await
        .unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());

    let tx = blackhat_program
        .drain_token_account(&user, &drainer.encodable_pubkey(), &mint.pubkey())
        .append(lighthouse_program.create_assert_compact(
            &user,
            user_ata,
            Assertion::TokenAccountField(TokenAccountDataField::Amount(69_000), Operator::Equal),
        ))
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap();

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await;
}
