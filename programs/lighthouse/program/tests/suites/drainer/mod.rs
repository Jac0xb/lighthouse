use crate::utils::{
    context::TestContext,
    process_transaction_assert_failure, process_transaction_assert_success,
    program::{
        create_cache_account, create_mint, create_user, find_cache_account, mint_to, Program,
    },
    utils::{build_tx, to_transaction_error},
};
use lighthouse::{
    error::ProgramError,
    structs::{Assertion, Operator, WriteType},
};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_drain_solana() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    let drainer = Keypair::new();
    let user_balance = context
        .client()
        .get_account(user.encodable_pubkey())
        .await
        .unwrap()
        .unwrap()
        .lamports;

    let drainer_ixs = program.drain_solana(&user, &drainer.encodable_pubkey()).ixs;
    let assert_ix = program
        .create_assertion(
            &user,
            vec![Assertion::AccountBalance(
                user_balance - 10_000,
                Operator::GreaterThan,
            )],
            vec![user.encodable_pubkey()],
            None,
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
        Ok(tx),
        to_transaction_error(1, ProgramError::AssertionFailed),
        None,
    )
    .await;
}

#[tokio::test]
async fn test_drain_token_account() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());

    let drainer = Keypair::new();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(context, &user).await.unwrap();
    process_transaction_assert_success(context, Ok(tx)).await;

    let tx = mint_to(context, &mint.pubkey(), &user, &user.pubkey(), 69_000)
        .await
        .unwrap();
    process_transaction_assert_success(context, Ok(tx)).await;

    let drainer_ix = program
        .drain_token_account(&user, &drainer.encodable_pubkey(), &mint.pubkey())
        .ixs;
    // let assert_ix = program
    //     .create_assertion(
    //         &user,
    //         vec![Assertion::TokenAccountBalance(69_000, Operator::Equal)],
    //         vec![user.encodable_pubkey()],
    //         None,
    //     )
    //     .to_instruction();

    let tx = build_tx(
        drainer_ix,
        vec![&user],
        &user.encodable_pubkey(),
        &mut context.client(),
    )
    .await
    .unwrap();

    process_transaction_assert_failure(
        context,
        Ok(tx),
        to_transaction_error(1, ProgramError::AssertionFailed),
        None,
    )
    .await;
}
