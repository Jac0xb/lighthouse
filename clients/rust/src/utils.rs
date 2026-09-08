use base64::{engine::general_purpose::STANDARD, Engine};
use borsh::BorshDeserialize;
use solana_instruction::Instruction;
use solana_signer::{Signer, SignerError};
use solana_transaction::{versioned::VersionedTransaction, Transaction};

use crate::{
    transaction::{append_instructions, decompile_instructions, TransactionError},
    types::AssertionResult,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    Transaction(#[from] TransactionError),
    #[error("Cannot sign transaction: {0}")]
    SignerError(#[from] SignerError),
    #[error("Cannot decode log: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Cannot read assertion result: {0}")]
    IOError(#[from] std::io::Error),
}

pub fn parse_evaluation_payloads_from_logs(
    logs: Vec<&String>,
) -> Result<Vec<AssertionResult>, ClientError> {
    logs.into_iter()
        .filter_map(|log| log.split_once("Program data: ").map(|(_, data)| data))
        .map(|encoded| {
            let decoded = STANDARD.decode(encoded)?;
            Ok(AssertionResult::try_from_slice(&decoded)?)
        })
        .collect()
}

pub fn append_instructions_to_transaction(
    transaction: &Transaction,
    ixs: Vec<Instruction>,
) -> Result<Transaction, ClientError> {
    let appended = append_instructions(&transaction.clone().into(), &ixs)?;
    appended.into_legacy_transaction().ok_or_else(|| {
        TransactionError::InvalidTransaction("Expected a legacy message".into()).into()
    })
}

pub fn append_instructions_to_versioned_transaction(
    transaction: &VersionedTransaction,
    ixs: Vec<Instruction>,
    signers: &[&dyn Signer],
) -> Result<VersionedTransaction, ClientError> {
    let appended = append_instructions(transaction, &ixs)?;
    Ok(VersionedTransaction::try_new(appended.message, signers)?)
}

pub fn decompile_instruction_from_versioned_transaction(
    transaction: &VersionedTransaction,
) -> Result<Vec<Instruction>, ClientError> {
    Ok(decompile_instructions(transaction)?)
}

pub fn decompile_instruction_from_transaction(
    transaction: &Transaction,
) -> Result<Vec<Instruction>, ClientError> {
    Ok(decompile_instructions(&transaction.clone().into())?)
}
