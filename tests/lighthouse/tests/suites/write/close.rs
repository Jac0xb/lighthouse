use core::panic;

use lighthouse::types::{Assertion, DataValue, Operator, WriteType};
use rust_sdk::{find_memory_account, LighthouseProgram};
use solana_program_test::tokio;
use solana_sdk::{signer::EncodableKeypair, transaction::Transaction};

use crate::utils::utils::process_transaction_assert_success;
use crate::utils::{context::TestContext, create_test_account, create_user};

#[tokio::test]
async fn test_create_write_assert_close() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let user = create_user(context).await.unwrap();

    let test_account = create_test_account(context, &user, false).await.unwrap();

    let create_memory_account_ix = program.create_memory_account(&user, 0, 1).ixs;

    let write_account = find_memory_account(user.encodable_pubkey(), 0).0;

    let write_ix = program
        .write_v1(
            &user,
            test_account.encodable_pubkey(),
            0,
            lighthouse::types::WriteTypeParameter::WriteU8(
                0,
                WriteType::AccountData(8, Some(1), None),
            ),
        )
        .ixs;

    let assert_ix = program
        .create_assert(
            &user,
            write_account,
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            None,
        )
        .ixs;

    let close_memory_account_ix = program.close_memory_account(&user, 0).ixs;

    let ixs_flattened = create_memory_account_ix
        .into_iter()
        .chain(write_ix.into_iter())
        .chain(assert_ix.into_iter())
        .chain(close_memory_account_ix.into_iter())
        .collect::<Vec<_>>();

    let tx_result = process_transaction_assert_success(
        context,
        Transaction::new_signed_with_payer(
            ixs_flattened.as_ref(),
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash(),
        ),
    )
    .await
    .unwrap();

    let memory_account = context.client().get_account(write_account).await.unwrap();

    println!(
        "{:?}",
        tx_result.metadata.clone().unwrap().compute_units_consumed
    );

    assert!(memory_account.is_none());
}
