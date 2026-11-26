#[allow(unused)]
#[allow(clippy::identity_op)]
mod generated;
pub mod hooked;
pub mod registry;

pub use generated::programs::LIGHTHOUSE_ID;
pub use generated::programs::LIGHTHOUSE_ID as ID;
pub use generated::types;
pub use lighthouse_common::{CompactU64, LEB128Vec};
pub use solana_address::{error::AddressError, Address};

pub mod instructions {
    pub use crate::generated::instructions::{
        AssertAccountDataBuilder, AssertAccountDataMultiBuilder, AssertAccountDeltaBuilder,
        AssertAccountInfoBuilder, AssertAccountInfoMultiBuilder,
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
        AssertAccountDataCpiBuilder, AssertAccountDataMultiCpiBuilder,
        AssertAccountDeltaCpiBuilder, AssertAccountInfoCpiBuilder,
        AssertAccountInfoMultiCpiBuilder, AssertBubblegumTreeConfigAccountCpiBuilder,
        AssertMerkleTreeAccountCpiBuilder, AssertMintAccountCpiBuilder,
        AssertMintAccountMultiCpiBuilder, AssertStakeAccountCpiBuilder,
        AssertStakeAccountMultiCpiBuilder, AssertSysvarClockCpiBuilder,
        AssertTokenAccountCpiBuilder, AssertTokenAccountMultiCpiBuilder,
        AssertUpgradeableLoaderAccountCpiBuilder, AssertUpgradeableLoaderAccountMultiCpiBuilder,
        MemoryCloseCpiBuilder, MemoryWriteCpiBuilder,
    };
}

pub mod errors {
    pub use crate::generated::errors::*;
}

pub fn find_memory_pda(payer: Address, memory_id: u8) -> (Address, u8) {
    Address::find_program_address(&[b"memory", payer.as_ref(), &[memory_id]], &crate::ID)
}

pub fn find_memory_pda_bump_iterate(
    payer: Address,
    memory_id: u8,
    bump_skip: u8,
    start_bump: Option<u8>,
) -> Option<(Address, u8)> {
    let seeds = [b"memory".as_ref(), payer.as_ref(), &[memory_id]];

    let mut bump_seed = [start_bump.unwrap_or(u8::MAX)];
    let mut bump_skip = bump_skip as usize;

    loop {
        let mut seeds_with_bump = seeds.to_vec();
        seeds_with_bump.push(&bump_seed);
        match Address::create_program_address(&seeds_with_bump, &crate::ID) {
            Ok(address) => {
                if bump_skip == 0 {
                    return Some((address, bump_seed[0]));
                } else {
                    bump_skip -= 1;
                }
            }
            Err(AddressError::InvalidSeeds) => {}
            _ => break,
        }
        if bump_seed[0] == 0 {
            break;
        }
        bump_seed[0] -= 1;
    }

    None
}

#[cfg(feature = "sdk")]
pub mod utils {
    use crate::generated::types::AssertionResult;
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use borsh::BorshDeserialize;
    use solana_instruction::{AccountMeta, Instruction};
    use solana_message::{legacy, v0, CompileError, Message, VersionedMessage};
    use solana_signer::{Signer, SignerError};
    use solana_transaction::{versioned::VersionedTransaction, Transaction};

    #[derive(Debug, thiserror::Error)]
    #[repr(u32)]
    pub enum ClientError {
        #[error("Transaction already signed")]
        TransactionAlreadySigned,
        #[error("Address table lookups not supported")]
        AddressTableLookupsNotSupported,
        #[error("Empty transaction")]
        EmptyTransaction,
        #[error("Compile error: {0}")]
        CompileError(CompileError),
        #[error("Signer error: {0}")]
        SignerError(SignerError),
        #[error("Base64 decode error: {0}")]
        Base64DecodeError(base64::DecodeError),
        #[error("IO error: {0}")]
        IOError(std::io::Error),
    }

    pub fn parse_evaluation_payloads_from_logs(
        logs: Vec<&String>,
    ) -> Result<Vec<AssertionResult>, ClientError> {
        logs.iter()
            .filter(|log| log.contains("Program data: "))
            .map(|log| {
                let encoded = log.split("Program data: ").collect::<Vec<&str>>()[1];
                let decoded = STANDARD
                    .decode(encoded)
                    .map_err(ClientError::Base64DecodeError)?;
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

        let recent_blockhash = transaction.message.recent_blockhash().clone();

        let versioned_message = match transaction.message {
            VersionedMessage::Legacy(_) => {
                VersionedMessage::Legacy(legacy::Message::new(&merged_ixs, Some(payer)))
            }
            VersionedMessage::V0(_) => VersionedMessage::V0(
                v0::Message::try_compile(payer, &merged_ixs, &[], recent_blockhash.clone())
                    .map_err(ClientError::CompileError)?,
            ),
        };

        let mut transaction = VersionedTransaction::try_new(versioned_message, signers)
            .map_err(ClientError::SignerError)?;

        transaction.message.set_recent_blockhash(recent_blockhash);

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
                        is_writable: transaction.message.is_maybe_writable(*index as usize, None),
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
                        is_writable: transaction.message.is_maybe_writable(*index as usize, None),
                    })
                    .collect(),
                data: instruction.data.clone(),
            });
        }

        Ok(modified_ixs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::{
        AssertAccountDataBuilder, AssertAccountDataMultiBuilder, AssertAccountInfoBuilder,
        AssertSysvarClockBuilder, MemoryWriteBuilder,
    };
    use crate::registry::{AssertVoteAccountBuilder, VoteAccountAssertion};
    use crate::types::{
        AccountInfoAssertion, DataValueAssertion, EquatableOperator, IntegerOperator, LogLevel,
        SysvarClockAssertion, WriteType,
    };
    use borsh::BorshDeserialize;

    #[test]
    fn test_find_memory_pda() {
        let payer = Address::new_from_array([1u8; 32]);
        let (pda, bump) = find_memory_pda(payer, 0);
        let derived = Address::create_program_address(
            &[b"memory", payer.as_ref(), &[0u8], &[bump]],
            &LIGHTHOUSE_ID,
        )
        .unwrap();
        assert_eq!(pda, derived);

        let iterated = find_memory_pda_bump_iterate(payer, 0, 0, None).unwrap();
        assert_eq!(iterated, (pda, bump));
    }

    #[test]
    fn test_instruction_builders_and_wire_format() {
        let target = Address::new_from_array([2u8; 32]);

        let ix = AssertAccountInfoBuilder::new()
            .target_account(target)
            .assertion(AccountInfoAssertion::Lamports {
                value: 1_000_000,
                operator: IntegerOperator::GreaterThanOrEqual,
            })
            .instruction();
        assert_eq!(ix.program_id, LIGHTHOUSE_ID);
        assert_eq!(ix.accounts.len(), 1);
        assert_eq!(ix.accounts[0].pubkey, target);
        // discriminator 5 (AssertAccountInfo), log_level 0 (Silent), variant 0 (Lamports)
        assert_eq!(ix.data[0], 5);
        assert_eq!(ix.data[1], 0);

        let data_ix = AssertAccountDataBuilder::new()
            .target_account(target)
            .offset(CompactU64(64))
            .assertion(DataValueAssertion::U64 {
                value: 42,
                operator: IntegerOperator::Equal,
            })
            .instruction();
        assert_eq!(data_ix.data[0], 2);

        let multi_ix = AssertAccountDataMultiBuilder::new()
            .target_account(target)
            .assertions(
                vec![hooked::AccountDataAssertion {
                    offset: CompactU64(0),
                    assertion: DataValueAssertion::U8 {
                        value: 1,
                        operator: IntegerOperator::Equal,
                    },
                }]
                .into(),
            )
            .instruction();
        assert_eq!(multi_ix.data[0], 3);

        let clock_ix = AssertSysvarClockBuilder::new()
            .log_level(LogLevel::PlaintextMessage)
            .assertion(SysvarClockAssertion::Slot {
                value: 100,
                operator: IntegerOperator::GreaterThan,
            })
            .instruction();
        assert_eq!(clock_ix.accounts.len(), 0);
        assert_eq!(clock_ix.data[0], 15);

        let (memory, bump) = find_memory_pda(target, 0);
        let write_ix = MemoryWriteBuilder::new()
            .program_id(LIGHTHOUSE_ID)
            .payer(target)
            .memory(memory)
            .source_account(target)
            .memory_bump(bump)
            .write_offset(CompactU64(0))
            .write_type(WriteType::AccountData {
                offset: 0,
                data_length: 8,
            })
            .instruction();
        assert_eq!(write_ix.data[0], 0);
        assert_eq!(write_ix.accounts.len(), 5);

        let vote_ix = AssertVoteAccountBuilder::new()
            .target_account(target)
            .assertion(VoteAccountAssertion::AuthorizedWithdrawer {
                value: target,
                operator: EquatableOperator::Equal,
            })
            .instruction();
        assert_eq!(vote_ix.data[0], 3);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip() {
        let addr = Address::new_from_array([7u8; 32]);
        let assertion = DataValueAssertion::Pubkey {
            value: addr,
            operator: EquatableOperator::Equal,
        };
        let json = serde_json::to_string(&assertion).unwrap();
        let decoded: DataValueAssertion = serde_json::from_str(&json).unwrap();
        assert_eq!(assertion, decoded);
    }

    #[cfg(feature = "sdk")]
    #[test]
    fn test_parse_evaluation_payloads_from_logs() {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let expected = types::AssertionResult::U64(Some(100), Some(100), 0, true);
        let bytes = borsh::to_vec(&expected).unwrap();
        let log_line = format!("Program data: {}", STANDARD.encode(&bytes));
        let other_line = "Program log: hello".to_string();
        let parsed =
            utils::parse_evaluation_payloads_from_logs(vec![&other_line, &log_line]).unwrap();
        assert_eq!(parsed, vec![expected]);
        // Also verify round-trip borsh deserialization directly
        assert_eq!(
            types::AssertionResult::try_from_slice(&bytes).unwrap(),
            parsed[0]
        );
    }
}
