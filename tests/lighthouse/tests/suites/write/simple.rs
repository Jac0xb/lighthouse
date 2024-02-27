use std::mem;

use crate::utils::utils::process_transaction_assert_success;
use crate::utils::{context::TestContext, create_memory_account, create_test_account, create_user};
use lighthouse_client::instructions::{AssertAccountDataBuilder, WriteBuilder};
use lighthouse_client::types::{
    DataValueAssertion, EquatableOperator, IntegerOperator, WriteType, WriteTypeParameter,
};
use lighthouse_sdk::find_memory_account;
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn test_write() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    // Create test account
    let test_account = create_test_account(context, &user, false).await.unwrap();
    let test_account_data = context
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .data;

    let account = context
        .client()
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .unwrap();
    let account_data_length = account.data.len() as u64;

    let _ = create_memory_account(context, &user, 0, account_data_length - 8).await;

    let (memory_account, memory_account_bump) = find_memory_account(user.encodable_pubkey(), 0);

    let tx = Transaction::new_signed_with_payer(
        &[WriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(test_account.encodable_pubkey())
            .memory_account(memory_account)
            .lighthouse_program(lighthouse_client::programs::LIGHTHOUSE_ID)
            .memory_index(0)
            .memory_account_bump(memory_account_bump)
            .write_type(WriteTypeParameter::WriteU16 {
                offset: 0,
                write_type: WriteType::AccountData {
                    offset: 8,
                    data_length: Some((account_data_length - 8) as u16),
                },
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    // Test writing account data to memory.
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let memory_account_data = context.get_account(memory_account).await.unwrap().data;

    assert_eq!(test_account_data[8..], memory_account_data[..]);

    // Assert that data was properly written to memory.
    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U8 {
                    expected_value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(0)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I8 {
                    expected_value: -1,
                    operator: IntegerOperator::Equal,
                })
                .offset(1)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U16 {
                    expected_value: (u8::MAX as u16) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(2)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I16 {
                    expected_value: (i8::MIN as i16) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(4)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U32 {
                    expected_value: (u16::MAX as u32) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(6)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I32 {
                    expected_value: (i16::MIN as i32) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(10)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U64 {
                    expected_value: (u32::MAX as u64) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(14)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I64 {
                    expected_value: (i32::MIN as i64) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(22)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U128 {
                    expected_value: (u64::MAX as u128) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(30)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I128 {
                    expected_value: (i64::MIN as i128) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(46)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Bytes {
                    expected_value: vec![u8::MAX; 32],
                    operator: EquatableOperator::Equal,
                })
                .offset(62)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Pubkey {
                    expected_value: user.encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                })
                .offset(94)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Bool {
                    expected_value: false,
                    operator: EquatableOperator::Equal,
                })
                .offset(95)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .offset(146)
                .assertion(DataValueAssertion::U8 {
                    expected_value: u8::MAX,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx.clone())
        .await
        .unwrap();
}
