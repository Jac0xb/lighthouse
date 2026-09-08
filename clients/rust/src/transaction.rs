//! Transaction helpers for legacy, v0, and v1 with the current Solana crates.
//! Use `into_instruction` to convert an instruction from a Lighthouse builder.

use solana_instruction::{AccountMeta, Instruction};
use solana_message::{legacy, v0, v1, CompileError, VersionedMessage};
use solana_transaction::versioned::VersionedTransaction;

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Transaction already signed")]
    AlreadySigned,
    #[error("Address table lookups must be resolved before appending instructions")]
    AddressTableLookupsNotSupported,
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Transaction has {size} bytes; the limit is {limit}")]
    TooLarge { size: usize, limit: usize },
    #[error("Cannot compile transaction: {0}")]
    Compile(#[from] CompileError),
    #[error("Cannot encode or decode transaction: {0}")]
    WireEncoding(String),
}

/// Convert a Lighthouse instruction to the current Solana instruction type.
pub fn into_instruction(instruction: solana_program::instruction::Instruction) -> Instruction {
    Instruction {
        program_id: instruction.program_id.to_bytes().into(),
        accounts: instruction
            .accounts
            .into_iter()
            .map(|account| AccountMeta {
                pubkey: account.pubkey.to_bytes().into(),
                is_signer: account.is_signer,
                is_writable: account.is_writable,
            })
            .collect(),
        data: instruction.data,
    }
}

/// Append instructions before signing. Preserve the version, lifetime, and v1 config.
/// Simulate again to check the resource limits after adding assertions.
pub fn append_instructions(
    transaction: &VersionedTransaction,
    instructions: &[Instruction],
) -> Result<VersionedTransaction, TransactionError> {
    if transaction
        .signatures
        .iter()
        .any(|signature| *signature != Default::default())
    {
        return Err(TransactionError::AlreadySigned);
    }
    let mut merged = decompile_instructions(transaction)?;
    merged.extend_from_slice(instructions);
    let payer = transaction
        .message
        .static_account_keys()
        .first()
        .ok_or_else(|| TransactionError::InvalidTransaction("Missing fee payer".into()))?;
    let blockhash = transaction.message.recent_blockhash().clone();
    let message = match &transaction.message {
        VersionedMessage::Legacy(_) => VersionedMessage::Legacy(
            legacy::Message::new_with_blockhash(&merged, Some(payer), &blockhash),
        ),
        VersionedMessage::V0(_) => {
            VersionedMessage::V0(v0::Message::try_compile(payer, &merged, &[], blockhash)?)
        }
        VersionedMessage::V1(message) => VersionedMessage::V1(
            v1::Message::try_compile_with_config(payer, &merged, blockhash, message.config)?,
        ),
    };
    let transaction = VersionedTransaction {
        signatures: vec![Default::default(); usize::from(message.header().num_required_signatures)],
        message,
    };
    encode_transaction(&transaction)?;
    Ok(transaction)
}

/// Read instructions with inline accounts. Signed transactions can also be read.
pub fn decompile_instructions(
    transaction: &VersionedTransaction,
) -> Result<Vec<Instruction>, TransactionError> {
    transaction
        .message
        .sanitize()
        .map_err(|error| TransactionError::InvalidTransaction(error.to_string()))?;
    if transaction
        .message
        .address_table_lookups()
        .is_some_and(|tables| !tables.is_empty())
    {
        return Err(TransactionError::AddressTableLookupsNotSupported);
    }
    let keys = transaction.message.static_account_keys();
    let header = transaction.message.header();
    let signers = usize::from(header.num_required_signatures);
    let writable_signers = signers - usize::from(header.num_readonly_signed_accounts);
    let writable_end = keys.len() - usize::from(header.num_readonly_unsigned_accounts);
    Ok(transaction
        .message
        .instructions()
        .iter()
        .map(|instruction| Instruction {
            program_id: keys[usize::from(instruction.program_id_index)],
            accounts: instruction
                .accounts
                .iter()
                .map(|index| {
                    let index = usize::from(*index);
                    AccountMeta {
                        pubkey: keys[index],
                        is_signer: index < signers,
                        is_writable: if index < signers {
                            index < writable_signers
                        } else {
                            index < writable_end
                        },
                    }
                })
                .collect(),
            data: instruction.data.clone(),
        })
        .collect())
}

/// Encode the native wire format, including the v1 signature tail.
/// Use base64 when passing these bytes to an RPC method.
pub fn encode_transaction(transaction: &VersionedTransaction) -> Result<Vec<u8>, TransactionError> {
    transaction
        .sanitize()
        .map_err(|error| TransactionError::InvalidTransaction(error.to_string()))?;
    let bytes = wincode::serialize(transaction)
        .map_err(|error| TransactionError::WireEncoding(error.to_string()))?;
    check_size(&bytes, &transaction.message)?;
    Ok(bytes)
}

/// Decode and validate a complete legacy, v0, or v1 wire transaction.
pub fn decode_transaction(bytes: &[u8]) -> Result<VersionedTransaction, TransactionError> {
    if bytes.len() > v1::MAX_TRANSACTION_SIZE {
        return Err(TransactionError::TooLarge {
            size: bytes.len(),
            limit: v1::MAX_TRANSACTION_SIZE,
        });
    }
    let transaction: VersionedTransaction = wincode::deserialize(bytes)
        .map_err(|error| TransactionError::WireEncoding(error.to_string()))?;
    let encoded = encode_transaction(&transaction)?;
    if encoded != bytes {
        return Err(TransactionError::InvalidTransaction(
            "Trailing data or noncanonical encoding".into(),
        ));
    }
    Ok(transaction)
}

fn check_size(bytes: &[u8], message: &VersionedMessage) -> Result<(), TransactionError> {
    let limit = match message {
        VersionedMessage::V1(_) => v1::MAX_TRANSACTION_SIZE,
        VersionedMessage::Legacy(_) | VersionedMessage::V0(_) => 1232,
    };
    if bytes.len() > limit {
        return Err(TransactionError::TooLarge {
            size: bytes.len(),
            limit,
        });
    }
    Ok(())
}
