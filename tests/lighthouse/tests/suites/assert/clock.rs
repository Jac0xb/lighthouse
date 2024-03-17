use crate::utils::context::TestContext;
use crate::utils::create_user;
use crate::utils::process_transaction_assert_failure;
use crate::utils::process_transaction_assert_success;
use crate::utils::to_transaction_error;
use borsh::BorshDeserialize;
use lighthouse_client::errors::LighthouseError;
use lighthouse_client::find_memory_pda;
use lighthouse_client::instructions::{AssertSysvarClockBuilder, MemoryWriteBuilder};
use lighthouse_client::types::{
    ClockField, IntegerOperator, LogLevel, SysvarClockAssertion, WriteType,
};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn simple() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    ctx.warp_to_slot(100_000_000).expect("warp to slot failed");

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

    // No easy way to get epoch from bankclient so we just use lighthouse to write it to memory lol

    let tx = Transaction::new_signed_with_payer(
        &[MemoryWriteBuilder::new()
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(0)
            .program_id(lighthouse_client::ID)
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_client::ID)
            .write_offset(0)
            .write_type(WriteType::Clock(ClockField::Epoch))
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let epoch = u64::try_from_slice(
        ctx.client()
            .get_account(memory)
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Epoch {
                    value: epoch,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Epoch {
                    value: epoch / 2,
                    operator: IntegerOperator::GreaterThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Epoch {
                    value: epoch * 2,
                    operator: IntegerOperator::LessThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Epoch {
                    value: 0,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Store slot in memory

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 1);

    let tx = Transaction::new_signed_with_payer(
        &[MemoryWriteBuilder::new()
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(1)
            .program_id(lighthouse_client::ID)
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_client::ID)
            .write_offset(0)
            .write_type(WriteType::Clock(ClockField::Slot))
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let slot = u64::try_from_slice(
        ctx.client()
            .get_account(memory)
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Slot {
                    value: slot,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Slot {
                    value: slot / 2,
                    operator: IntegerOperator::GreaterThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Slot {
                    value: slot * 2,
                    operator: IntegerOperator::LessThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::Slot {
                    value: 0,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Store unix timestamp in memory

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 2);

    let tx = Transaction::new_signed_with_payer(
        &[MemoryWriteBuilder::new()
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(2)
            .program_id(lighthouse_client::ID)
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_client::ID)
            .write_offset(0)
            .write_type(WriteType::Clock(ClockField::UnixTimestamp))
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let unix_timestamp = i64::try_from_slice(
        ctx.client()
            .get_account(memory)
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::UnixTimestamp {
                    value: unix_timestamp,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::UnixTimestamp {
                    value: unix_timestamp / 2,
                    operator: IntegerOperator::GreaterThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::UnixTimestamp {
                    value: unix_timestamp * 2,
                    operator: IntegerOperator::LessThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::UnixTimestamp {
                    value: 0,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Store leader schedule epoch in memory

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 3);

    let tx = Transaction::new_signed_with_payer(
        &[MemoryWriteBuilder::new()
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(3)
            .program_id(lighthouse_client::ID)
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_client::ID)
            .write_offset(0)
            .write_type(WriteType::Clock(ClockField::LeaderScheduleEpoch))
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let leader_schedule_epoch = u64::try_from_slice(
        ctx.client()
            .get_account(memory)
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::LeaderScheduleEpoch {
                    value: leader_schedule_epoch,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::LeaderScheduleEpoch {
                    value: leader_schedule_epoch / 2,
                    operator: IntegerOperator::GreaterThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::LeaderScheduleEpoch {
                    value: leader_schedule_epoch * 2,
                    operator: IntegerOperator::LessThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::LeaderScheduleEpoch {
                    value: 0,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Store epoch start timestamp in memory

    let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 4);

    let tx = Transaction::new_signed_with_payer(
        &[MemoryWriteBuilder::new()
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(4)
            .program_id(lighthouse_client::ID)
            .payer(user.encodable_pubkey())
            .source_account(lighthouse_client::ID)
            .write_offset(0)
            .write_type(WriteType::Clock(ClockField::EpochStartTimestamp))
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let epoch_start_timestamp = i64::try_from_slice(
        ctx.client()
            .get_account(memory)
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::EpochStartTimestamp {
                    value: epoch_start_timestamp,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::EpochStartTimestamp {
                    value: epoch_start_timestamp / 2,
                    operator: IntegerOperator::GreaterThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::EpochStartTimestamp {
                    value: epoch_start_timestamp * 2,
                    operator: IntegerOperator::LessThan,
                })
                .instruction(),
            AssertSysvarClockBuilder::new()
                .log_level(LogLevel::Silent)
                .assertion(SysvarClockAssertion::EpochStartTimestamp {
                    value: 0,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Failures

    let tx = Transaction::new_signed_with_payer(
        &[AssertSysvarClockBuilder::new()
            .log_level(LogLevel::Silent)
            .assertion(SysvarClockAssertion::Epoch {
                value: epoch * 2,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertSysvarClockBuilder::new()
            .log_level(LogLevel::Silent)
            .assertion(SysvarClockAssertion::Slot {
                value: slot * 2,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertSysvarClockBuilder::new()
            .log_level(LogLevel::Silent)
            .assertion(SysvarClockAssertion::UnixTimestamp {
                value: unix_timestamp * 2,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertSysvarClockBuilder::new()
            .log_level(LogLevel::Silent)
            .assertion(SysvarClockAssertion::LeaderScheduleEpoch {
                value: leader_schedule_epoch * 2,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertSysvarClockBuilder::new()
            .log_level(LogLevel::Silent)
            .assertion(SysvarClockAssertion::EpochStartTimestamp {
                value: epoch_start_timestamp * 2,
                operator: IntegerOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertSysvarClockBuilder::new()
            .log_level(LogLevel::Silent)
            .assertion(SysvarClockAssertion::Epoch {
                value: epoch,
                operator: IntegerOperator::GreaterThan,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_failure(
        ctx,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}
