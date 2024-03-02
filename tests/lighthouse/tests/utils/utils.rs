use super::context::TestContext;
use super::error::Error;
use lighthouse_client::errors::LighthouseError;
use solana_banks_interface::BanksTransactionResultWithMetadata;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::instruction::InstructionError;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::AccountSharedData;
use solana_sdk::transaction::{Transaction, TransactionError};

pub async fn process_transaction(
    context: &mut TestContext,
    tx: &Transaction,
) -> Result<BanksTransactionResultWithMetadata, Error> {
    let result: solana_banks_interface::BanksTransactionResultWithMetadata = context
        .client()
        .process_transaction_with_metadata(tx.clone())
        .await
        .unwrap();

    Ok(result)
}

pub async fn process_transaction_assert_success(
    context: &mut TestContext,
    tx: Transaction,
) -> Result<BanksTransactionResultWithMetadata, Error> {
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
        return Err(Error::TransactionFailed(format!(
            "Tx Result {:?}",
            tx_metadata.result.clone().err()
        )));
    }

    Ok(tx_metadata)
}

pub async fn process_transaction_assert_failure(
    context: &mut TestContext,
    tx: Transaction,
    expected_tx_error: TransactionError,
    log_match_regex: Option<&[String]>,
) -> Result<(), Error> {
    let tx_metadata = process_transaction(context, &tx).await.unwrap();

    let logs = tx_metadata.metadata.clone().unwrap().log_messages;
    for log in logs {
        println!("{:?}", log);
    }

    if tx_metadata.result.is_ok() {
        return Err(Error::TransactionExpectedFailure(
            "Transaction was expected to fail".to_string(),
        ));
    }

    let actual_tx_error = tx_metadata.result.unwrap_err();

    if actual_tx_error != expected_tx_error {
        match &actual_tx_error {
            TransactionError::InstructionError(ix_index, program_error) => {
                match &expected_tx_error {
                    TransactionError::InstructionError(
                        expected_ix_index,
                        expected_program_error,
                    ) => {
                        if ix_index != expected_ix_index || program_error != expected_program_error
                        {
                            return Err(Error::TransactionExpectedFailure(format!(
                                "Expected error code: {:?}, got: {:?}",
                                expected_tx_error, &actual_tx_error
                            )));
                        }
                    }
                    _ => {
                        return Err(Error::TransactionExpectedFailure(format!(
                            "Expected error code: {:?}, got: {:?}",
                            expected_tx_error, actual_tx_error
                        )));
                    }
                }

                return Err(Error::TransactionExpectedFailure(format!(
                    "Expected error code: {:?}, got: {:?}",
                    expected_tx_error, program_error
                )));
            }
            _ => {
                return Err(Error::TransactionExpectedFailure(format!(
                    "Expected error code: {:?}, got: {:?}",
                    expected_tx_error, actual_tx_error
                )));
            }
        }
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
                return Err(Error::LogNotFound(format!("Log not found: {}", regex)));
            }
        }
    }

    Ok(())
}

pub fn to_transaction_error(ix_index: u8, program_error: LighthouseError) -> TransactionError {
    TransactionError::InstructionError(
        ix_index,
        InstructionError::Custom(6000 + program_error as u32),
    )
}

pub fn to_transaction_error_u8(ix_index: u8, program_error: u32) -> TransactionError {
    TransactionError::InstructionError(ix_index, InstructionError::Custom(program_error))
}

pub async fn set_account_from_rpc(
    context: &mut TestContext,
    connection: &RpcClient,
    account_pubkey: &Pubkey,
) {
    let account = connection.get_account(account_pubkey).await.unwrap();

    let mut shared_account =
        AccountSharedData::new(account.lamports, account.data.len(), &account.owner);
    shared_account.set_data_from_slice(account.data.as_slice());

    context
        .program_context
        .set_account(account_pubkey, &shared_account);
}
