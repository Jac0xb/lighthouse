use lighthouse::types::{
    AccountInfoFieldAssertion, Assertion, AssertionConfigV1, ComparableOperator,
};
use rust_sdk::LighthouseProgram;
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

use crate::utils::context::TestContext;
use crate::utils::create_user;
use crate::utils::utils::process_transaction_assert_success;

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let mut tx_builder = program.create_assert(
        &user,
        user.encodable_pubkey(),
        Assertion::AccountInfoField(AccountInfoFieldAssertion::Lamports(
            0,
            ComparableOperator::GreaterThan,
        )),
        Some(AssertionConfigV1 { verbose: true }),
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
