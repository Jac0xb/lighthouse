use crate::utils::context::TestContext;
use crate::utils::process_transaction_assert_success;
use crate::utils::program::{
    create_cache_account, create_test_account, create_user, find_cache_account, find_test_account,
    Program,
};
use lighthouse::structs::{Assertion, DataValue, Operator, WriteType};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

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
                .to_transaction()
                .await,
        )
        .await;

        let discrim_length = 8;

        // Assert that data was properly written to cache.
        let tx = program
            .create_assertion(
                &user,
                vec![
                    Assertion::AccountData(discrim_length, Operator::Equal, DataValue::U8(1)),
                    Assertion::AccountData(discrim_length, Operator::GreaterThan, DataValue::U8(0)),
                    Assertion::AccountData(discrim_length, Operator::LessThan, DataValue::U8(2)),
                    Assertion::AccountData(
                        discrim_length,
                        Operator::GreaterThanOrEqual,
                        DataValue::U8(1),
                    ),
                    Assertion::AccountData(
                        discrim_length,
                        Operator::LessThanOrEqual,
                        DataValue::U8(1),
                    ),
                    Assertion::AccountData(discrim_length + 1, Operator::Equal, DataValue::I8(-1)),
                    Assertion::AccountData(
                        discrim_length + 1,
                        Operator::GreaterThan,
                        DataValue::I8(-2),
                    ),
                    Assertion::AccountData(
                        discrim_length + 1,
                        Operator::LessThan,
                        DataValue::I8(0),
                    ),
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
                vec![cache_account],
                None,
            )
            .to_transaction()
            .await;

        process_transaction_assert_success(context, tx).await;
    }
}
