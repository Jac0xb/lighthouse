use lighthouse::types::{Assertion, DataValue, Operator, WriteType};
use rust_sdk::{find_memory_account, LighthouseProgram};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

use crate::utils::{
    context::TestContext, create_memory_account, create_test_account, create_user,
    process_transaction_assert_success,
};

#[tokio::test]
async fn test_write() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    // Create test account
    let test_account = create_test_account(context, &user, false).await.unwrap();
    let _ = create_memory_account(context, &user, 256).await;

    let memory_account = find_memory_account(user.encodable_pubkey(), 0).0;

    // Test writing account data to memory.
    process_transaction_assert_success(
        context,
        program
            .write_v1(
                &user,
                test_account.encodable_pubkey(),
                0,
                lighthouse::types::WriteTypeParameter::WriteU8(
                    0,
                    WriteType::AccountData(8, Some(128), None),
                ),
            )
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();

    let discrim_length = 8;

    // Assert that data was properly written to memory.
    // Program L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK consumed 85510 of 1400000 compute units
    let tx = program
        .create_assert_multi(
            &user,
            vec![
                Assertion::AccountData(discrim_length + 0, Operator::Equal, DataValue::U8(1)),
                Assertion::AccountData(discrim_length + 0, Operator::GreaterThan, DataValue::U8(0)),
                Assertion::AccountData(discrim_length + 0, Operator::LessThan, DataValue::U8(2)),
                Assertion::AccountData(
                    discrim_length + 0,
                    Operator::GreaterThanOrEqual,
                    DataValue::U8(1),
                ),
                Assertion::AccountData(
                    discrim_length + 0,
                    Operator::LessThanOrEqual,
                    DataValue::U8(1),
                ),
                Assertion::AccountData(discrim_length + 1, Operator::Equal, DataValue::I8(-1)),
                Assertion::AccountData(
                    discrim_length + 1,
                    Operator::GreaterThan,
                    DataValue::I8(-2),
                ),
                Assertion::AccountData(discrim_length + 1, Operator::LessThan, DataValue::I8(0)),
                Assertion::AccountData(
                    discrim_length + 1,
                    Operator::GreaterThanOrEqual,
                    DataValue::I8(-1),
                ),
                Assertion::AccountData(
                    discrim_length + 1,
                    Operator::LessThanOrEqual,
                    DataValue::I8(-1),
                ),
                Assertion::AccountData(
                    discrim_length + 2,
                    Operator::Equal,
                    DataValue::U16((u8::MAX as u16) + 1),
                ),
                Assertion::AccountData(
                    discrim_length + 4,
                    Operator::Equal,
                    DataValue::I16((i8::MIN as i16) - 1),
                ),
                Assertion::AccountData(
                    discrim_length + 6,
                    Operator::Equal,
                    DataValue::U32((u16::MAX as u32) + 1),
                ),
                Assertion::AccountData(
                    discrim_length + 10,
                    Operator::Equal,
                    DataValue::I32((i16::MIN as i32) - 1),
                ),
                Assertion::AccountData(
                    discrim_length + 14,
                    Operator::Equal,
                    DataValue::U64((u32::MAX as u64) + 1),
                ),
                Assertion::AccountData(
                    discrim_length + 22,
                    Operator::Equal,
                    DataValue::I64((i32::MIN as i64) - 1),
                ),
                Assertion::AccountData(
                    discrim_length + 30,
                    Operator::Equal,
                    DataValue::U128((u64::MAX as u128) + 1),
                ),
                Assertion::AccountData(
                    discrim_length + 46,
                    Operator::Equal,
                    DataValue::I128((i64::MIN as i128) - 1),
                ),
            ],
            vec![memory_account],
        )
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Assert that data was properly written to memory.
    // L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK consumed 7872 of 1400000 compute units
    let tx_compact = program
        .create_assert_multi_compact(
            &user,
            vec![
                Assertion::AccountData(discrim_length + 0, Operator::Equal, DataValue::U8(1)),
                Assertion::AccountData(discrim_length + 0, Operator::GreaterThan, DataValue::U8(0)),
                Assertion::AccountData(discrim_length + 0, Operator::LessThan, DataValue::U8(2)),
                Assertion::AccountData(
                    discrim_length + 0,
                    Operator::GreaterThanOrEqual,
                    DataValue::U8(1),
                ),
                Assertion::AccountData(
                    discrim_length + 0,
                    Operator::LessThanOrEqual,
                    DataValue::U8(1),
                ),
                Assertion::AccountData(discrim_length + 1, Operator::Equal, DataValue::I8(-1)),
                Assertion::AccountData(
                    discrim_length + 1,
                    Operator::GreaterThan,
                    DataValue::I8(-2),
                ),
                Assertion::AccountData(discrim_length + 1, Operator::LessThan, DataValue::I8(0)),
                Assertion::AccountData(
                    discrim_length + 1,
                    Operator::GreaterThanOrEqual,
                    DataValue::I8(-1),
                ),
                Assertion::AccountData(
                    discrim_length + 1,
                    Operator::LessThanOrEqual,
                    DataValue::I8(-1),
                ),
                Assertion::AccountData(
                    discrim_length + 2,
                    Operator::Equal,
                    DataValue::U16((u8::MAX as u16) + 1),
                ),
                Assertion::AccountData(
                    discrim_length + 4,
                    Operator::Equal,
                    DataValue::I16((i8::MIN as i16) - 1),
                ),
                Assertion::AccountData(
                    discrim_length + 6,
                    Operator::Equal,
                    DataValue::U32((u16::MAX as u32) + 1),
                ),
                Assertion::AccountData(
                    discrim_length + 10,
                    Operator::Equal,
                    DataValue::I32((i16::MIN as i32) - 1),
                ),
                Assertion::AccountData(
                    discrim_length + 14,
                    Operator::Equal,
                    DataValue::U64((u32::MAX as u64) + 1),
                ),
                Assertion::AccountData(
                    discrim_length + 22,
                    Operator::Equal,
                    DataValue::I64((i32::MIN as i64) - 1),
                ),
            ],
            vec![memory_account],
        )
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap();

    process_transaction_assert_success(context, tx_compact)
        .await
        .unwrap();
}
