use crate::utils::context::TestContext;
use crate::utils::tx_builder::TxBuilder;
use crate::utils::{create_test_account, create_user_with_balance};
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use lighthouse_sdk::errors::LighthouseError;
use lighthouse_sdk::instructions::AssertAccountDataBuilder;
use lighthouse_sdk::types::{DataValueAssertion, EquatableOperator, IntegerOperator};
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn simple() {
    // print all env vars
    for (key, value) in std::env::vars() {
        println!("{}:  {}", key, value);
    }

    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let test_account = create_test_account(context, &user, false).await.unwrap();
    let log_level = lighthouse_sdk::types::LogLevel::Silent;

    let mut tx = TxBuilder {
        ixs: vec![
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(8u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::Equal,
                })
                .offset(9u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U16 {
                    value: (u8::MAX as u16) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(10u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::I16 {
                    value: (i8::MIN as i16) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(12u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U32 {
                    value: (u16::MAX as u32) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(14u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::I32 {
                    value: (i16::MIN as i32) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(18u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U64 {
                    value: (u32::MAX as u64) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(22u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::I64 {
                    value: (i32::MIN as i64) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(30u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U128 {
                    value: (u64::MAX as u128) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(38u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::I128 {
                    value: (i64::MIN as i128) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(54u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::Bytes {
                    value: vec![u8::MAX; 32].into(),
                    operator: EquatableOperator::Equal,
                })
                .offset(70u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::Bool {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .offset(102u8.into())
                .instruction(),
            // False represented as 0
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(103u8.into())
                .instruction(),
            // Some in Option<u8>
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(104u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U8 {
                    value: u8::MAX,
                    operator: IntegerOperator::Equal,
                })
                .offset(105u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(106u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::Bytes {
                    value: vec![1, 255, 255].into(),
                    operator: EquatableOperator::Equal,
                })
                .offset(107u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::Bytes {
                    value: vec![0].into(),
                    operator: EquatableOperator::Equal,
                })
                .offset(110u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::Pubkey {
                    value: user.encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                })
                .offset(111u8.into())
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(log_level.clone())
                .assertion(DataValueAssertion::Bytes {
                    value: [32, 0, 0, 0]
                        .iter()
                        .cloned()
                        .chain(vec![255; 32])
                        .collect::<Vec<u8>>()
                        .into(),
                    operator: EquatableOperator::Equal,
                })
                .offset(143u8.into())
                .instruction(),
        ],
        look_up_tables: None,
    };

    let blockhash = context.get_blockhash().await;

    let result = process_transaction_assert_success(
        context,
        tx.to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
            .unwrap(),
    )
    .await
    .unwrap();

    assert!(
        result.metadata.as_ref().unwrap().compute_units_consumed < 16150,
        "Exceeded expected CU 16150 was {:?}",
        result.metadata.unwrap().compute_units_consumed
    );
}

#[tokio::test]
async fn empty_account_fail() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let test_account = create_test_account(context, &user, false).await.unwrap();

    let mut tx = TxBuilder {
        ixs: vec![AssertAccountDataBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
            .assertion(DataValueAssertion::U8 {
                value: 1,
                operator: IntegerOperator::Equal,
            })
            .offset(1234u16.into())
            .instruction()],
        look_up_tables: None,
    };

    let blockhash = context.get_blockhash().await;

    process_transaction_assert_failure(
        context,
        tx.to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
            .unwrap(),
        to_transaction_error(0, LighthouseError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();

    let mut tx = TxBuilder {
        ixs: vec![AssertAccountDataBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
            .assertion(DataValueAssertion::U128 {
                value: 1,
                operator: IntegerOperator::Equal,
            })
            .offset(0u8.into())
            .instruction()],
        look_up_tables: None,
    };

    process_transaction_assert_failure(
        context,
        tx.to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
            .unwrap(),
        to_transaction_error(0, LighthouseError::AccountNotInitialized),
        None,
    )
    .await
    .unwrap();

    let mut tx = TxBuilder {
        ixs: vec![AssertAccountDataBuilder::new()
            .target_account(Keypair::new().encodable_pubkey())
            .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
            .assertion(DataValueAssertion::U128 {
                value: 1,
                operator: IntegerOperator::Equal,
            })
            .offset(0u8.into())
            .instruction()],
        look_up_tables: None,
    };

    process_transaction_assert_failure(
        context,
        tx.to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
            .unwrap(),
        to_transaction_error(0, LighthouseError::AccountNotInitialized),
        None,
    )
    .await
    .unwrap();
}
