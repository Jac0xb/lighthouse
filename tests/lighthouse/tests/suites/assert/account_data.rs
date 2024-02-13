use crate::utils::context::TestContext;
use crate::utils::utils::process_transaction_assert_success;
use crate::utils::{create_test_account, create_user};
use lighthouse::types::{Assertion, DataValue, Operator};
use rust_sdk::LighthouseProgram;
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn test_borsh_account_data() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let test_account = create_test_account(context, &user, false).await.unwrap();

    process_transaction_assert_success(
        context,
        program
            .create_assert_multi(
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
                vec![test_account.encodable_pubkey()],
            )
            .to_transaction_and_sign(vec![&user], context.get_blockhash())
            .unwrap(),
    )
    .await
    .unwrap();
}
