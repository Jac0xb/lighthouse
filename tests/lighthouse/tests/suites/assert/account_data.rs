use crate::utils::context::TestContext;
use crate::utils::tx_builder::TxBuilder;
use crate::utils::utils::process_transaction_assert_success;
use crate::utils::{create_test_account, create_user_with_balance};
use lighthouse_client::instructions::AssertAccountDataBuilder;
use lighthouse_client::types::{
    BytesOperator, DataValueAssertion, EquatableOperator, IntegerOperator,
};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn borsh_account_data() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let test_account = create_test_account(context, &user, false).await.unwrap();

    let mut tx = TxBuilder {
        ixs: vec![
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(8)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I8 {
                    value: -1,
                    operator: IntegerOperator::Equal,
                })
                .offset(9)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U16 {
                    value: (u8::MAX as u16) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(10)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I16 {
                    value: (i8::MIN as i16) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(12)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U32 {
                    value: (u16::MAX as u32) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(14)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I32 {
                    value: (i16::MIN as i32) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(18)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U64 {
                    value: (u32::MAX as u64) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(22)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I64 {
                    value: (i32::MIN as i64) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(30)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U128 {
                    value: (u64::MAX as u128) + 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(38)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::I128 {
                    value: (i64::MIN as i128) - 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(54)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Bytes {
                    value: vec![u8::MAX; 32],
                    operator: BytesOperator::Equal,
                })
                .offset(70)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Bool {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .offset(102)
                .instruction(),
            // False represented as 0
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(103)
                .instruction(),
            // Some in Option<u8>
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U8 {
                    value: 1,
                    operator: IntegerOperator::Equal,
                })
                .offset(104)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U8 {
                    value: u8::MAX,
                    operator: IntegerOperator::Equal,
                })
                .offset(105)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::U8 {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .offset(106)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Bytes {
                    value: [1, 255, 255].to_vec(),
                    operator: BytesOperator::Equal,
                })
                .offset(107)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Bytes {
                    value: [0].to_vec(),
                    operator: BytesOperator::Equal,
                })
                .offset(110)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Pubkey {
                    value: user.encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                })
                .offset(111)
                .instruction(),
            AssertAccountDataBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .assertion(DataValueAssertion::Bytes {
                    value: [32, 0, 0, 0]
                        .iter()
                        .cloned()
                        .chain(vec![255; 32])
                        .collect::<Vec<u8>>(),
                    operator: BytesOperator::Equal,
                })
                .offset(143)
                .instruction(),
        ],
        look_up_tables: None,
    };

    let blockhash = context.get_blockhash().await;

    process_transaction_assert_success(
        context,
        tx.to_transaction_and_sign(vec![&user], user.encodable_pubkey(), blockhash)
            .unwrap(),
    )
    .await
    .unwrap();
}
