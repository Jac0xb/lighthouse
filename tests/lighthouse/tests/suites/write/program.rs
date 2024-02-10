use lighthouse::structs::{Assertion, DataValue, Operator, WriteType};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

use crate::utils::{
    context::TestContext,
    process_transaction_assert_success,
    program::{
        create_memory_account, create_test_account, create_user, find_memory_account,
        find_test_account, Program,
    },
};

#[tokio::test]
async fn test_write_program() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    // Create test account
    let _ = create_test_account(context, &user).await;
    let _ = create_memory_account(context, &user, 256).await;

    let memory_account = find_memory_account(user.encodable_pubkey(), 0).0;

    {
        // Test writing account data to memory.
        process_transaction_assert_success(
            context,
            program
                .write_v1(
                    &user,
                    lighthouse::ID,
                    0,
                    lighthouse::structs::WriteTypeParameter::WriteU8(0, WriteType::Program),
                )
                .to_transaction(vec![])
                .await,
        )
        .await;

        // Assert that data was properly written to memory.
        let tx = program
            .create_assertion(
                &user,
                vec![Assertion::Memory(0, Operator::Equal, DataValue::U8(1))],
                vec![],
                None,
                Some(memory_account),
            )
            .to_transaction(vec![])
            .await;

        process_transaction_assert_success(context, tx).await;
    }
}
