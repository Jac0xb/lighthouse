use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::error::LighthouseError;
use lighthouse::types::{ComparableOperator, EquatableOperator, TokenAccountAssertion};
use rust_sdk::blackhat_program::BlackhatProgram;
use rust_sdk::{blackhat_program, LighthouseProgram, TxBuilder};
use solana_program::program_pack::Pack;
use solana_program::system_instruction::transfer;
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::Transaction;
use spl_token::state::AccountState as TokenAccountState;

use crate::utils::context::TestContext;
use crate::utils::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
    to_transaction_error_u8,
};
use crate::utils::{create_mint, create_user, CreateMintParameters};

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: None,
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 100)),
            decimals: 9,
        },
    )
    .await
    .unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let token_account = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let mut tx_builder = program.assert_token_account_multi(
        user.encodable_pubkey(),
        token_account,
        vec![
            (TokenAccountAssertion::Mint(mint.pubkey(), EquatableOperator::Equal)),
            (TokenAccountAssertion::Owner(user.pubkey(), EquatableOperator::Equal)),
            (TokenAccountAssertion::Amount(100, ComparableOperator::Equal)),
            (TokenAccountAssertion::Delegate(None, EquatableOperator::Equal)),
            (TokenAccountAssertion::State(
                TokenAccountState::Frozen as u8,
                ComparableOperator::NotEqual,
            )),
            (TokenAccountAssertion::IsNative(None, ComparableOperator::Equal)),
            (TokenAccountAssertion::DelegatedAmount(0, ComparableOperator::LessThanOrEqual)),
            (TokenAccountAssertion::CloseAuthority(None, EquatableOperator::Equal)),
            (TokenAccountAssertion::TokenAccountOwnerIsDerived),
        ],
        None,
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

// This tests the assumption that non-native accounts cannot be closed by a malicious actor.
#[tokio::test]
async fn set_token_close_authority() {
    let context = &mut TestContext::new().await.unwrap();
    let mut blackhat_program = blackhat_program::BlackhatProgram {};
    let user = create_user(context).await.unwrap();
    let bad_actor = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: None,
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 100)),
            decimals: 9,
        },
    )
    .await
    .unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let token_account = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let mut tx_builder = blackhat_program.switch_token_account_authority(
        user.encodable_pubkey(),
        Some(bad_actor.pubkey()),
        token_account,
        spl_token::instruction::AuthorityType::CloseAccount,
    );

    process_transaction_assert_success(
        context,
        tx_builder
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();

    // close token account to bad actor

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_associated_token_account::instruction::create_associated_token_account(
                &bad_actor.pubkey(),
                &bad_actor.pubkey(),
                &mint.pubkey(),
                &spl_token::id(),
            ),
            spl_token::instruction::close_account(
                &spl_token::id(),
                &token_account,
                &bad_actor.pubkey(),
                &bad_actor.pubkey(),
                &[],
            )
            .unwrap(),
        ],
        Some(&bad_actor.pubkey()),
        &[&bad_actor],
        context.get_blockhash(),
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error_u8(1, spl_token::error::TokenError::NonNativeHasBalance as u32),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn set_token_close_authority_native() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let mut blackhat_program = blackhat_program::BlackhatProgram {};
    let user = create_user(context).await.unwrap();
    let bad_actor = create_user(context).await.unwrap();

    let native_token_account =
        get_associated_token_address(&user.pubkey(), &spl_token::native_mint::id());

    let bad_actor_token_account =
        get_associated_token_address(&bad_actor.pubkey(), &spl_token::native_mint::id());

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_associated_token_account::instruction::create_associated_token_account(
                &user.pubkey(),
                &user.pubkey(),
                &spl_token::native_mint::id(),
                &spl_token::id(),
            ),
            transfer(&user.pubkey(), &native_token_account, 100_000),
            spl_token::instruction::sync_native(&spl_token::ID, &native_token_account).unwrap(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash(),
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let token_account_data = spl_token::state::Account::unpack(
        &context
            .client()
            .get_account(native_token_account)
            .await
            .unwrap()
            .unwrap()
            .data,
    )
    .unwrap();

    assert_eq!(token_account_data.amount, 100_000);

    // close token account to bad actor
    let tx = blackhat_program
        .switch_token_account_authority(
            user.encodable_pubkey(),
            Some(bad_actor.pubkey()),
            native_token_account,
            spl_token::instruction::AuthorityType::CloseAccount,
        )
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let mut tx = Transaction::new_signed_with_payer(
        &[
            spl_associated_token_account::instruction::create_associated_token_account(
                &bad_actor.pubkey(),
                &bad_actor.pubkey(),
                &spl_token::native_mint::id(),
                &spl_token::id(),
            ),
            spl_token::instruction::close_account(
                &spl_token::id(),
                &native_token_account,
                &bad_actor_token_account,
                &bad_actor.pubkey(),
                &[],
            )
            .unwrap(),
            program
                .assert_token_account(
                    bad_actor.encodable_pubkey(),
                    bad_actor_token_account,
                    TokenAccountAssertion::Amount(100_000, ComparableOperator::Equal),
                    None,
                )
                .ix(),
        ],
        Some(&bad_actor.pubkey()),
        &[&bad_actor],
        context.get_blockhash(),
    );

    tx.message.recent_blockhash = context.get_blockhash();

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(2, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn set_token_owner_attack_assert_owner_equal() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let mut blackhat_program = blackhat_program::BlackhatProgram {};
    let user = create_user(context).await.unwrap();
    let bad_actor = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: None,
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 100)),
            decimals: 9,
        },
    )
    .await
    .unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let token_account = get_associated_token_address(&user.pubkey(), &mint.pubkey());

    process_transaction_assert_failure(
        context,
        TxBuilder {
            ixs: vec![
                blackhat_program
                    .switch_token_account_authority(
                        user.encodable_pubkey(),
                        Some(bad_actor.pubkey()),
                        token_account,
                        spl_token::instruction::AuthorityType::AccountOwner,
                    )
                    .ix(),
                program
                    .assert_token_account(
                        user.encodable_pubkey(),
                        token_account,
                        TokenAccountAssertion::Owner(user.pubkey(), EquatableOperator::Equal),
                        None,
                    )
                    .ix(),
            ],
            payer: user.pubkey(),
            look_up_tables: None,
        }
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap(),
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn set_token_owner_attack_assert_token_owner_derived() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let mut blackhat_program = blackhat_program::BlackhatProgram {};
    let user = create_user(context).await.unwrap();
    let bad_actor = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: None,
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 100)),
            decimals: 9,
        },
    )
    .await
    .unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let token_account = get_associated_token_address(&user.pubkey(), &mint.pubkey());

    process_transaction_assert_failure(
        context,
        TxBuilder {
            ixs: vec![
                blackhat_program
                    .switch_token_account_authority(
                        user.encodable_pubkey(),
                        Some(bad_actor.pubkey()),
                        token_account,
                        spl_token::instruction::AuthorityType::AccountOwner,
                    )
                    .ix(),
                program
                    .assert_token_account(
                        user.encodable_pubkey(),
                        token_account,
                        TokenAccountAssertion::TokenAccountOwnerIsDerived,
                        None,
                    )
                    .ix(),
            ],
            payer: user.pubkey(),
            look_up_tables: None,
        }
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap(),
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_drain_token_account() {
    let context = &mut TestContext::new().await.unwrap();
    let mut lighthouse_program = LighthouseProgram {};
    let mut blackhat_program = BlackhatProgram {};

    let drainer = Keypair::new();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());

    let tx = blackhat_program
        .drain_token_account(
            user.encodable_pubkey(),
            drainer.encodable_pubkey(),
            mint.pubkey(),
        )
        .append(lighthouse_program.assert_token_account(
            user.encodable_pubkey(),
            user_ata,
            TokenAccountAssertion::Amount(69_000, ComparableOperator::Equal),
            None,
        ))
        .append(lighthouse_program.assert_token_account(
            user.encodable_pubkey(),
            user_ata,
            TokenAccountAssertion::Delegate(None, EquatableOperator::Equal),
            None,
        ))
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
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
