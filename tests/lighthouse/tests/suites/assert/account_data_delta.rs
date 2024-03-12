use crate::utils::context::TestContext;
use crate::utils::create_user_with_balance;
use crate::utils::process_transaction_assert_success;
use lighthouse_client::find_memory_pda;
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

    let (memory, bump) = find_memory_pda(user.encodable_pubkey(), 0);

    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .memory(memory)
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthouse_client::ID)
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
                .program_id(lighthouse_client::ID)
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
