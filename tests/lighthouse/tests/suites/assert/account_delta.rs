use crate::utils::context::TestContext;
use crate::utils::create_user;
use crate::utils::create_user_with_balance;
use crate::utils::process_transaction_assert_failure;
use crate::utils::process_transaction_assert_success;
use crate::utils::set_account_from_refs;
use crate::utils::to_transaction_error;
use borsh::BorshSerialize;
use lighthaus_sdk::errors::lighthausError;
use lighthaus_sdk::find_memory_pda;
use lighthaus_sdk::instructions::{AssertAccountDeltaBuilder, MemoryWriteBuilder};
use lighthaus_sdk::types::DataValueDeltaAssertion;
use lighthaus_sdk::types::EquatableOperator;
use lighthaus_sdk::types::IntegerOperator;
use lighthaus_sdk::types::{
    AccountDeltaAssertion, AccountInfoDeltaAssertion, AccountInfoField, LogLevel, WriteType,
};
use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::system_instruction::{self};
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use test_program::processor::TestAccountV1;

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn slippage_check() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let (memory, bump) = find_memory_pda(user.encodable_pubkey(), 0);

    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .memory(memory)
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthaus_sdk::ID)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .memory_id(0)
                .write_offset(0)
                .memory_bump(bump)
                .instruction(),
            AssertAccountDeltaBuilder::new()
                .log_level(LogLevel::Silent)
                .account_a(memory)
                .account_b(user.encodable_pubkey())
                .assertion(AccountDeltaAssertion::AccountInfo {
                    a_offset: 0,
                    assertion: AccountInfoDeltaAssertion::Lamports {
                        value: 0,
                        operator: IntegerOperator::Equal,
                    },
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .memory(memory)
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthaus_sdk::ID)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .memory_id(0)
                .write_offset(0)
                .memory_bump(bump)
                .instruction(),
            system_instruction::transfer(
                &user.encodable_pubkey(),
                &Keypair::new().encodable_pubkey(),
                1e9 as u64,
            ),
            AssertAccountDeltaBuilder::new()
                .log_level(LogLevel::PlaintextMessage)
                .account_a(memory)
                .account_b(user.encodable_pubkey())
                .assertion(AccountDeltaAssertion::AccountInfo {
                    a_offset: 0,
                    assertion: AccountInfoDeltaAssertion::Lamports {
                        value: -1e9 as i128,
                        operator: IntegerOperator::Equal,
                    },
                })
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
async fn test_data_delta() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    let test_account: TestAccountV1 = TestAccountV1 {
        u8: u8::MAX,
        i8: i8::MAX,
        u16: u16::MAX,
        i16: i16::MAX,
        u32: u32::MAX,
        i32: i32::MAX,
        u64: u64::MAX,
        i64: i64::MAX,
        u128: u128::MAX,
        i128: i128::MAX,
        bytes: [u8::MAX; 32],
        true_field: true,
        false_field: false,
        option_u8: Some(u8::MAX),
        option_u8_none: None,
        option_u16: Some(u16::MAX),
        option_u16_none: None,
        pubkey: user.encodable_pubkey(),
        vec: vec![u8::MAX; 32],
    };

    let test_account_a = Keypair::new().encodable_pubkey();
    set_account_from_refs(
        ctx,
        &test_account_a,
        &test_account.try_to_vec().unwrap(),
        &test_program::ID,
    )
    .await;

    let test_account_b = Keypair::new().encodable_pubkey();
    set_account_from_refs(
        ctx,
        &test_account_b,
        &test_account.try_to_vec().unwrap(),
        &test_program::ID,
    )
    .await;

    let build_assertion = |offset: u16, assertion: DataValueDeltaAssertion| {
        AssertAccountDeltaBuilder::new()
            .log_level(LogLevel::Silent)
            .account_a(test_account_a)
            .account_b(test_account_b)
            .assertion(AccountDeltaAssertion::Data {
                a_offset: offset,
                b_offset: offset,
                assertion,
            })
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            build_assertion(
                0,
                DataValueDeltaAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                1,
                DataValueDeltaAssertion::I8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                2,
                DataValueDeltaAssertion::U16 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                4,
                DataValueDeltaAssertion::I16 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                6,
                DataValueDeltaAssertion::U32 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                10,
                DataValueDeltaAssertion::I32 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                14,
                DataValueDeltaAssertion::U64 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                22,
                DataValueDeltaAssertion::I64 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                62,
                DataValueDeltaAssertion::Bytes {
                    length: 32,
                    operator: EquatableOperator::Equal,
                },
            ),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let test_account_default: TestAccountV1 = TestAccountV1 {
        u8: 0,
        i8: 0,
        u16: 0,
        i16: 0,
        u32: 0,
        i32: 0,
        u64: 0,
        i64: 0,
        u128: 0,
        i128: 0,
        bytes: [0; 32],
        true_field: false,
        false_field: false,
        option_u8: None,
        option_u8_none: None,
        option_u16: None,
        option_u16_none: None,
        pubkey: user.encodable_pubkey(),
        vec: vec![0; 32],
    };

    let test_account_empty = Keypair::new().encodable_pubkey();
    set_account_from_refs(
        ctx,
        &test_account_empty,
        &test_account_default.try_to_vec().unwrap(),
        &test_program::ID,
    )
    .await;

    let build_assertion = |offset: u16, assertion: DataValueDeltaAssertion| {
        AssertAccountDeltaBuilder::new()
            .log_level(LogLevel::PlaintextMessage)
            .account_a(test_account_empty)
            .account_b(test_account_b)
            .assertion(AccountDeltaAssertion::Data {
                a_offset: offset,
                b_offset: offset,
                assertion,
            })
            .instruction()
    };

    println!("test_account: {:?}", test_account.u8 as i16);

    let tx = Transaction::new_signed_with_payer(
        &[
            build_assertion(
                0,
                DataValueDeltaAssertion::U8 {
                    value: test_account.u8 as i16,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                1,
                DataValueDeltaAssertion::I8 {
                    value: test_account.i8 as i16,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                2,
                DataValueDeltaAssertion::U16 {
                    value: test_account.u16 as i32,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                4,
                DataValueDeltaAssertion::I16 {
                    value: test_account.i16 as i32,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                6,
                DataValueDeltaAssertion::U32 {
                    value: test_account.u32 as i64,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                10,
                DataValueDeltaAssertion::I32 {
                    value: test_account.i32 as i64,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                14,
                DataValueDeltaAssertion::U64 {
                    value: test_account.u64 as i128,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                22,
                DataValueDeltaAssertion::I64 {
                    value: test_account.i64 as i128,
                    operator: IntegerOperator::Equal,
                },
            ),
            build_assertion(
                62,
                DataValueDeltaAssertion::Bytes {
                    length: 32,
                    operator: EquatableOperator::NotEqual,
                },
            ),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();
}

#[tokio::test]
async fn test_account_info_delta() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    let test_account = Keypair::new().encodable_pubkey();

    let build_assertion = |offset: u16, assertion: AccountInfoDeltaAssertion| {
        AssertAccountDeltaBuilder::new()
            .log_level(LogLevel::PlaintextMessage)
            .account_a(test_account)
            .account_b(user.encodable_pubkey())
            .assertion(AccountDeltaAssertion::AccountInfo {
                a_offset: offset,
                assertion,
            })
            .instruction()
    };

    // Lamport test
    set_borsh_account(ctx, &test_account, 69_000u64).await;

    let user_balance = ctx
        .get_account(user.encodable_pubkey())
        .await
        .unwrap()
        .lamports;

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::Lamports {
                value: (user_balance as i128) - 69_000 - 5000,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // failure case

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::Lamports {
                value: (user_balance as i128) - 69_000 - 5000,
                operator: IntegerOperator::GreaterThan,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test data length

    set_borsh_account(ctx, &test_account, 100u64).await;

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::DataLength {
                value: -100,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // failure case

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::DataLength {
                value: 100,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test ownership, no change

    set_borsh_account(ctx, &test_account, system_program::ID).await;

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::Owner {
                operator: EquatableOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // failure case

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::Owner {
                operator: EquatableOperator::NotEqual,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test ownership, change

    set_borsh_account(ctx, &test_account, user.encodable_pubkey()).await;

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::Owner {
                operator: EquatableOperator::NotEqual,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // failure case

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::Owner {
                operator: EquatableOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test rent epoch

    let user_rent_epoch = ctx
        .get_account(user.encodable_pubkey())
        .await
        .unwrap()
        .rent_epoch;

    set_borsh_account(ctx, &test_account, user_rent_epoch - 10).await;

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::RentEpoch {
                value: 10,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // failure case

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::RentEpoch {
                value: 10,
                operator: IntegerOperator::LessThan,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Test rent epoch, no change

    set_borsh_account(ctx, &test_account, user_rent_epoch).await;

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            0,
            AccountInfoDeltaAssertion::RentEpoch {
                value: 0,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();
}

#[tokio::test]
async fn out_of_bounds_account_info_delta() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    // Test lamports

    let test_account = Keypair::new().encodable_pubkey();
    set_borsh_account(ctx, &test_account, [u8::MAX, 8]).await;

    let build_assertion = |offset: u16, assertion: AccountInfoDeltaAssertion| {
        AssertAccountDeltaBuilder::new()
            .log_level(LogLevel::PlaintextMessage)
            .account_a(test_account)
            .account_b(user.encodable_pubkey())
            .assertion(AccountDeltaAssertion::AccountInfo {
                a_offset: offset,
                assertion,
            })
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            4,
            AccountInfoDeltaAssertion::Lamports {
                value: 0,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();

    // Test data length

    set_borsh_account(ctx, &test_account, [u8::MAX, 8]).await;
    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            2,
            AccountInfoDeltaAssertion::DataLength {
                value: 0,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();

    // Test ownership

    set_borsh_account(ctx, &test_account, [u8::MAX, 128]).await;
    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            128 - 31,
            AccountInfoDeltaAssertion::Owner {
                operator: EquatableOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();

    // Test rent epoch

    set_borsh_account(ctx, &test_account, [u8::MAX, 128]).await;
    let tx = Transaction::new_signed_with_payer(
        &[build_assertion(
            128 - 4,
            AccountInfoDeltaAssertion::RentEpoch {
                value: 0,
                operator: IntegerOperator::Equal,
            },
        )],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::RangeOutOfBounds),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn account_empty() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    let test_account = Keypair::new().encodable_pubkey();

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountDeltaBuilder::new()
            .log_level(LogLevel::Silent)
            .account_a(test_account)
            .account_b(user.encodable_pubkey())
            .assertion(AccountDeltaAssertion::AccountInfo {
                a_offset: 0,
                assertion: AccountInfoDeltaAssertion::Lamports {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, lighthausError::AccountNotInitialized),
        None,
    )
    .await
    .unwrap();
}

async fn set_borsh_account(ctx: &mut TestContext, account: &Pubkey, data: impl BorshSerialize) {
    let data = data.try_to_vec().unwrap();
    set_account_from_refs(ctx, account, &data, &test_program::ID).await;
}
