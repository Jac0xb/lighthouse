use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::types::{Assertion, Operator, TokenAccountDataField};
use rust_sdk::LighthouseProgram;
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

use crate::utils::context::TestContext;
use crate::utils::{create_mint, create_user, mint_to, process_transaction_assert_success};

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(context, &user).await.unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = mint_to(context, &mint.pubkey(), &user, &user.pubkey(), 100)
        .await
        .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let token_account = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let mut tx_builder = program.create_assert_multi(
        &user,
        vec![
            Assertion::TokenAccountField(TokenAccountDataField::Amount(0), Operator::GreaterThan),
            Assertion::TokenAccountField(TokenAccountDataField::Amount(101), Operator::LessThan),
            Assertion::TokenAccountField(
                TokenAccountDataField::Amount(100),
                Operator::LessThanOrEqual,
            ),
            Assertion::TokenAccountField(
                TokenAccountDataField::Amount(100),
                Operator::GreaterThanOrEqual,
            ),
            Assertion::TokenAccountField(TokenAccountDataField::Amount(100), Operator::Equal),
            Assertion::TokenAccountField(TokenAccountDataField::Amount(99), Operator::NotEqual),
        ],
        vec![token_account],
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
