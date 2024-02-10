use super::context::TestContext;
use super::error::Error;
use crate::utils;
use lighthouse::error::LighthouseError;
use solana_banks_interface::BanksTransactionResultWithMetadata;
use solana_program::instruction::InstructionError;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::BanksClient;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::{Transaction, TransactionError};

pub async fn process_transaction(
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

pub async fn process_transaction_assert_success(context: &TestContext, tx: Transaction) {
    let tx_metadata = process_transaction(context, &tx).await;

    if let Err(err) = tx_metadata {
        panic!("Transaction failed to process: {:?}", err);
    }

    let tx_metadata = tx_metadata.unwrap();

    if let Some(logs) = tx_metadata.metadata.clone().map(|m| m.log_messages) {
        println!("Transaction Logs:");
        for log in logs {
            println!("{}", log);
        }
    }

    if tx_metadata.result.is_err() {
        println!("Tx Result {:?}", tx_metadata.result.clone().err());
        panic!("Transaction failed");
    }
}

pub async fn process_transaction_assert_failure(
    context: &TestContext,
    tx: Transaction,
    expected_error_code: TransactionError,
    log_match_regex: Option<&[String]>,
) {
    let tx_metadata = process_transaction(context, &tx).await.unwrap();

    let logs = tx_metadata.metadata.clone().unwrap().log_messages;
    for log in logs {
        println!("{:?}", log);
    }

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

pub fn to_transaction_error(ix_index: u8, program_error: LighthouseError) -> TransactionError {
    TransactionError::InstructionError(ix_index, InstructionError::Custom(program_error.into()))
}

pub async fn build_tx(
    ixs: Vec<Instruction>,
    signers: Vec<&Keypair>,
    payer: &Pubkey,
    client: &mut BanksClient,
) -> Result<Transaction, Error> {
    let recent_blockhash = client
        .get_latest_blockhash()
        .await
        .map_err(Error::BanksClient)?;

    let tx = &mut Transaction::new_with_payer(&ixs, Some(payer));
    tx.partial_sign(&signers, recent_blockhash);

    Ok(tx.clone())
}
