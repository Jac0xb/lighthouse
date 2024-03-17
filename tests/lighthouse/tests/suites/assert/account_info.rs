use crate::utils::blackhat_program::BlackhatProgram;
use crate::utils::context::TestContext;
use crate::utils::tx_builder::TxBuilder;
use crate::utils::{create_mint, create_test_account, create_user, CreateMintParameters};
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use lighthouse_client::errors::LighthouseError;
use lighthouse_client::instructions::AssertAccountInfoBuilder;
use lighthouse_client::types::{
    AccountInfoAssertion, EquatableOperator, IntegerOperator, KnownProgram,
};
use solana_program_test::tokio;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::{keccak, system_program};
use spl_associated_token_account::get_associated_token_address;

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
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(AccountInfoAssertion::Owner {
                    value: system_program::id(),
                    operator: EquatableOperator::Equal,
                })
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
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::Lamports {
                value: user_balance - 5000,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}

#[tokio::test]
async fn data_hash() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    let test_account = create_test_account(ctx, &user, false).await.unwrap();

    let test_account_data = ctx
        .client()
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .unwrap();

    let account_hash = keccak::hashv(&[&test_account_data.data]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: account_hash,
                start: None,
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

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

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(token_account)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: account_hash,
                start: None,
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let account_hash = keccak::hashv(&[&token_account_data.data[30..]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(token_account)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: account_hash,
                start: Some(30),
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let account_hash = keccak::hashv(&[&token_account_data.data[29..]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(token_account)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: account_hash,
                start: Some(30),
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn simple() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    let test_account = create_test_account(ctx, &user, false).await.unwrap();

    // Test Owner

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::Owner {
                value: system_program::ID,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // fail with NotEqual

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::Owner {
                value: system_program::ID,
                operator: EquatableOperator::NotEqual,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Fail with wrong owner

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::Owner {
                value: user.encodable_pubkey(),
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test KnownOwner

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::KnownOwner {
                value: KnownProgram::System,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail with NotEqual

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::KnownOwner {
                value: KnownProgram::System,
                operator: EquatableOperator::NotEqual,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Fail with wrong owner

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::KnownOwner {
                value: KnownProgram::BpfLoader,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test Lamports

    let test_balance = ctx
        .client()
        .get_balance(test_account.encodable_pubkey())
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Lamports {
                    value: test_balance / 2,
                    operator: IntegerOperator::GreaterThanOrEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Lamports {
                    value: test_balance * 2,
                    operator: IntegerOperator::LessThanOrEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Lamports {
                    value: test_balance,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Lamports {
                    value: 0,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(AccountInfoAssertion::Lamports {
                value: test_balance + 1,
                operator: IntegerOperator::GreaterThanOrEqual,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test DataLength

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(AccountInfoAssertion::DataLength {
                value: 0,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail

    let test_account_len = ctx
        .client()
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .unwrap()
        .data
        .len() as u64;

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: 128,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: test_account_len,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: test_account_len + 1,
                    operator: IntegerOperator::LessThan,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Test Executable

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Executable {
                    value: false,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Executable {
                    value: true,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(lighthouse_client::ID)
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Executable {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(lighthouse_client::ID)
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::Executable {
                    value: false,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(lighthouse_client::ID)
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(AccountInfoAssertion::Executable {
                value: false,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test IsSigner

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsSigner {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsSigner {
                    value: false,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(lighthouse_client::ID)
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsSigner {
                    value: true,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(lighthouse_client::ID)
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(AccountInfoAssertion::IsSigner {
                value: true,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test IsWritable

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsWritable {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsWritable {
                    value: false,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(lighthouse_client::ID)
                .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsWritable {
                    value: false,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(lighthouse_client::ID)
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(AccountInfoAssertion::IsWritable {
                value: false,
                operator: EquatableOperator::NotEqual,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test RentEpoch

    let rent_epoch = ctx
        .client()
        .get_account(user.encodable_pubkey())
        .await
        .unwrap()
        .unwrap()
        .rent_epoch;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(AccountInfoAssertion::RentEpoch {
                value: rent_epoch,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(AccountInfoAssertion::RentEpoch {
                value: rent_epoch - 1,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn verify_hash() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();
    let test_account = create_test_account(ctx, &user, false).await.unwrap();
    let test_account_data = ctx
        .client()
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .unwrap();

    let hash = keccak::hashv(&[&test_account_data.data]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: None,
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // (none, Some)

    let hash = keccak::hashv(&[&test_account_data.data[0..128]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: None,
                length: Some(128),
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // (Some, Some)

    let hash = keccak::hashv(&[&test_account_data.data[128..256]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(128),
                length: Some(128),
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // (Some, None)

    let hash = keccak::hashv(&[&test_account_data.data[128..]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(128),
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Fail (Some, Some)

    let hash = keccak::hashv(&[&test_account_data.data[128..]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(128),
                length: Some(64),
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Fail (Some, None)

    let hash = keccak::hashv(&[&test_account_data.data]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(128),
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Fail (None, Some)

    let hash = keccak::hashv(&[&test_account_data.data[16..128]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: None,
                length: Some(128),
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Fail (None, None)

    let hash = keccak::hashv(&[&test_account_data.data[1..]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: None,
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Out of bounds (None, Some)

    let hash = keccak::hashv(&[&test_account_data.data[128..]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(128),
                length: Some(1024),
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();

    // Out of bounds (Some, None)

    let hash = keccak::hashv(&[&test_account_data.data]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(1024),
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();

    // Out of bounds (Some, Some)

    let hash = keccak::hashv(&[&test_account_data.data[128..]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(1024),
                length: Some(1024),
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();

    // Empty account

    let hash = keccak::hashv(&[&[]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: None,
                length: None,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Empty account (Some, Some)

    let hash = keccak::hashv(&[&[]]).0;

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(AccountInfoAssertion::VerifyDatahash {
                expected_hash: hash,
                start: Some(0),
                length: Some(0),
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();
}
