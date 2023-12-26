use lighthouse::{
    error::ProgramError,
    structs::{Assertion, DataValue, Expression, Operator},
};
use solana_program_test::tokio;

use crate::utils::{
    context::TestContext,
    process_transaction_assert_failure, process_transaction_assert_success,
    program::{create_test_account, create_user, find_test_account, Program},
    utils::to_transaction_error,
};
///
/// Test various logical expressions (false OR true), (true OR false), (true AND true).
///
#[tokio::test]
async fn test_logical_expression() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    create_test_account(context, &user).await.unwrap();

    println!("Test that the assertion passes when the logical expression is true.");
    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(5)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16(30)),
        ],
        vec![find_test_account().0; 4],
        Some(vec![
            Expression::Or(vec![0, 1]),
            Expression::Or(vec![3, 2]),
            Expression::And(vec![0, 2]),
        ]),
    );
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;

    // Test that the assertion fails when the logical expression is false.
    println!("Test that the assertion fails when the logical expression is false.");
    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(5)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16(30)),
        ],
        vec![find_test_account().0; 4],
        Some(vec![Expression::Or(vec![1, 3])]),
    );
    process_transaction_assert_failure(
        context,
        tx_builder.to_transaction(vec![]).await,
        to_transaction_error(0, ProgramError::AssertionFailed),
        Some(&["1 == 5".to_string(), "256 == 30".to_string()]),
    )
    .await;

    // Test that the assertion fails when the logical expression is false.
    println!("Test that the assertion fails when the logical expression is false.");
    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            Assertion::AccountData(8, Operator::GreaterThan, DataValue::U8(0)),
            Assertion::AccountData(10, Operator::LessThan, DataValue::U16(u8::MAX as u16)),
        ],
        vec![find_test_account().0; 4],
        Some(vec![Expression::And(vec![0, 1]), Expression::Or(vec![2])]),
    );
    process_transaction_assert_failure(
        context,
        tx_builder.to_transaction(vec![]).await,
        to_transaction_error(0, ProgramError::AssertionFailed),
        Some(&["1 == 1".to_string(), "256 < 255".to_string()]),
    )
    .await;
}
