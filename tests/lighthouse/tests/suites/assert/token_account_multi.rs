use anchor_spl::associated_token::get_associated_token_address;
use lighthouse_client::instructions::AssertTokenAccountMultiBuilder;
use lighthouse_client::types::{ComparableOperator, EquatableOperator, TokenAccountAssertion};
use solana_program_test::tokio;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_token::state::AccountState as TokenAccountState;

use crate::utils::context::TestContext;
use crate::utils::process_transaction_assert_success;
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
            .lighthouse_program(lighthouse_client::ID)
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

    panic!("Test not implemented");
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
            .lighthouse_program(lighthouse_client::ID)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertions(vec![
                TokenAccountAssertion::Mint {
                    value: mint.pubkey(),
                    operator: EquatableOperator::Equal,
                },
                // TokenAccountAssertion::Owner {
                //     value: user.pubkey(),
                //     operator: EquatableOperator::Equal,
                // },
                // TokenAccountAssertion::Amount {
                //     value: 90,
                //     operator: ComparableOperator::GreaterThanOrEqual,
                // },
                // TokenAccountAssertion::Delegate {
                //     value: None,
                //     operator: EquatableOperator::Equal,
                // },
            ])
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // panic!("This test is not implemented yet");
}
