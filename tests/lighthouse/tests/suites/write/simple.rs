use crate::utils::{context::TestContext, create_test_account, create_user};
use crate::utils::{find_memory_account, process_transaction_assert_success};
use lighthouse_client::instructions::{AssertAccountDataBuilder, MemoryWriteBuilder};
use lighthouse_client::types::{
    ByteSliceOperator, DataValue, DataValueAssertion, EquatableOperator, IntegerOperator, WriteType,
};
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn test_write() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    // Create test account
    let test_account = create_test_account(context, &user, false).await.unwrap();
    let test_account_data = &mut context
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

    let (memory_account, memory_account_bump) = find_memory_account(user.encodable_pubkey(), 0);

    let tx = Transaction::new_signed_with_payer(
        &[MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(test_account.encodable_pubkey())
            .memory_account(memory_account)
            .program_id(lighthouse_client::ID)
            .memory_index(0)
            .memory_account_bump(memory_account_bump)
            .memory_offset(0)
            .system_program(system_program::id())
            .write_type(WriteType::AccountData {
                offset: 8,
                data_length: (account_data_length - 8) as u16,
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
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(0)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::Equal,
                })
                .offset(1)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U16 {
                    value: (u8::MAX as u16) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(2)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I16 {
                    value: (i8::MIN as i16) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(4)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U32 {
                    value: (u16::MAX as u32) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(6)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I32 {
                    value: (i16::MIN as i32) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(10)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U64 {
                    value: (u32::MAX as u64) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(14)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I64 {
                    value: (i32::MIN as i64) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(22)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U128 {
                    value: (u64::MAX as u128) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(30)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I128 {
                    value: (i64::MIN as i128) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(46)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: vec![u8::MAX; 32],
                    operator: ByteSliceOperator::Equal,
                })
                .offset(62)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bool {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .offset(94)
                .instruction(),
            // False represented as 0
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(95)
                .instruction(),
            // Some in Option<u8>
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(96)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: u8::MAX,
                    operator: IntegerOperator::Equal,
                })
                .offset(97)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(98)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: [1, 255, 255].to_vec(),
                    operator: ByteSliceOperator::Equal,
                })
                .offset(99)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: [0].to_vec(),
                    operator: ByteSliceOperator::Equal,
                })
                .offset(102)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Pubkey {
                    value: user.encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                })
                .offset(103)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: [32, 0, 0, 0]
                        .iter()
                        .cloned()
                        .chain(vec![255; 32])
                        .collect::<Vec<u8>>(),
                    operator: ByteSliceOperator::Equal,
                })
                .offset(135)
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

#[tokio::test]
async fn test_write_u64() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (memory_account, memory_account_bump) = find_memory_account(user.encodable_pubkey(), 0);

    // Assert that data was properly written to memory.
    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(lighthouse_client::ID)
                .memory_account(memory_account)
                .program_id(lighthouse_client::ID)
                .memory_account_bump(memory_account_bump)
                .memory_offset(0)
                .memory_index(0)
                .system_program(system_program::id())
                .write_type(WriteType::DataValue(DataValue::U64(u64::MAX / 2)))
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U64 {
                    value: u64::MAX / 2,
                    operator: IntegerOperator::Equal,
                })
                .offset(0)
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx.clone())
        .await
        .unwrap();

    let random_keypair = Keypair::new();

    // Assert that data was properly written to memory.
    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(lighthouse_client::ID)
                .program_id(lighthouse_client::ID)
                .memory_account(memory_account)
                .memory_account_bump(memory_account_bump)
                .memory_offset(512)
                .memory_index(0)
                .system_program(system_program::id())
                .write_type(WriteType::DataValue(DataValue::U128(u128::MAX)))
                .instruction(),
            MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(lighthouse_client::ID)
                .program_id(lighthouse_client::ID)
                .memory_account(memory_account)
                .memory_account_bump(memory_account_bump)
                .memory_offset(128)
                .memory_index(0)
                .system_program(system_program::id())
                .write_type(WriteType::DataValue(DataValue::Pubkey(
                    random_keypair.encodable_pubkey(),
                )))
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U64 {
                    value: u64::MAX / 2,
                    operator: IntegerOperator::Equal,
                })
                .offset(0)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Pubkey {
                    value: random_keypair.encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                })
                .offset(128)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory_account)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U128 {
                    value: u128::MAX,
                    operator: IntegerOperator::Equal,
                })
                .offset(512)
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
