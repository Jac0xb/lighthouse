use crate::utils::utils::process_transaction_assert_success;
use crate::utils::{context::TestContext, create_memory_account, create_test_account, create_user};
use lighthouse::types::{Assertion, ComparableOperator, DataValueAssertion, WriteType};
use rust_sdk::{find_memory_account, LighthouseProgram};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

#[tokio::test]
async fn test_write() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    // Create test account
    let test_account = create_test_account(context, &user, false).await.unwrap();
    let _ = create_memory_account(context, &user, 256).await;

    let memory_account = find_memory_account(user.encodable_pubkey(), 0).0;

    println!("user pubkey: {:?}", user.encodable_pubkey());

    let tx = program
        .write_v1(
            &user,
            test_account.encodable_pubkey(),
            0,
            lighthouse::types::WriteTypeParameter::WriteU8(0, WriteType::AccountData(8, Some(128))),
        )
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap();

    println!("Transaction: {:?}", tx);

    // Test writing account data to memory.
    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Assert that data was properly written to memory.
    // Program L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK consumed 85510 of 1400000 compute units
    let tx = program
        .create_assert_multi(
            &user,
            vec![
                Assertion::AccountData(0, DataValueAssertion::U8(1, ComparableOperator::Equal)),
                Assertion::AccountData(
                    0,
                    DataValueAssertion::U8(0, ComparableOperator::GreaterThan),
                ),
                Assertion::AccountData(0, DataValueAssertion::U8(2, ComparableOperator::LessThan)),
                Assertion::AccountData(
                    0,
                    DataValueAssertion::U8(1, ComparableOperator::GreaterThanOrEqual),
                ),
                Assertion::AccountData(
                    0,
                    DataValueAssertion::U8(1, ComparableOperator::LessThanOrEqual),
                ),
                Assertion::AccountData(1, DataValueAssertion::I8(-1, ComparableOperator::Equal)),
                Assertion::AccountData(
                    1,
                    DataValueAssertion::I8(-2, ComparableOperator::GreaterThan),
                ),
                Assertion::AccountData(1, DataValueAssertion::I8(0, ComparableOperator::LessThan)),
                Assertion::AccountData(
                    1,
                    DataValueAssertion::I8(-1, ComparableOperator::GreaterThanOrEqual),
                ),
                Assertion::AccountData(
                    1,
                    DataValueAssertion::I8(-1, ComparableOperator::LessThanOrEqual),
                ),
                Assertion::AccountData(
                    2,
                    DataValueAssertion::U16((u8::MAX as u16) + 1, ComparableOperator::Equal),
                ),
                Assertion::AccountData(
                    4,
                    DataValueAssertion::I16((i8::MIN as i16) - 1, ComparableOperator::Equal),
                ),
                Assertion::AccountData(
                    6,
                    DataValueAssertion::U32((u16::MAX as u32) + 1, ComparableOperator::Equal),
                ),
                Assertion::AccountData(
                    10,
                    DataValueAssertion::I32((i16::MIN as i32) - 1, ComparableOperator::Equal),
                ),
                Assertion::AccountData(
                    14,
                    DataValueAssertion::U64((u32::MAX as u64) + 1, ComparableOperator::Equal),
                ),
                Assertion::AccountData(
                    22,
                    DataValueAssertion::I64((i32::MIN as i64) - 1, ComparableOperator::Equal),
                ),
                Assertion::AccountData(
                    30,
                    DataValueAssertion::U128((u64::MAX as u128) + 1, ComparableOperator::Equal),
                ),
                Assertion::AccountData(
                    46,
                    DataValueAssertion::I128((i64::MIN as i128) - 1, ComparableOperator::Equal),
                ),
            ],
            vec![memory_account],
        )
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}
