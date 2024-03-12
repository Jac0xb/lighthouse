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
    use crate::types::assert::LogLevel;
    use crate::{error::LighthouseError, instruction::LighthouseInstruction};
    use borsh::BorshDeserialize;
    use solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
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

        if instruction.get_log_level() == LogLevel::PlaintextMessage {
            msg!("Instruction: {:?}", instruction.get_name());
        }

        match instruction {
            LighthouseInstruction::MemoryWrite {
                memory_id,
                memory_bump,
                write_offset,
                write_type,
            } => {
                let context = MemoryWriteContext::load(
                    &mut accounts.iter(),
                    memory_id,
                    write_offset,
                    memory_bump,
                    &write_type,
                )?;
                processor::memory_write(&context, write_offset, &write_type)?;
            }
            LighthouseInstruction::MemoryClose {
                memory_id,
                memory_bump,
            } => {
                let context =
                    MemoryCloseContext::load(&mut accounts.iter(), memory_id, memory_bump)?;
                processor::memory_close(&context)?;
            }
            LighthouseInstruction::AssertAccountData {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertAccountDelta {
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
            LighthouseInstruction::AssertAccountInfoMulti {
                log_level,
                assertions,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;

                for (i, assertion) in assertions.iter().enumerate() {
                    processor::assert_target_account(context.clone(), assertion, log_level)
                        .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
                }
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

                for (i, assertion) in assertions.iter().enumerate() {
                    processor::assert_target_account(context.clone(), assertion, log_level)
                        .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
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

                for (i, assertion) in assertions.iter().enumerate() {
                    processor::assert_target_account(context.clone(), assertion, log_level)
                        .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
                }
            }
            LighthouseInstruction::AssertStakeAccount {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertStakeAccountMulti {
                log_level,
                assertions,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;

                for (i, assertion) in assertions.iter().enumerate() {
                    processor::assert_target_account(context.clone(), assertion, log_level)
                        .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
                }
            }
            LighthouseInstruction::AssertUpgradeableLoaderAccount {
                log_level,
                assertion,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(context, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertUpgradeableLoaderAccountMulti {
                log_level,
                assertions,
            } => {
                let context = AssertTargetAccountContext::load(&mut accounts.iter())?;

                for (i, assertion) in assertions.iter().enumerate() {
                    processor::assert_target_account(context.clone(), assertion, log_level)
                        .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
                }
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
