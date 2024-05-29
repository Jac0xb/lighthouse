#[allow(unused)]
#[allow(clippy::identity_op)]
mod generated;
mod hooked;

pub use generated::programs::LIGHTHOUSE_ID;
pub use generated::programs::LIGHTHOUSE_ID as ID;
use solana_program::pubkey::{Pubkey, PubkeyError};

pub use generated::types;
pub use lighthouse_common::{CompactU64, LEB128Vec};

pub mod instructions {
    pub use crate::generated::instructions::{
        AssertAccountDataBuilder, AssertAccountDeltaBuilder, AssertAccountInfoBuilder,
        AssertBubblegumTreeConfigAccountBuilder, AssertMerkleTreeAccountBuilder,
        AssertMintAccountBuilder, AssertMintAccountMultiBuilder, AssertStakeAccountBuilder,
        AssertStakeAccountMultiBuilder, AssertSysvarClockBuilder, AssertTokenAccountBuilder,
        AssertTokenAccountMultiBuilder, AssertUpgradeableLoaderAccountBuilder,
        AssertUpgradeableLoaderAccountMultiBuilder, MemoryCloseBuilder, MemoryWriteBuilder,
    };
}

#[cfg(feature = "cpi")]
pub mod cpi {
    pub use crate::generated::instructions::{
        AssertAccountDataBuilder, AssertAccountDeltaBuilder, AssertAccountInfoBuilder,
        AssertAccountInfoMultiBuilder, AssertBubblegumTreeConfigAccountCpiBuilder,
        AssertMerkleTreeAccountBuilder, AssertMintAccountBuilder, AssertMintAccountMultiBuilder,
        AssertStakeAccountBuilder, AssertStakeAccountMultiBuilder, AssertSysvarClockBuilder,
        AssertTokenAccountBuilder, AssertTokenAccountMultiBuilder,
        AssertUpgradeableLoaderAccountBuilder, MemoryCloseCpiBuilder, MemoryWriteCpiBuilder,
    };
}

pub mod errors {
    pub use crate::generated::errors::*;
}

pub fn find_memory_pda(payer: Pubkey, memory_id: u8) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &["memory".to_string().as_ref(), payer.as_ref(), &[memory_id]],
        &crate::ID,
    )
}

pub fn find_memory_pda_bump_iterate(
    payer: Pubkey,
    memory_id: u8,
    bump_skip: u8,
    start_bump: Option<u8>,
) -> Option<(solana_program::pubkey::Pubkey, u8)> {
    let memory_ref = "memory".to_string();
    let seeds = [memory_ref.as_ref(), payer.as_ref(), &[memory_id]];

    let mut bump_seed = [start_bump.unwrap_or(std::u8::MAX)];
    let mut bump_skip = bump_skip as usize;

    for _ in 0..std::u8::MAX {
        let mut seeds_with_bump = seeds.to_vec();
        seeds_with_bump.push(&bump_seed);
        match Pubkey::create_program_address(&seeds_with_bump, &crate::ID) {
            Ok(address) => {
                if bump_skip == 0 {
                    return Some((address, bump_seed[0]));
                } else {
                    bump_skip -= 1;
                }
            }
            Err(PubkeyError::InvalidSeeds) => {}
            _ => break,
        }
        bump_seed[0] -= 1;

        println!("bump_seed: {:?}", bump_seed[0])
    }

    None
}

#[cfg(feature = "sdk")]
pub mod utils {
    use crate::generated::types::AssertionResult;
    use borsh::BorshDeserialize;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        message::{legacy, v0, CompileError, Message, VersionedMessage},
        signer::{Signer, SignerError},
        transaction::{Transaction, VersionedTransaction},
    };

    #[derive(Debug, thiserror::Error)]
    #[repr(u32)]
    pub enum ClientError {
        #[error("Transaction already signed")]
        TransactionAlreadySigned,
        #[error("Address table lookups not supported")]
        AddressTableLookupsNotSupported,
        #[error("Empty transaction")]
        EmptyTransaction,
        #[error("...")]
        CompileError(CompileError),
        #[error("...")]
        SignerError(SignerError),
        #[error("...")]
        Base64DecodeError(base64::DecodeError),
        #[error("...")]
        IOError(std::io::Error),
    }

    #[allow(deprecated)]
    pub fn parse_evaluation_payloads_from_logs(
        logs: Vec<&String>,
    ) -> Result<Vec<AssertionResult>, ClientError> {
        logs.iter()
            .filter(|log| log.contains("Program data: "))
            .map(|log| {
                let encoded = log.split("Program data: ").collect::<Vec<&str>>()[1];
                let decoded = base64::decode(encoded).map_err(ClientError::Base64DecodeError)?;
                AssertionResult::try_from_slice(&decoded).map_err(ClientError::IOError)
            })
            .collect()
    }

    pub fn append_instructions_to_transaction(
        transaction: &Transaction,
        ixs: Vec<Instruction>,
    ) -> Result<Transaction, ClientError> {
        if !transaction.signatures.is_empty() {
            return Err(ClientError::TransactionAlreadySigned);
        }

        let mut merged_ixs = decompile_instruction_from_transaction(transaction)?;
        merged_ixs.extend(ixs);

        let transaction = Transaction::new_unsigned(Message::new(
            &merged_ixs,
            Some(
                transaction
                    .message
                    .account_keys
                    .first()
                    .ok_or(ClientError::EmptyTransaction)?,
            ),
        ));

        Ok(transaction)
    }

    pub fn append_instructions_to_versioned_transaction(
        transaction: &VersionedTransaction,
        ixs: Vec<Instruction>,
        signers: &[&dyn Signer],
    ) -> Result<VersionedTransaction, ClientError> {
        if !transaction.signatures.is_empty() {
            return Err(ClientError::TransactionAlreadySigned);
        }

        if transaction.message.address_table_lookups().is_some() {
            return Err(ClientError::AddressTableLookupsNotSupported);
        }

        let mut merged_ixs = decompile_instruction_from_versioned_transaction(transaction)?;
        merged_ixs.extend(ixs);

        let payer = transaction
            .message
            .static_account_keys()
            .first()
            .ok_or(ClientError::EmptyTransaction)?;

        let versioned_messsage = match transaction.message {
            VersionedMessage::Legacy(_) => {
                VersionedMessage::Legacy(legacy::Message::new(&merged_ixs, Some(payer)))
            }
            VersionedMessage::V0(_) => VersionedMessage::V0(
                v0::Message::try_compile(
                    payer,
                    &merged_ixs,
                    &[],
                    *transaction.message.recent_blockhash(),
                )
                .map_err(ClientError::CompileError)?,
            ),
        };

        let mut transaction = VersionedTransaction::try_new(versioned_messsage, signers)
            .map_err(ClientError::SignerError)?;

        transaction
            .message
            .set_recent_blockhash(*transaction.message.recent_blockhash());

        Ok(transaction)
    }

    pub fn decompile_instruction_from_versioned_transaction(
        transaction: &VersionedTransaction,
    ) -> Result<Vec<Instruction>, ClientError> {
        if !transaction.signatures.is_empty() {
            return Err(ClientError::TransactionAlreadySigned);
        }

        if transaction.message.address_table_lookups().is_some() {
            return Err(ClientError::AddressTableLookupsNotSupported);
        }

        let mut modified_ixs = vec![];
        let compiled_ixs = transaction.message.instructions();

        for instruction in compiled_ixs {
            let account_keys = transaction.message.static_account_keys();

            modified_ixs.push(Instruction {
                program_id: account_keys[instruction.program_id_index as usize],
                accounts: instruction
                    .accounts
                    .iter()
                    .map(|index| AccountMeta {
                        pubkey: account_keys[*index as usize],
                        is_signer: index < &transaction.message.header().num_required_signatures,
                        is_writable: transaction.message.is_maybe_writable(*index as usize),
                    })
                    .collect(),
                data: instruction.data.clone(),
            });
        }

        Ok(modified_ixs)
    }

    pub fn decompile_instruction_from_transaction(
        transaction: &Transaction,
    ) -> Result<Vec<Instruction>, ClientError> {
        if !transaction.signatures.is_empty() {
            return Err(ClientError::TransactionAlreadySigned);
        }

        let mut modified_ixs = vec![];

        for instruction in &transaction.message.instructions {
            modified_ixs.push(Instruction {
                program_id: transaction.message.account_keys[instruction.program_id_index as usize],
                accounts: instruction
                    .accounts
                    .iter()
                    .map(|index| AccountMeta {
                        pubkey: transaction.message.account_keys[*index as usize],
                        is_signer: index < &transaction.message.header.num_required_signatures,
                        is_writable: transaction.message.is_writable(*index as usize),
                    })
                    .collect(),
                data: instruction.data.clone(),
            });
        }

        Ok(modified_ixs)
    }
}
