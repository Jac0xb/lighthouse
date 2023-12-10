pub mod utils;

use std::io::Error;

use lighthouse::error::ProgramError;
use lighthouse::structs::{Assertion, DataValue, Expression, Operator, WriteType};
use solana_program::instruction::InstructionError;
use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::TransactionError;
use solana_sdk::{signer::EncodableKeypair, transaction::Transaction};

use solana_banks_interface::BanksTransactionResultWithMetadata;
use utils::context::{TestContext, DEFAULT_LAMPORTS_FUND_AMOUNT};
use utils::program::Program;

pub fn find_test_account() -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &["test_account".to_string().as_ref()],
        &lighthouse::ID,
    )
}

pub fn find_cache_account(user: Pubkey, cache_index: u8) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &["cache".to_string().as_ref(), user.as_ref(), &[cache_index]],
        &lighthouse::ID,
    )
}

#[tokio::test]
async fn test_basic() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountBalance(0, Operator::GreaterThan),
            // Assertion::AccountBalance(0, Operator::LessThan),
        ],
        vec![user.encodable_pubkey(), user.encodable_pubkey()],
        None,
        None,
    );

    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;
}

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn test_borsh_account_data() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    create_test_account(context, &user).await.unwrap();
    process_transaction_assert_success(
        context,
        program
            .create_assertion(
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
                vec![find_test_account().0; 13],
                None,
                None,
            )
            .to_transaction(vec![])
            .await,
    )
    .await;
}

///
/// Test various logical expressions (false OR true), (true OR false), (true AND true).
///
#[tokio::test]
async fn test_logical_expression() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    create_test_account(context, &user).await.unwrap();

    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(5)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16(30)),
        ],
        vec![find_test_account().0; 4],
        Some(vec![
            Expression::Or(vec![0, 1]),
            Expression::Or(vec![3, 2]),
            Expression::And(vec![0, 2]),
        ]),
        None,
    );
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;

    // Test that the assertion passes when the logical expression is true.
    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(5)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16(30)),
        ],
        vec![find_test_account().0; 4],
        Some(vec![
            Expression::Or(vec![0, 1]),
            Expression::Or(vec![3, 2]),
            Expression::And(vec![0, 2]),
        ]),
        None,
    );
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;

    // Test that the assertion fails when the logical expression is false.
    let mut tx_builder = program.create_assertion(
        &user,
        vec![
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(1)),
            Assertion::AccountData(8, Operator::Equal, DataValue::U8(5)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
            Assertion::AccountData(10, Operator::Equal, DataValue::U16(30)),
        ],
        vec![find_test_account().0; 4],
        Some(vec![Expression::Or(vec![1, 2])]),
        None,
    );
    process_transaction_assert_failure(
        context,
        tx_builder.to_transaction(vec![]).await,
        to_transaction_error(0, ProgramError::AssertionFailed),
        Some(&["1 == 5".to_string(), "256 == 30".to_string()]),
    )
    .await;
}

#[tokio::test]
async fn test_account_balance() {
    let context = &mut TestContext::new().await.unwrap();

    let mut program = Program::new(context.client());
    // create_test_account(context, user).await.unwrap();
}

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
                        WriteType::AccountData(8, 128),
                    ),
                )
                .to_transaction(vec![])
                .await,
        )
        .await;

        // Assert that data was properly written to cache.
        let tx = program
            .create_assertion(
                &user,
                vec![
                    Assertion::Memory(0, Operator::Equal, DataValue::U8(1)),
                    Assertion::Memory(0, Operator::GreaterThan, DataValue::U8(0)),
                    Assertion::Memory(0, Operator::LessThan, DataValue::U8(2)),
                    Assertion::Memory(0, Operator::GreaterThanOrEqual, DataValue::U8(1)),
                    Assertion::Memory(0, Operator::LessThanOrEqual, DataValue::U8(1)),
                    Assertion::Memory(1, Operator::Equal, DataValue::I8(-1)),
                    Assertion::Memory(1, Operator::GreaterThan, DataValue::I8(-2)),
                    Assertion::Memory(1, Operator::LessThan, DataValue::I8(0)),
                    Assertion::Memory(1, Operator::GreaterThanOrEqual, DataValue::I8(-1)),
                    Assertion::Memory(1, Operator::LessThanOrEqual, DataValue::I8(-1)),
                    Assertion::Memory(2, Operator::Equal, DataValue::U16((u8::MAX as u16) + 1)),
                    Assertion::Memory(4, Operator::Equal, DataValue::I16((i8::MIN as i16) - 1)),
                    Assertion::Memory(6, Operator::Equal, DataValue::U32((u16::MAX as u32) + 1)),
                    Assertion::Memory(10, Operator::Equal, DataValue::I32((i16::MIN as i32) - 1)),
                    Assertion::Memory(14, Operator::Equal, DataValue::U64((u32::MAX as u64) + 1)),
                    Assertion::Memory(22, Operator::Equal, DataValue::I64((i32::MIN as i64) - 1)),
                    Assertion::Memory(30, Operator::Equal, DataValue::U128((u64::MAX as u128) + 1)),
                    Assertion::Memory(46, Operator::Equal, DataValue::I128((i64::MIN as i128) - 1)),
                ],
                vec![],
                None,
                Some(cache_account),
            )
            .to_transaction(vec![])
            .await;

        process_transaction_assert_success(context, tx).await;
    }
}

fn format_hex(data: &[u8]) -> String {
    let mut result = String::new();
    for (i, chunk) in data.chunks(32).enumerate() {
        // Write the offset
        result.push_str(&format!("{:08x} ({:08}): ", i * 32, i * 32));

        // Write each byte in the chunk
        for byte in chunk {
            result.push_str(&format!("{:02x} ", byte));
        }

        // Add a new line
        result.push('\r');
        result.push('\n');
    }
    result
}

async fn process_transaction(
    context: &TestContext,
    tx: &Transaction,
) -> Result<BanksTransactionResultWithMetadata, Error> {
    let result: solana_banks_interface::BanksTransactionResultWithMetadata = context
        .client()
        .process_transaction_with_metadata(tx.clone())
        .await
        .unwrap();

    Ok(result)
}

async fn process_transaction_assert_success(
    context: &TestContext,
    tx: Result<Transaction, Box<utils::Error>>,
) {
    let tx = tx.expect("Should have been processed");

    let tx_metadata = process_transaction(context, &tx).await.unwrap();

    let logs = tx_metadata.metadata.unwrap().log_messages;
    for log in logs {
        println!("{:?}", log);
    }

    if tx_metadata.result.is_err() {
        panic!("Transaction failed");
    }
}

async fn process_transaction_assert_failure(
    context: &TestContext,
    tx: Result<Transaction, Box<utils::Error>>,
    expected_error_code: TransactionError,
    log_match_regex: Option<&[String]>,
) {
    let tx = tx.expect("Should have been processed");

    let tx_metadata = process_transaction(context, &tx).await.unwrap();

    if tx_metadata.result.is_ok() {
        panic!("Transaction should have failed");
    }

    let err = tx_metadata.result.unwrap_err();

    if err != expected_error_code {
        panic!("Transaction failed with unexpected error code");
    }

    if let Some(log_regex) = log_match_regex {
        let regexes = log_regex
            .iter()
            .map(|s| regex::Regex::new(s).unwrap())
            .collect::<Vec<regex::Regex>>();

        let logs = tx_metadata.metadata.unwrap().log_messages;
        for log in &logs {
            println!("{:?}", log);
        }

        // find one log that matches each regex
        for regex in regexes {
            let mut found = false;
            for log in &logs {
                if regex.is_match(log) {
                    found = true;
                    break;
                }
            }

            if !found {
                panic!("Log not found: {}", regex);
            }
        }
    }
}

async fn create_test_account(context: &mut TestContext, payer: &Keypair) -> Result<(), Error> {
    let mut program = Program::new(context.client());
    let mut tx_builder = program.create_test_account(&payer);
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;
    Ok(())
}

async fn create_cache_account(
    context: &mut TestContext,
    user: &Keypair,
    size: u64,
) -> Result<(), Error> {
    let mut program = Program::new(context.client());
    let mut tx_builder = program.create_cache_account(&user, 0, size);
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;
    Ok(())
}

fn to_transaction_error(ix_index: u8, program_error: ProgramError) -> TransactionError {
    TransactionError::InstructionError(ix_index, InstructionError::Custom(program_error.into()))
}

async fn create_user(ctx: &mut TestContext) -> Result<Keypair, Error> {
    let user = Keypair::new();
    let _ = ctx
        .fund_account(user.encodable_pubkey(), DEFAULT_LAMPORTS_FUND_AMOUNT)
        .await;

    Ok(user)
}

// Tests to write
