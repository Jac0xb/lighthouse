use crate::utils::context::TestContext;
use crate::utils::process_transaction_assert_success;
use crate::utils::{create_user_with_balance, find_memory_account};
use lighthouse_client::instructions::{AssertAccountDeltaBuilder, MemoryWriteBuilder};
use lighthouse_client::types::{
    AccountDeltaAssertion, AccountInfoDeltaAssertion, AccountInfoField, IntegerOperator, LogLevel,
    WriteType,
};
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::system_instruction::{self};
use solana_sdk::transaction::Transaction;

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn slippage_check() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let (memory_account, bump) = find_memory_account(user.encodable_pubkey(), 0);

    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .memory_account(memory_account)
                .lighthouse_program(lighthouse_client::programs::LIGHTHOUSE_ID)
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .memory_index(0)
                .memory_offset(0)
                .memory_account_bump(bump)
                .instruction(),
            AssertAccountDeltaBuilder::new()
                .log_level(LogLevel::Silent)
                .account_a(memory_account)
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
                .memory_account(memory_account)
                .lighthouse_program(lighthouse_client::programs::LIGHTHOUSE_ID)
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .memory_index(0)
                .memory_offset(0)
                .memory_account_bump(bump)
                .instruction(),
            system_instruction::transfer(
                &user.encodable_pubkey(),
                &Keypair::new().encodable_pubkey(),
                1e9 as u64,
            ),
            AssertAccountDeltaBuilder::new()
                .log_level(LogLevel::PlaintextMsgLog)
                .account_a(memory_account)
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
