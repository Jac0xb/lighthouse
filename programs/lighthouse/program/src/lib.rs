pub mod error;
pub mod instruction;
pub mod processor;
pub mod types;
pub mod utils;
pub mod validation;

#[cfg(test)]
pub mod test_utils;

use solana_program::declare_id;
pub use utils::Result;

declare_id!("L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK");

pub mod lighthouse {
    use crate::processor;
    use crate::processor::*;
    use crate::{err, error::LighthouseError, instruction::LighthouseInstruction};
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        msg,
        program::invoke,
        pubkey::Pubkey,
    };

    #[cfg(not(feature = "no-entrypoint"))]
    solana_program::entrypoint!(process_instruction);
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let mut remaining_instruction_data = instruction_data;
        let instruction = LighthouseInstruction::deserialize(&mut remaining_instruction_data)
            .or(Err(LighthouseError::InvalidInstructionData))?;

        // TODO: printing the instruction name is 1000's Compute Units, lets think about that.
        // msg!("Lighthouse instruction: {:?}", instruction);

        match instruction {
            LighthouseInstruction::MemoryWrite {
                memory_index,
                memory_account_bump,
                memory_offset,
                write_type,
            } => {
                let context = MemoryWriteContext::load(
                    &mut accounts.iter(),
                    memory_index,
                    memory_offset,
                    memory_account_bump,
                    &write_type,
                )?;
                processor::memory_write(&context, memory_offset, &write_type)?;
            }
            LighthouseInstruction::MemoryClose {
                memory_index,
                memory_account_bump,
            } => {
                let context = MemoryCloseContext::load(
                    &mut accounts.iter(),
                    memory_index,
                    memory_account_bump,
                )?;
                processor::memory_close(&context)?;
            }
            LighthouseInstruction::AssertAccountData {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertAccountDataDelta {
                log_level,
                assertion,
            } => {
                let context = AssertAccountDeltaContext::load(&mut accounts.iter())?;
                processor::assert_account_delta(&context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertAccountInfo {
                assertion,
                log_level,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertMintAccount {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertMintAccountMulti {
                log_level,
                assertions,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;

                for assertion in assertions.iter() {
                    invoke(
                        &Instruction {
                            program_id: crate::ID,
                            accounts: vec![AccountMeta::new_readonly(
                                *context.target_account.key,
                                false,
                            )],
                            data: LighthouseInstruction::AssertMintAccount {
                                log_level: log_level.clone(),
                                assertion: assertion.clone(),
                            }
                            .try_to_vec()
                            .map_err(|e| {
                                msg!("Failed to serialize assertion: {:?}", e);
                                err!(LighthouseError::FailedToSerialize)
                            })?,
                        },
                        accounts,
                    )?;
                }
            }
            LighthouseInstruction::AssertTokenAccount {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;

                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertTokenAccountMulti {
                log_level,
                assertions,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;

                for assertion in assertions.iter() {
                    invoke(
                        &Instruction {
                            program_id: crate::ID,
                            accounts: vec![AccountMeta::new_readonly(
                                *context.target_account.key,
                                false,
                            )],
                            data: LighthouseInstruction::AssertTokenAccount {
                                log_level: log_level.clone(),
                                assertion: assertion.clone(),
                            }
                            .try_to_vec()
                            .map_err(|e| {
                                msg!("Failed to serialize assertion: {:?}", e);
                                err!(LighthouseError::FailedToSerialize)
                            })?,
                        },
                        accounts,
                    )?;
                }
            }
            LighthouseInstruction::AssertStakeAccount {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertUpgradeableLoaderAccount {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertSysvarClock {
                log_level,
                assertion,
            } => {
                processor::assert_clock(&assertion, log_level)?;
            }
            LighthouseInstruction::AssertMerkleTreeAccount {
                log_level,
                assertion,
            } => {
                let context = AssertMerkleTreeAccountContext::load(&mut accounts.iter())?;
                processor::assert_merkle_tree_account(&context, &assertion, log_level)?;
            }
        }

        Ok(())
    }
}
