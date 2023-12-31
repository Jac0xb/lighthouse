use lighthouse::structs::{AccountInfoDataField, Assertion, Operator};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

use crate::utils::process_transaction_assert_success;
use crate::utils::{
    context::TestContext,
    program::{create_user, Program},
};

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    let mut tx_builder = program.create_assert(
        &user,
        user.encodable_pubkey(),
        Assertion::AccountInfoField(AccountInfoDataField::Lamports(0), Operator::GreaterThan),
    );

    process_transaction_assert_success(context, tx_builder.to_transaction().await).await;
}
