use crate::utils::context::TestContext;
use crate::utils::program::{
    create_cache_account, create_test_account, create_user, find_cache_account, find_test_account,
    Program,
};
use crate::utils::utils::to_transaction_error;
use crate::utils::{process_transaction_assert_failure, process_transaction_assert_success};
use lighthouse::error::ProgramError;
use lighthouse::structs::{Assertion, DataValue, Expression, Operator, WriteType};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountBalance(0, Operator::GreaterThan),
            // Assertion::AccountBalance(0, Operator::LessThan),
        ],
        vec![user.encodable_pubkey(), user.encodable_pubkey()],
        None,
        None,
    );

    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;
}

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn test_borsh_account_data() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    create_test_account(context, &user).await.unwrap();
    process_transaction_assert_success(
        context,
        program
            .create_assertion(
                &user,
                vec![
                    Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
                    Assertion::AccountData(9, Operator::Equal, DataValue::I8(-1)),
                    Assertion::AccountData(
                        10,
                        Operator::Equal,
                        DataValue::U16((u8::MAX as u16) + 1),
                    ),
                    Assertion::AccountData(
                        12,
                        Operator::Equal,
                        DataValue::I16((i8::MIN as i16) - 1),
                    ),
                    Assertion::AccountData(
                        14,
                        Operator::Equal,
                        DataValue::U32((u16::MAX as u32) + 1),
                    ),
                    Assertion::AccountData(
                        18,
                        Operator::Equal,
                        DataValue::I32((i16::MIN as i32) - 1),
                    ),
                    Assertion::AccountData(
                        22,
                        Operator::Equal,
                        DataValue::U64((u32::MAX as u64) + 1),
                    ),
                    Assertion::AccountData(
                        30,
                        Operator::Equal,
                        DataValue::I64((i32::MIN as i64) - 1),
                    ),
                    Assertion::AccountData(
                        38,
                        Operator::Equal,
                        DataValue::U128((u64::MAX as u128) + 1),
                    ),
                    Assertion::AccountData(
                        54,
                        Operator::Equal,
                        DataValue::I128((i64::MIN as i128) - 1),
                    ),
                    Assertion::AccountData(
                        70,
                        Operator::Equal,
                        DataValue::Bytes(vec![u8::MAX; 32]),
                    ),
                    Assertion::AccountData(102, Operator::Equal, DataValue::Bool(true)),
                    Assertion::AccountData(103, Operator::Equal, DataValue::Bool(false)),
                ],
                vec![find_test_account().0; 13],
                None,
                None,
            )
            .to_transaction(vec![])
            .await,
    )
    .await;
}

///
/// Test various logical expressions (false OR true), (true OR false), (true AND true).
///
#[tokio::test]
async fn test_logical_expression() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    create_test_account(context, &user).await.unwrap();

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
        None,
    );
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;

    // Test that the assertion passes when the logical expression is true.
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
        None,
    );
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;

    // Test that the assertion fails when the logical expression is false.
    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(5)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16(30)),
        ],
        vec![find_test_account().0; 4],
        Some(vec![Expression::Or(vec![1, 2])]),
        None,
    );
    process_transaction_assert_failure(
        context,
        tx_builder.to_transaction(vec![]).await,
        to_transaction_error(0, ProgramError::AssertionFailed),
        Some(&["1 == 5".to_string(), "256 == 30".to_string()]),
    )
    .await;
}

#[tokio::test]
async fn test_account_balance() {
    // let context = &mut TestContext::new().await.unwrap();

    // let mut program = Program::new(context.client());
    // create_test_account(context, user).await.unwrap();
}

#[tokio::test]
async fn test_write() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    // Create test account
    let _ = create_test_account(context, &user).await;
    let _ = create_cache_account(context, &user, 256).await;

    let cache_account = find_cache_account(user.encodable_pubkey(), 0).0;

    {
        // Test writing account data to cache.
        process_transaction_assert_success(
            context,
            program
                .write_v1(
                    &user,
                    find_test_account().0,
                    0,
                    lighthouse::structs::WriteTypeParameter::WriteU8(
                        0,
                        WriteType::AccountData(8, Some(128), None),
                    ),
                )
                .to_transaction(vec![])
                .await,
        )
        .await;

        // Assert that data was properly written to cache.
        let tx = program
            .create_assertion(
                &user,
                vec![
                    Assertion::Memory(0, Operator::Equal, DataValue::U8(1)),
                    Assertion::Memory(0, Operator::GreaterThan, DataValue::U8(0)),
                    Assertion::Memory(0, Operator::LessThan, DataValue::U8(2)),
                    Assertion::Memory(0, Operator::GreaterThanOrEqual, DataValue::U8(1)),
                    Assertion::Memory(0, Operator::LessThanOrEqual, DataValue::U8(1)),
                    Assertion::Memory(1, Operator::Equal, DataValue::I8(-1)),
                    Assertion::Memory(1, Operator::GreaterThan, DataValue::I8(-2)),
                    Assertion::Memory(1, Operator::LessThan, DataValue::I8(0)),
                    Assertion::Memory(1, Operator::GreaterThanOrEqual, DataValue::I8(-1)),
                    Assertion::Memory(1, Operator::LessThanOrEqual, DataValue::I8(-1)),
                    Assertion::Memory(2, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
                    Assertion::Memory(4, Operator::Equal, DataValue::I16((i8::MIN as i16) - 1)),
                    Assertion::Memory(6, Operator::Equal, DataValue::U32((u16::MAX as u32) + 1)),
                    Assertion::Memory(10, Operator::Equal, DataValue::I32((i16::MIN as i32) - 1)),
                    Assertion::Memory(14, Operator::Equal, DataValue::U64((u32::MAX as u64) + 1)),
                    Assertion::Memory(22, Operator::Equal, DataValue::I64((i32::MIN as i64) - 1)),
                    Assertion::Memory(30, Operator::Equal, DataValue::U128((u64::MAX as u128) + 1)),
                    Assertion::Memory(46, Operator::Equal, DataValue::I128((i64::MIN as i128) - 1)),
                ],
                vec![],
                None,
                Some(cache_account),
            )
            .to_transaction(vec![])
            .await;

        process_transaction_assert_success(context, tx).await;
    }
}
