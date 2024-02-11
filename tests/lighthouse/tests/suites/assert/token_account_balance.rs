use anchor_lang::AnchorSerialize;
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::error::LighthouseError;
use lighthouse::types::{Assertion, DataValue, Operator};
use rust_sdk::LighthouseProgram;
use solana_program::keccak;
use solana_program::stake::instruction::create_account_and_delegate_stake;
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_vote_program::vote_instruction::{self, CreateVoteAccountConfig};

use crate::utils::context::{TestContext, DEFAULT_LAMPORTS_FUND_AMOUNT};
use crate::utils::utils::to_transaction_error;
use crate::utils::{
    create_mint, create_test_account, create_user, mint_to, process_transaction_assert_failure,
    process_transaction_assert_success,
};

#[tokio::test]
async fn test_basic() {
    let ctx = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
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
                Assertion::AccountDataHash(account_hash, Operator::Equal, None, None),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
    )
    .await;

    let (tx, mint) = create_mint(ctx, &user).await.unwrap();

    process_transaction_assert_success(ctx, tx).await;

    let tx = mint_to(
        ctx,
        &mint.encodable_pubkey(),
        &user,
        &user.encodable_pubkey(),
        100,
    )
    .await
    .unwrap();
    process_transaction_assert_success(ctx, tx).await;

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
                Assertion::AccountDataHash(account_hash, Operator::Equal, None, None),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
    )
    .await;

    let account_hash = keccak::hashv(&[&token_account_data.data[30..]]).0;

    process_transaction_assert_success(
        ctx,
        program
            .create_assert(
                &user,
                token_account,
                Assertion::AccountDataHash(account_hash, Operator::Equal, Some(30), None),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
    )
    .await;

    let account_hash = keccak::hashv(&[&token_account_data.data[29..]]).0;

    process_transaction_assert_failure(
        ctx,
        program
            .create_assert(
                &user,
                token_account,
                Assertion::AccountDataHash(account_hash, Operator::Equal, Some(30), None),
            )
            .to_transaction_and_sign(vec![&user], ctx.get_blockhash())
            .unwrap(),
        to_transaction_error(0, LighthouseError::AssertionFailed),
        Some(&["1 == 5".to_string(), "256 == 30".to_string()]),
    )
    .await;

    panic!("ass")

    //
    //
    //
    //
    //

    // let stake = Keypair::new();
    // let vote_account = Keypair::new();
    // let node_account = Keypair::new();

    // let _ = ctx.fund_account(user.pubkey(), 100e9 as u64).await;
    // let _ = ctx.fund_account(node_account.pubkey(), 100e9 as u64).await;

    // let ixs = vote_instruction::create_account_with_config(
    //     &user.encodable_pubkey(),
    //     &vote_account.encodable_pubkey(),
    //     &solana_program::vote::state::VoteInit {
    //         node_pubkey: node_account.encodable_pubkey(),
    //         authorized_voter: user.encodable_pubkey(),
    //         authorized_withdrawer: user.encodable_pubkey(),
    //         commission: 0,
    //     },
    //     0,
    //     CreateVoteAccountConfig {
    //         space: 128,
    //         with_seed: None,
    //     },
    // );

    // let tx = Transaction::new_signed_with_payer(
    //     &ixs,
    //     Some(&user.encodable_pubkey()),
    //     &[&user, &vote_account, &node_account],
    //     ctx.get_blockhash(),
    // );

    // process_transaction_assert_success(ctx, tx).await;

    // create_account_and_delegate_stake(
    //     user.encodable_pubkey(),
    //     stake.encodable_pubkey(),
    //     vote_pubkey,
    //     authorized,
    //     lockup,
    //     lamports,
    // )
}
