use crate::utils::blackhat_program::BlackhatProgram;
use crate::utils::tx_builder::TxBuilder;
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse_client::errors::LighthouseError;
use lighthouse_client::instructions::{AssertTokenAccountBuilder, AssertTokenAccountMultiBuilder};
use lighthouse_client::types::{ComparableOperator, EquatableOperator, TokenAccountAssertion};
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
    let tx = Transaction::new_signed_with_payer(
        &[AssertTokenAccountMultiBuilder::new()
            .target_account(token_account)
            .lighthouse_program(lighthouse_client::programs::LIGHTHOUSE_ID)
            .log_level(lighthouse_client::types::LogLevel::PlaintextLog)
            .assertions(vec![
                TokenAccountAssertion::Mint {
                    value: mint.pubkey(),
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::Owner {
                    value: user.pubkey(),
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::Amount {
                    value: 100,
                    operator: ComparableOperator::Equal,
                },
                TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::State {
                    value: TokenAccountState::Frozen as u8,
                    operator: ComparableOperator::NotEqual,
                },
                TokenAccountAssertion::IsNative {
                    value: None,
                    operator: ComparableOperator::Equal,
                },
                TokenAccountAssertion::DelegatedAmount {
                    value: 0,
                    operator: ComparableOperator::LessThanOrEqual,
                },
                TokenAccountAssertion::CloseAuthority {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::TokenAccountOwnerIsDerived,
            ])
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}

// This tests the assumption that non-native accounts cannot be closed by a malicious actor.
#[tokio::test]
async fn set_token_close_authority() {
    let context = &mut TestContext::new().await.unwrap();
    let blackhat_program = BlackhatProgram {};
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

    let blockhash = context.get_blockhash().await;

    process_transaction_assert_success(
        context,
        tx_builder
            .to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
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
        context.get_blockhash().await,
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
    let blackhat_program = BlackhatProgram {};
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
        context.get_blockhash().await,
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
        .to_transaction_and_sign(
            vec![&user],
            user.encodable_pubkey(),
            context.get_blockhash().await,
        )
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
            AssertTokenAccountBuilder::new()
                .target_account(bad_actor_token_account)
                .log_level(lighthouse_client::types::LogLevel::PlaintextLog)
                .assertion(TokenAccountAssertion::Amount {
                    value: 100_000,
                    operator: ComparableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&bad_actor.pubkey()),
        &[&bad_actor],
        context.get_blockhash().await,
    );

    tx.message.recent_blockhash = context.get_blockhash().await;

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
    let blackhat_program = BlackhatProgram {};
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

    let blockhash = context.get_blockhash().await;

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
                AssertTokenAccountBuilder::new()
                    .target_account(token_account)
                    .log_level(lighthouse_client::types::LogLevel::PlaintextLog)
                    .assertion(TokenAccountAssertion::Owner {
                        value: user.pubkey(),
                        operator: EquatableOperator::Equal,
                    })
                    .instruction(),
            ],
            look_up_tables: None,
        }
        .to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
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
    let blackhat_program = BlackhatProgram {};
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
    let blockhash = context.get_blockhash().await;

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
                AssertTokenAccountBuilder::new()
                    .target_account(token_account)
                    .log_level(lighthouse_client::types::LogLevel::PlaintextLog)
                    .assertion(TokenAccountAssertion::TokenAccountOwnerIsDerived)
                    .instruction(),
            ],
            look_up_tables: None,
        }
        .to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
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
    let blackhat_program = BlackhatProgram {};

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

    let tx = Transaction::new_signed_with_payer(
        &[
            blackhat_program
                .drain_token_account(
                    user.encodable_pubkey(),
                    drainer.encodable_pubkey(),
                    mint.pubkey(),
                )
                .ix(),
            AssertTokenAccountBuilder::new()
                .target_account(user_ata)
                .log_level(lighthouse_client::types::LogLevel::PlaintextLog)
                .assertion(TokenAccountAssertion::Amount {
                    value: 69_000,
                    operator: ComparableOperator::Equal,
                })
                .instruction(),
            AssertTokenAccountBuilder::new()
                .target_account(user_ata)
                .log_level(lighthouse_client::types::LogLevel::PlaintextLog)
                .assertion(TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}
