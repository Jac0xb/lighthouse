use crate::utils::context::TestContext;
use crate::utils::{create_mint, create_user, CreateMintParameters};
use crate::utils::{process_transaction_assert_failure, process_transaction_assert_success};
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse_client::instructions::AssertTokenAccountMultiBuilder;
use lighthouse_client::types::{EquatableOperator, IntegerOperator, TokenAccountAssertion};
use solana_program_test::tokio;
use solana_sdk::instruction::InstructionError;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::{Transaction, TransactionError};
use spl_token::state::AccountState as TokenAccountState;

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
            .log_level(lighthouse_client::types::LogLevel::Silent)
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
                    operator: IntegerOperator::Equal,
                },
                TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::State {
                    value: TokenAccountState::Frozen as u8,
                    operator: IntegerOperator::NotEqual,
                },
                TokenAccountAssertion::IsNative {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::DelegatedAmount {
                    value: 0,
                    operator: IntegerOperator::LessThanOrEqual,
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

#[tokio::test]
async fn prod_test() {
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
            .log_level(lighthouse_client::types::LogLevel::Silent)
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
                    value: 90,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
                TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
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

#[tokio::test]
async fn multi_errors() {
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
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertions(vec![
                // Fail
                TokenAccountAssertion::Mint {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::Owner {
                    value: user.pubkey(),
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::Amount {
                    value: 90,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
                TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
            ])
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        TransactionError::InstructionError(0, InstructionError::Custom(0x1900)),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertTokenAccountMultiBuilder::new()
            .target_account(token_account)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertions(vec![
                TokenAccountAssertion::Mint {
                    value: mint.pubkey(),
                    operator: EquatableOperator::Equal,
                },
                // Fail
                TokenAccountAssertion::Owner {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::Amount {
                    value: 90,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
                TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
            ])
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        TransactionError::InstructionError(0, InstructionError::Custom(0x1901)),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertTokenAccountMultiBuilder::new()
            .target_account(token_account)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertions(vec![
                TokenAccountAssertion::Mint {
                    value: mint.pubkey(),
                    operator: EquatableOperator::Equal,
                },
                TokenAccountAssertion::Owner {
                    value: user.pubkey(),
                    operator: EquatableOperator::Equal,
                },
                // Fail
                TokenAccountAssertion::Amount {
                    value: 100,
                    operator: IntegerOperator::GreaterThan,
                },
                TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
            ])
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        TransactionError::InstructionError(0, InstructionError::Custom(0x1902)),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertTokenAccountMultiBuilder::new()
            .target_account(token_account)
            .log_level(lighthouse_client::types::LogLevel::Silent)
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
                    value: 90,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
                // Fail
                TokenAccountAssertion::Delegate {
                    value: None,
                    operator: EquatableOperator::NotEqual,
                },
            ])
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        TransactionError::InstructionError(0, InstructionError::Custom(0x1903)),
        None,
    )
    .await
    .unwrap();
}
