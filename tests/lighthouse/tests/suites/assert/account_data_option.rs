use crate::utils::context::TestContext;
use crate::utils::utils::process_transaction_assert_success;
use crate::utils::{create_test_account, create_user};
use lighthouse::types::{Assertion, DataValue, Operator};
use rust_sdk::LighthouseProgram;
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

// Define a macro that adds an associated function to the struct to get field offsets

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn test_borsh_account_data() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let test_account = create_test_account(context, &user, false).await.unwrap();

    process_transaction_assert_success(
        context,
        program
            .create_assert_multi(
                &user,
                vec![
                    Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
                    Assertion::AccountData(9, Operator::Equal, DataValue::I8(-1)),
                ],
                vec![test_account.encodable_pubkey()],
            )
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();
}
