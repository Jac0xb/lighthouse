use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::types::{
    Assertion, ComparableOperator, EquatableOperator, TokenAccountFieldAssertion,
};
use rust_sdk::LighthouseProgram;
use solana_program_test::tokio;
use solana_sdk::signer::Signer;
use spl_token::state::AccountState as TokenAccountState;

use crate::utils::context::TestContext;
use crate::utils::utils::process_transaction_assert_success;
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
    let mut tx_builder = program.create_assert_multi(
        &user,
        vec![
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Amount(
                0,
                ComparableOperator::GreaterThan,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Amount(
                101,
                ComparableOperator::LessThan,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Amount(
                100,
                ComparableOperator::LessThanOrEqual,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Amount(
                100,
                ComparableOperator::GreaterThanOrEqual,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Amount(
                100,
                ComparableOperator::Equal,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Amount(
                99,
                ComparableOperator::NotEqual,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Delegate(
                None,
                EquatableOperator::Equal,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::CloseAuthority(
                None,
                EquatableOperator::Equal,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Mint(
                mint.pubkey(),
                EquatableOperator::Equal,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Owner(
                user.pubkey(),
                EquatableOperator::Equal,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::State(
                TokenAccountState::Initialized as u8,
                ComparableOperator::Equal,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::State(
                TokenAccountState::Frozen as u8,
                ComparableOperator::NotEqual,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::DelegatedAmount(
                0,
                ComparableOperator::LessThanOrEqual,
            )),
            Assertion::TokenAccountField(TokenAccountFieldAssertion::IsNative(
                None,
                ComparableOperator::Equal,
            )),
        ],
        vec![token_account],
    );

    println!(
        "tx size: {}",
        tx_builder
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap()
    );

    process_transaction_assert_success(
        context,
        tx_builder
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();

    panic!("test");
}
