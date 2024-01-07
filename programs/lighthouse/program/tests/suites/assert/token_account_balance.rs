use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::structs::{Assertion, Operator};
use solana_program_test::tokio;
use solana_sdk::signer::Signer;

use crate::utils::process_transaction_assert_success;
use crate::utils::program::{create_mint, mint_to};
use crate::utils::{
    context::TestContext,
    program::{create_user, Program},
};

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(context, &user).await.unwrap();
    process_transaction_assert_success(context, Ok(tx)).await;

    let tx = mint_to(context, &mint.pubkey(), &user, &user.pubkey(), 100)
        .await
        .unwrap();
    process_transaction_assert_success(context, Ok(tx)).await;

    let token_account = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let mut tx_builder = program.create_assert_multi(
        &user,
        vec![
            Assertion::LegacyTokenAccountField(
                lighthouse::structs::LegacyTokenAccountDataField::Amount(0),
                Operator::GreaterThan,
            ),
            Assertion::LegacyTokenAccountField(
                lighthouse::structs::LegacyTokenAccountDataField::Amount(101),
                Operator::LessThan,
            ),
            Assertion::LegacyTokenAccountField(
                lighthouse::structs::LegacyTokenAccountDataField::Amount(100),
                Operator::LessThanOrEqual,
            ),
            Assertion::LegacyTokenAccountField(
                lighthouse::structs::LegacyTokenAccountDataField::Amount(100),
                Operator::GreaterThanOrEqual,
            ),
            Assertion::LegacyTokenAccountField(
                lighthouse::structs::LegacyTokenAccountDataField::Amount(100),
                Operator::Equal,
            ),
            Assertion::LegacyTokenAccountField(
                lighthouse::structs::LegacyTokenAccountDataField::Amount(99),
                Operator::NotEqual,
            ),
        ],
        vec![token_account],
    );

    process_transaction_assert_success(context, tx_builder.to_transaction().await).await;
}
