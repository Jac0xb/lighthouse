use crate::utils::context::TestContext;
use crate::utils::{create_mint, create_user, CreateMintParameters};
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use lighthouse_sdk::errors::LighthouseError;
use lighthouse_sdk::instructions::AssertMintAccountBuilder;
use lighthouse_sdk::types::{EquatableOperator, IntegerOperator, LogLevel, MintAccountAssertion};
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn simple() {
    let context = &mut TestContext::new().await.unwrap();
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

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::MintAuthority {
                    value: Some(user.pubkey()),
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::Supply {
                    value: 69_000,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::Decimals {
                    value: 9,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::IsInitialized {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::FreezeAuthority {
                    value: None,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Mint with freeze authority

    let freezer = Keypair::new();
    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: Some(freezer.encodable_pubkey()),
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::FreezeAuthority {
                    value: Some(freezer.pubkey()),
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::FreezeAuthority {
                    value: Some(user.encodable_pubkey()),
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::FreezeAuthority {
                    value: None,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Mint authority test

    let minter = Keypair::new();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(minter.pubkey())),
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

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::MintAuthority {
                    value: Some(minter.pubkey()),
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::MintAuthority {
                    value: Some(user.pubkey()),
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::MintAuthority {
                    value: None,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Supply test

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: Some(freezer.encodable_pubkey()),
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::Supply {
                    value: 69_000,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::Supply {
                    value: 69_001,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Fail supply test

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: Some(freezer.encodable_pubkey()),
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertMintAccountBuilder::new()
            .target_account(mint.encodable_pubkey())
            .log_level(LogLevel::Silent)
            .assertion(MintAccountAssertion::Supply {
                value: 69_001,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Decimals fail

    let tx = Transaction::new_signed_with_payer(
        &[AssertMintAccountBuilder::new()
            .target_account(mint.encodable_pubkey())
            .log_level(LogLevel::Silent)
            .assertion(MintAccountAssertion::Decimals {
                value: 8,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Freeze authority fail None

    let tx = Transaction::new_signed_with_payer(
        &[AssertMintAccountBuilder::new()
            .target_account(mint.encodable_pubkey())
            .log_level(LogLevel::Silent)
            .assertion(MintAccountAssertion::FreezeAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Freeze authority fail Some

    let tx = Transaction::new_signed_with_payer(
        &[AssertMintAccountBuilder::new()
            .target_account(mint.encodable_pubkey())
            .log_level(LogLevel::Silent)
            .assertion(MintAccountAssertion::FreezeAuthority {
                value: Some(Keypair::new().encodable_pubkey()),
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn account_not_owned_by_token_program() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertMintAccountBuilder::new()
            .target_account(Keypair::new().encodable_pubkey())
            .log_level(LogLevel::Silent)
            .assertion(MintAccountAssertion::IsInitialized {
                value: true,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AccountOwnerMismatch),
        None,
    )
    .await
    .unwrap();
}

// #[tokio::test]
