pub mod utils;

use std::io::Error;

use anchor_lang::InstructionData;
use lighthouse::structs::{Assertion, BorshField, BorshValue, Expression, Operator};
use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::{signer::EncodableKeypair, transaction::Transaction};

use solana_banks_interface::BanksTransactionResultWithMetadata;
use utils::context::TestContext;
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

/*
#[tokio::test]
async fn test_basic() {
    let context = TestContext::new().await.unwrap();

    let mut program = Program::new(context.client());

    let mut tx_builder = program.create_assertion(
        &context.payer(),
        vec![
            Assertion::AccountBalance(0, Operator::GreaterThan),
            Assertion::AccountBalance(0, Operator::LessThan),
        ],
        vec![
            context.payer().encodable_pubkey(),
            context.payer().encodable_pubkey(),
        ],
        None,
    );

    let tx = tx_builder.to_transaction(vec![]).await;

    if let Err(err) = tx {
        println!("err: {:?}", err);

        panic!("Should have passed");
    } else if let Ok(tx) = tx {
        println!("Tx size: {}", tx.message().serialize().len());

        let response = context
            .client()
            .process_transaction_with_metadata(tx)
            .await
            .unwrap();

        let logs = response.metadata.unwrap().log_messages;

        for log in logs {
            println!("{:?}", log);
        }
    } else {
        panic!("Should have passed");
    }
}
*/

/*
#[tokio::test]
async fn test_borsh_account_data() {
    let context = TestContext::new().await.unwrap();

    let mut program = Program::new(context.client());

    let account = find_cache().0;

    let mut tx_builder = program.create_assertion(
        &context.payer(),
        vec![
            Assertion::BorshAccountData(
                8,
                BorshField::U8,
                Operator::Equal,
                lighthouse::BorshValue::U8(1),
            ),
            Assertion::BorshAccountData(
                9,
                BorshField::I8,
                Operator::Equal,
                lighthouse::BorshValue::I8(-1),
            ),
            Assertion::BorshAccountData(
                10,
                BorshField::U16,
                Operator::Equal,
                lighthouse::BorshValue::U16((u8::MAX as u16) + 1),
            ),
            Assertion::BorshAccountData(
                12,
                BorshField::I16,
                Operator::Equal,
                lighthouse::BorshValue::I16((i8::MIN as i16) - 1),
            ),
            Assertion::BorshAccountData(
                14,
                BorshField::U32,
                Operator::Equal,
                lighthouse::BorshValue::U32((u16::MAX as u32) + 1),
            ),
            Assertion::BorshAccountData(
                18,
                BorshField::I32,
                Operator::Equal,
                lighthouse::BorshValue::I32((i16::MIN as i32) - 1),
            ),
            Assertion::BorshAccountData(
                22,
                BorshField::U64,
                Operator::Equal,
                lighthouse::BorshValue::U64((u32::MAX as u64) + 1),
            ),
            Assertion::BorshAccountData(
                30,
                BorshField::I64,
                Operator::Equal,
                lighthouse::BorshValue::I64((i32::MIN as i64) - 1),
            ),
            Assertion::BorshAccountData(
                38,
                BorshField::U128,
                Operator::Equal,
                lighthouse::BorshValue::U128((u64::MAX as u128) + 1),
            ),
            Assertion::BorshAccountData(
                54,
                BorshField::I128,
                Operator::Equal,
                lighthouse::BorshValue::I128((i64::MIN as i128) - 1),
            ),
        ],
        vec![
            account, account, account, account, account, account, account, account, account,
            account, account,
        ],
        None,
    );

    let tx = &tx_builder
        .to_transaction(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: lighthouse::accounts::CreateTestAccount {
                signer: context.payer().encodable_pubkey(),
                test_account: find_cache().0,
                rent: Rent::id(),
                system_program: System::id(),
            }
            .to_account_metas(None),
            data: (lighthouse::instruction::CreateTestAccount {}).data(),
        }])
        .await;

    if let Err(err) = tx {
        println!("err: {:?}", err);
        panic!("Should have passed");
    } else if let Ok(tx) = tx {
        println!("Tx size: {}", tx.message().serialize().len());

        let response = context
            .client()
            .process_transaction_with_metadata(tx.clone())
            .await
            .unwrap();

        let logs = response.metadata.unwrap().log_messages;

        for log in logs {
            println!("{:?}", log);
        }

        println!(
            "account: {:?}",
            context
                .client()
                .get_account(find_cache().0)
                .await
                .unwrap()
                .unwrap()
                .data
        );
    } else {
        panic!("Should have passed");
    }
}
*/

/*
#[tokio::test]
async fn test_logical_expression() {
    let context = TestContext::new().await.unwrap();

    let mut program = Program::new(context.client());

    let account = find_cache().0;

    let mut tx_builder = program.create_assertion(
        &context.payer(),
        vec![
            Assertion::BorshAccountData(8, BorshField::U8, Operator::Equal, BorshValue::U8(1)),
            Assertion::BorshAccountData(8, BorshField::U8, Operator::Equal, BorshValue::U8(5)),
            Assertion::BorshAccountData(
                10,
                BorshField::U16,
                Operator::Equal,
                BorshValue::U16((u8::MAX as u16) + 1),
            ),
            Assertion::BorshAccountData(
                10,
                BorshField::U16,
                Operator::Equal,
                BorshValue::U16(30),
            ),
        ],
        vec![account, account, account, account],
        Some(vec![
            Expression::Or(vec![0, 1]),
            Expression::And(vec![2, 3]),
        ]),
    );

    // let value = &Expression::Or(vec![0, 1])

    let tx = &tx_builder
        .to_transaction(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: lighthouse::accounts::CreateTestAccount {
                signer: context.payer().encodable_pubkey(),
                test_account: find_cache().0,
                rent: Rent::id(),
                system_program: System::id(),
            }
            .to_account_metas(None),
            data: (lighthouse::instruction::CreateTestAccount {}).data(),
        }])
        .await;

    if let Err(err) = tx {
        println!("err: {:?}", err);
        panic!("Should have passed");
    } else if let Ok(tx) = tx {
        println!("Tx size: {}", tx.message().serialize().len());

        let response = context
            .client()
            .process_transaction_with_metadata(tx.clone())
            .await
            .unwrap();

        let logs = response.metadata.unwrap().log_messages;

        for log in logs {
            println!("{:?}", log);
        }

        println!(
            "account: {:?}",
            context
                .client()
                .get_account(find_cache().0)
                .await
                .unwrap()
                .unwrap()
                .data
        );
    } else {
        panic!("Should have passed");
    }
}

*/

/*
#[tokio::test]
async fn test_raw_account_data() {
    let context = TestContext::new().await.unwrap();

    let mut program = Program::new(context.client());

    let mut tx_builder = program.create_assertion(
        &context.payer(),
        vec![
            Assertion::AccountBalance(0, Operator::GreaterThan),
            Assertion::AccountBalance(0, Operator::LessThan),
        ],
        vec![
            context.payer().encodable_pubkey(),
            context.payer().encodable_pubkey(),
        ],
        None,
    );

    let tx = tx_builder.to_transaction(vec![]).await;

    if let Err(err) = tx {
        println!("err: {:?}", err);

        panic!("Should have passed");
    } else if let Ok(tx) = tx {
        // println!("tx: {:?}", tx);

        let response = context
            .client()
            .process_transaction_with_metadata(tx)
            .await
            .unwrap();

        let logs = response.metadata.unwrap().log_messages;

        for log in logs {
            println!("{:?}", log);
        }
    } else {
        panic!("Should have passed");
    }

    let account = find_cache().0;

    let mut tx_builder = program.create_assertion(
        &context.payer(),
        vec![Assertion::RawAccountData(
            0,
            Operator::Equal,
            vec![200, 208, 249, 117, 197, 42, 20, 255],
        )],
        vec![account],
        None,
    );

    let tx = tx_builder
        .to_transaction(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: lighthouse::accounts::CreateTestAccount {
                signer: context.payer().encodable_pubkey(),
                test_account: find_cache().0,
                rent: Rent::id(),
                system_program: System::id(),
            }
            .to_account_metas(None),
            data: (lighthouse::instruction::CreateTestAccount {}).data(),
        }])
        .await;

    if let Err(err) = tx {
        println!("err: {:?}", err);
        panic!("Should have passed");
    } else if let Ok(tx) = tx {
        println!("Tx size: {}", tx.message().serialize().len());

        let response = context
            .client()
            .process_transaction_with_metadata(tx)
            .await
            .unwrap();

        let logs = response.metadata.unwrap().log_messages;

        for log in logs {
            println!("{:?}", log);
        }

        println!(
            "account: {:?}",
            context
                .client()
                .get_account(find_cache().0)
                .await
                .unwrap()
                .unwrap()
                .data
        );
    } else {
        panic!("Should have passed");
    }
}
*/

#[tokio::test]
async fn create_cache_account() {
    let context = TestContext::new().await.unwrap();

    let mut program = Program::new(context.client());
    let mut create_cache_builder = program.create_cache_account(&context.payer(), 0, 64);
    let tx = create_cache_builder.to_transaction(vec![]).await;

    let tx = tx.expect("Should have been processed");
    let tx_metadata = process_transaction(&context, &tx)
        .await
        .unwrap()
        .metadata
        .unwrap();
    let logs = tx_metadata.log_messages;
    for log in logs {
        println!("{:?}", log);
    }

    let data = context
        .client()
        .get_account(find_cache_account(context.payer().encodable_pubkey(), 0).0)
        .await
        .unwrap()
        .unwrap()
        .data;

    println!("{}", format_hex(&data));

    let tx = program
        .create_test_account(&context.payer())
        .to_transaction(vec![])
        .await;
    process_transaction_assert_success(&context, tx).await;

    let mut load_cache_builder =
        program.load_cache_account(&context.payer(), find_test_account().0, 0, 0, 8, 128);
    let tx = load_cache_builder.to_transaction(vec![]).await;
    let tx = tx.expect("Should have been processed");
    println!("Tx size: {}", tx.message().serialize().len());

    let tx_metadata = process_transaction(&context, &tx)
        .await
        .unwrap()
        .metadata
        .unwrap();
    let logs = tx_metadata.log_messages;
    for log in logs {
        println!("{:?}", log);
    }

    let data = context
        .client()
        .get_account(find_cache_account(context.payer().encodable_pubkey(), 0).0)
        .await
        .unwrap()
        .unwrap()
        .data;

    println!("cache account {}", format_hex(&data));

    let data = context
        .client()
        .get_account(find_test_account().0)
        .await
        .unwrap()
        .unwrap()
        .data;

    println!("test account {}", format_hex(&data));

    let offset = 0;
    let account = find_cache_account(context.payer().encodable_pubkey(), 0).0;

    let mut tx = program
        .create_assertion(
            &context.payer(),
            vec![
                Assertion::BorshAccountData(
                    offset + 8,
                    BorshField::U8,
                    Operator::Equal,
                    BorshValue::U8(1),
                ),
                Assertion::BorshAccountData(
                    offset + 9,
                    BorshField::I8,
                    Operator::Equal,
                    BorshValue::I8(-1),
                ),
                Assertion::BorshAccountData(
                    offset + 10,
                    BorshField::U16,
                    Operator::Equal,
                    BorshValue::U16((u8::MAX as u16) + 1),
                ),
                Assertion::BorshAccountData(
                    offset + 12,
                    BorshField::I16,
                    Operator::Equal,
                    BorshValue::I16((i8::MIN as i16) - 1),
                ),
                Assertion::BorshAccountData(
                    offset + 14,
                    BorshField::U32,
                    Operator::Equal,
                    BorshValue::U32((u16::MAX as u32) + 1),
                ),
                Assertion::BorshAccountData(
                    offset + 18,
                    BorshField::I32,
                    Operator::Equal,
                    BorshValue::I32((i16::MIN as i32) - 1),
                ),
                Assertion::BorshAccountData(
                    offset + 22,
                    BorshField::U64,
                    Operator::Equal,
                    BorshValue::U64((u32::MAX as u64) + 1),
                ),
                Assertion::BorshAccountData(
                    offset + 30,
                    BorshField::I64,
                    Operator::Equal,
                    BorshValue::I64((i32::MIN as i64) - 1),
                ),
                Assertion::BorshAccountData(
                    offset + 38,
                    BorshField::U128,
                    Operator::Equal,
                    BorshValue::U128((u64::MAX as u128) + 1),
                ),
                Assertion::BorshAccountData(
                    offset + 54,
                    BorshField::I128,
                    Operator::Equal,
                    BorshValue::I128((i64::MIN as i128) - 1),
                ),
            ],
            vec![
                account, account, account, account, account, account, account, account, account,
                account, account,
            ],
            None,
        )
        .to_transaction(vec![])
        .await;

    process_transaction_assert_success(&context, tx).await;
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
    // .metadata
    // .unwrap();

    Ok(result)
}

async fn process_transaction_assert_success(
    context: &TestContext,
    tx: Result<Transaction, Box<utils::Error>>,
) {
    let tx = tx.expect("Should have been processed");

    // if (tx_metadata)

    let tx_metadata = process_transaction(&context, &tx).await.unwrap();

    let logs = tx_metadata.metadata.unwrap().log_messages;
    for log in logs {
        println!("{:?}", log);
    }

    if tx_metadata.result.is_err() {
        panic!("Transaction failed");
    }
}
