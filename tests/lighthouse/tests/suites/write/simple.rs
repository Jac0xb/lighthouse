use crate::utils::{context::TestContext, create_test_account, create_user};
use crate::utils::{
    create_and_transfer_token_account_ix, create_mint, process_transaction_assert_failure,
    process_transaction_assert_success, to_transaction_error, CreateMintParameters,
};
use anchor_lang::*;
use borsh::BorshSerialize;
use lighthouse_sdk::errors::LighthouseError;
use lighthouse_sdk::instructions::{
    AssertAccountDataBuilder, AssertAccountDeltaBuilder, MemoryCloseBuilder, MemoryWriteBuilder,
};
use lighthouse_sdk::types::{
    AccountDeltaAssertion, AccountInfoField, ByteSliceOperator, DataValue, DataValueAssertion,
    DataValueDeltaAssertion, EquatableOperator, IntegerOperator, LogLevel, WriteType,
};
use lighthouse_sdk::{find_memory_pda, find_memory_pda_bump_iterate};
use solana_program_test::tokio;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;
use solana_sdk::{bpf_loader, system_program};
use spl_associated_token_account::get_associated_token_address;
use std::u8::MAX;

#[tokio::test]
async fn write_account_data() {
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

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

    let tx = Transaction::new_signed_with_payer(
        &[MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(test_account.encodable_pubkey())
            .memory(memory)
            .program_id(lighthouse_sdk::ID)
            .memory_id(0)
            .memory_bump(memory_bump)
            .write_offset(0)
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

    let memory_data = context.get_account(memory).await.unwrap().data;

    assert_eq!(test_account_data[8..], memory_data[..]);

    // Assert that data was properly written to memory.
    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(0)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::Equal,
                })
                .offset(1)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U16 {
                    value: (u8::MAX as u16) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(2)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I16 {
                    value: (i8::MIN as i16) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(4)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U32 {
                    value: (u16::MAX as u32) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(6)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I32 {
                    value: (i16::MIN as i32) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(10)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U64 {
                    value: (u32::MAX as u64) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(14)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I64 {
                    value: (i32::MIN as i64) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(22)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U128 {
                    value: (u64::MAX as u128) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(30)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::I128 {
                    value: (i64::MIN as i128) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(46)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: vec![u8::MAX; 32],
                    operator: ByteSliceOperator::Equal,
                })
                .offset(62)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bool {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .offset(94)
                .instruction(),
            // False represented as 0
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(95)
                .instruction(),
            // Some in Option<u8>
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(96)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: u8::MAX,
                    operator: IntegerOperator::Equal,
                })
                .offset(97)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(98)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: [1, 255, 255].to_vec(),
                    operator: ByteSliceOperator::Equal,
                })
                .offset(99)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: [0].to_vec(),
                    operator: ByteSliceOperator::Equal,
                })
                .offset(102)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Pubkey {
                    value: user.encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                })
                .offset(103)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
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
async fn write_account_type() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();
    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

    let build_memory = |offset: u16, write_type: WriteType| {
        MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(user.encodable_pubkey())
            .memory(memory)
            .program_id(lighthouse_sdk::ID)
            .memory_id(0)
            .memory_bump(memory_bump)
            .write_offset(offset)
            .system_program(system_program::id())
            .write_type(write_type)
            .instruction()
    };

    let expected_blob = &mut vec![MAX; 94];
    expected_blob.extend(user.encodable_pubkey().try_to_vec().unwrap().to_vec());
    expected_blob.push(1);
    expected_blob.push(0);

    let tx = Transaction::new_signed_with_payer(
        &[
            build_memory(0, WriteType::DataValue(DataValue::U8(MAX))),
            build_memory(1, WriteType::DataValue(DataValue::U8(MAX))),
            build_memory(2, WriteType::DataValue(DataValue::U16(u16::MAX))),
            build_memory(4, WriteType::DataValue(DataValue::I16(-1))),
            build_memory(6, WriteType::DataValue(DataValue::U32(u32::MAX))),
            build_memory(10, WriteType::DataValue(DataValue::I32(-1))),
            build_memory(14, WriteType::DataValue(DataValue::U64(u64::MAX))),
            build_memory(22, WriteType::DataValue(DataValue::I64(-1))),
            build_memory(30, WriteType::DataValue(DataValue::U128(u128::MAX))),
            build_memory(46, WriteType::DataValue(DataValue::I128(-1))),
            build_memory(62, WriteType::DataValue(DataValue::Bytes(vec![MAX; 32]))),
            build_memory(
                94,
                WriteType::DataValue(DataValue::Pubkey(user.encodable_pubkey())),
            ),
            build_memory(126, WriteType::DataValue(DataValue::Bool(true))),
            build_memory(127, WriteType::DataValue(DataValue::Bool(false))),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: expected_blob.clone(),
                    operator: ByteSliceOperator::Equal,
                })
                .offset(0)
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    // Test writing account data to memory.
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let memory_data = context.get_account(memory).await.unwrap().data;

    assert_eq!(memory_data.len(), 128);
    assert_eq!(memory_data.as_slice(), expected_blob.as_slice());
}

#[tokio::test]
async fn write_reallocation() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

    // Assert that data was properly written to memory.
    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(lighthouse_sdk::ID)
                .memory(memory)
                .program_id(lighthouse_sdk::ID)
                .memory_bump(memory_bump)
                .write_offset(0)
                .memory_id(0)
                .system_program(system_program::id())
                .write_type(WriteType::DataValue(DataValue::U64(u64::MAX / 2)))
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
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
                .source_account(lighthouse_sdk::ID)
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_bump(memory_bump)
                .write_offset(512)
                .memory_id(0)
                .system_program(system_program::id())
                .write_type(WriteType::DataValue(DataValue::U128(u128::MAX)))
                .instruction(),
            MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(lighthouse_sdk::ID)
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_bump(memory_bump)
                .write_offset(128)
                .memory_id(0)
                .system_program(system_program::id())
                .write_type(WriteType::DataValue(DataValue::Pubkey(
                    random_keypair.encodable_pubkey(),
                )))
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::U64 {
                    value: u64::MAX / 2,
                    operator: IntegerOperator::Equal,
                })
                .offset(0)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Pubkey {
                    value: random_keypair.encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                })
                .offset(128)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
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

//
//  Tests the use where one craetes a memory account, writes token information to it
//  and then transfers tokens and asserts afterwards that the delta is above threshold.
//
#[tokio::test]
async fn token_transfer() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();
    let dest = Keypair::new();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: None,
            freeze_authority: None,
            mint_to: Some((user.encodable_pubkey(), 100)),
            decimals: 9,
        },
    )
    .await
    .unwrap();
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);
    let token_account =
        get_associated_token_address(&user.encodable_pubkey(), &mint.encodable_pubkey());

    let mut ixs = vec![MemoryWriteBuilder::new()
        .payer(user.encodable_pubkey())
        .source_account(token_account)
        .program_id(lighthouse_sdk::ID)
        .memory(memory)
        .memory_id(0)
        .write_offset(0)
        .memory_bump(memory_bump)
        .write_type(WriteType::AccountData {
            offset: 0,
            data_length: 72,
        })
        .instruction()];
    ixs.extend(
        create_and_transfer_token_account_ix(
            context,
            &user.encodable_pubkey(),
            &mint.encodable_pubkey(),
            &dest.encodable_pubkey(),
            69,
        )
        .await
        .unwrap(),
    );
    ixs.extend(vec![
        AssertAccountDeltaBuilder::new()
            .account_a(memory)
            .account_b(token_account)
            .assertion(AccountDeltaAssertion::Data {
                a_offset: 0,
                b_offset: 0,
                assertion: DataValueDeltaAssertion::Bytes {
                    operator: ByteSliceOperator::Equal,
                    length: 64,
                },
            })
            .log_level(LogLevel::Silent)
            .instruction(),
        AssertAccountDeltaBuilder::new()
            .account_a(memory)
            .account_b(token_account)
            .assertion(AccountDeltaAssertion::Data {
                a_offset: 64,
                b_offset: 64,
                assertion: DataValueDeltaAssertion::U64 {
                    value: -70,
                    operator: IntegerOperator::GreaterThan,
                },
            })
            .log_level(LogLevel::PlaintextMessage)
            .instruction(),
        MemoryCloseBuilder::new()
            .payer(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(0)
            .instruction(),
    ]);

    let tx = Transaction::new_signed_with_payer(
        &ixs,
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}

#[tokio::test]
async fn write_to_another_memory_index() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();
    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 8);

    let build_memory = |offset: u16, write_type: WriteType| {
        MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(user.encodable_pubkey())
            .memory(memory)
            .program_id(lighthouse_sdk::ID)
            .memory_id(8)
            .memory_bump(memory_bump)
            .write_offset(offset)
            .system_program(system_program::id())
            .write_type(write_type)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            build_memory(0, WriteType::DataValue(DataValue::U8(MAX))),
            build_memory(1, WriteType::DataValue(DataValue::U8(MAX))),
            build_memory(2, WriteType::DataValue(DataValue::U16(u16::MAX))),
            build_memory(4, WriteType::DataValue(DataValue::I16(-1))),
            build_memory(6, WriteType::DataValue(DataValue::U32(u32::MAX))),
            build_memory(10, WriteType::DataValue(DataValue::I32(-1))),
            build_memory(14, WriteType::DataValue(DataValue::U64(u64::MAX))),
            build_memory(22, WriteType::DataValue(DataValue::I64(-1))),
            build_memory(30, WriteType::DataValue(DataValue::U128(u128::MAX))),
            build_memory(46, WriteType::DataValue(DataValue::I128(-1))),
            build_memory(62, WriteType::DataValue(DataValue::Bytes(vec![MAX; 32]))),
            build_memory(
                94,
                WriteType::DataValue(DataValue::Pubkey(user.encodable_pubkey())),
            ),
            build_memory(126, WriteType::DataValue(DataValue::Bool(true))),
            build_memory(127, WriteType::DataValue(DataValue::Bool(false))),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: vec![MAX; 94],
                    operator: ByteSliceOperator::Equal,
                })
                .offset(0)
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}

#[tokio::test]
async fn write_to_another_bump() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();
    let (memory, memory_bump) =
        find_memory_pda_bump_iterate(user.encodable_pubkey(), 4, 4, None).unwrap();

    let build_memory = |offset: u16, write_type: WriteType| {
        MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(user.encodable_pubkey())
            .memory(memory)
            .program_id(lighthouse_sdk::ID)
            .memory_id(4)
            .memory_bump(memory_bump)
            .write_offset(offset)
            .system_program(system_program::id())
            .write_type(write_type)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            build_memory(0, WriteType::DataValue(DataValue::U8(MAX))),
            build_memory(1, WriteType::DataValue(DataValue::U8(MAX))),
            build_memory(2, WriteType::DataValue(DataValue::U16(u16::MAX))),
            build_memory(4, WriteType::DataValue(DataValue::I16(-1))),
            build_memory(6, WriteType::DataValue(DataValue::U32(u32::MAX))),
            build_memory(10, WriteType::DataValue(DataValue::I32(-1))),
            build_memory(14, WriteType::DataValue(DataValue::U64(u64::MAX))),
            build_memory(22, WriteType::DataValue(DataValue::I64(-1))),
            build_memory(30, WriteType::DataValue(DataValue::U128(u128::MAX))),
            build_memory(46, WriteType::DataValue(DataValue::I128(-1))),
            build_memory(62, WriteType::DataValue(DataValue::Bytes(vec![MAX; 32]))),
            build_memory(
                94,
                WriteType::DataValue(DataValue::Pubkey(user.encodable_pubkey())),
            ),
            build_memory(126, WriteType::DataValue(DataValue::Bool(true))),
            build_memory(127, WriteType::DataValue(DataValue::Bool(false))),
            AssertAccountDataBuilder::new()
                .target_account(memory)
                .log_level(lighthouse_sdk::types::LogLevel::Silent)
                .assertion(DataValueAssertion::Bytes {
                    value: vec![MAX; 94],
                    operator: ByteSliceOperator::Equal,
                })
                .offset(0)
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}

#[tokio::test]
async fn cpi_check() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

    let tx = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: test_program::ID,
            accounts: test_program::accounts::Write {
                signer: user.encodable_pubkey(),
                source_account: user.encodable_pubkey(),
                lighthouse: lighthouse_sdk::ID,
                system_program: system_program::id(),
                memory,
            }
            .to_account_metas(None),
            data: test_program::instruction::Write { memory_bump }.data(),
        }],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::CrossProgramInvokeViolation),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn account_info() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let test_account = create_test_account(context, &user, false).await.unwrap();
    let test_acount_info = &mut context
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap();

    let test_account_data = &test_acount_info.data;
    let test_account_lamports = test_acount_info.lamports;
    let test_account_rent_epoch = test_acount_info.rent_epoch;

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

    let builder_fn = |write_type: WriteType, offset: u16| {
        MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(test_account.encodable_pubkey())
            .memory(memory)
            .program_id(lighthouse_sdk::ID)
            .memory_id(0)
            .memory_bump(memory_bump)
            .write_offset(offset)
            .system_program(system_program::id())
            .write_type(write_type)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(WriteType::AccountInfoField(AccountInfoField::DataLength), 0),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Executable), 8),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Owner), 16),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Lamports), 48),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::RentEpoch), 56),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Key), 64),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let account_data = context.get_account(memory).await.unwrap().data;

    assert_eq!(account_data.len(), 96);

    let actual_value = u64::try_from_slice(&account_data[0..8]).unwrap();
    assert_eq!(actual_value, test_account_data.len() as u64);

    let actual_value = bool::try_from_slice(&account_data[8..9]).unwrap();
    assert!(!actual_value);

    let actual_value = Pubkey::try_from_slice(&account_data[16..48]).unwrap();
    assert_eq!(actual_value, test_program::ID);

    let actual_value = u64::try_from_slice(&account_data[48..56]).unwrap();
    assert_eq!(actual_value, test_account_lamports);

    let actual_value = u64::try_from_slice(&account_data[56..64]).unwrap();
    assert_eq!(actual_value, test_account_rent_epoch);

    let actual_value = Pubkey::try_from_slice(&account_data[64..96]).unwrap();
    assert_eq!(actual_value, test_account.encodable_pubkey());
}

#[tokio::test]
async fn account_info_program() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let program_acount_info = &mut context.get_account(lighthouse_sdk::ID).await.unwrap();

    let program_account_data = &program_acount_info.data;
    let program_account_lamports = program_acount_info.lamports;
    let program_account_rent_epoch = program_acount_info.rent_epoch;

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

    let builder_fn = |write_type: WriteType, offset: u16| {
        MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_sdk::ID)
            .memory(memory)
            .program_id(lighthouse_sdk::ID)
            .memory_id(0)
            .memory_bump(memory_bump)
            .write_offset(offset)
            .system_program(system_program::id())
            .write_type(write_type)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(WriteType::AccountInfoField(AccountInfoField::DataLength), 0),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Executable), 8),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Owner), 16),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Lamports), 48),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::RentEpoch), 56),
            builder_fn(WriteType::AccountInfoField(AccountInfoField::Key), 64),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let account_data = context.get_account(memory).await.unwrap().data;

    assert_eq!(account_data.len(), 96);

    let actual_value = u64::try_from_slice(&account_data[0..8]).unwrap();
    assert_eq!(actual_value, program_account_data.len() as u64);

    let actual_value = bool::try_from_slice(&account_data[8..9]).unwrap();
    assert!(actual_value);

    let actual_value = Pubkey::try_from_slice(&account_data[16..48]).unwrap();
    assert_eq!(actual_value, bpf_loader::ID);

    let actual_value = u64::try_from_slice(&account_data[48..56]).unwrap();
    assert_eq!(actual_value, program_account_lamports);

    let actual_value = u64::try_from_slice(&account_data[56..64]).unwrap();
    assert_eq!(actual_value, program_account_rent_epoch);

    let actual_value = Pubkey::try_from_slice(&account_data[64..96]).unwrap();
    assert_eq!(actual_value, lighthouse_sdk::ID);
}
