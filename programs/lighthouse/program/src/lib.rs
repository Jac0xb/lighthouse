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
            msg!("Instruction: {}", instruction.get_name());
        }

        match instruction {
            LighthouseInstruction::MemoryWrite {
                memory_id,
                memory_bump,
                write_offset,
                write_type,
            } => {
                let ctx = MemoryWriteContext::load(
                    &mut accounts.iter(),
                    memory_id,
                    write_offset,
                    memory_bump,
                    &write_type,
                )?;
                processor::memory_write(&ctx, write_offset, &write_type)?;
            }
            LighthouseInstruction::MemoryClose {
                memory_id,
                memory_bump,
            } => {
                let ctx = MemoryCloseContext::load(&mut accounts.iter(), memory_id, memory_bump)?;
                processor::memory_close(&ctx)?;
            }
            LighthouseInstruction::AssertAccountData {
                log_level,
                assertion,
            } => {
                let ctx = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(ctx, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertAccountDelta {
                log_level,
                assertion,
            } => {
                let ctx = AssertAccountDeltaContext::load(&mut accounts.iter())?;
                processor::assert_account_delta(&ctx, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertAccountInfo {
                assertion,
                log_level,
            } => {
                let ctx = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account(ctx, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertAccountInfoMulti {
                log_level,
                assertions,
            } => {
                let ctx = AssertTargetAccountContext::load(&mut accounts.iter())?;
                processor::assert_target_account_multi(ctx, &assertions, log_level)?;
            }
            LighthouseInstruction::AssertMintAccount {
                log_level,
                assertion,
            } => {
                let ctx = AssertMintAccountContext::load(&mut accounts.iter())?;
                processor::assert_mint_account(ctx, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertMintAccountMulti {
                log_level,
                assertions,
            } => {
                let ctx = AssertMintAccountContext::load(&mut accounts.iter())?;
                processor::assert_mint_account_multi(ctx, &assertions, log_level)?;
            }
            LighthouseInstruction::AssertTokenAccount {
                log_level,
                assertion,
            } => {
                let ctx = AssertTokenAccountContext::load(&mut accounts.iter())?;
                processor::assert_token_account(ctx, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertTokenAccountMulti {
                log_level,
                assertions,
            } => {
                let ctx = AssertTokenAccountContext::load(&mut accounts.iter())?;
                processor::assert_token_account_multi(ctx, &assertions, log_level)?;
            }
            LighthouseInstruction::AssertStakeAccount {
                log_level,
                assertion,
            } => {
                let ctx = AssertStakeAccountContext::load(&mut accounts.iter())?;
                processor::assert_stake_account(ctx, assertion, log_level)?;
            }
            LighthouseInstruction::AssertStakeAccountMulti {
                log_level,
                assertions,
            } => {
                let ctx = AssertStakeAccountContext::load(&mut accounts.iter())?;
                processor::assert_stake_account_multi(ctx, &assertions, log_level)?
            }
            LighthouseInstruction::AssertUpgradeableLoaderAccount {
                log_level,
                assertion,
            } => {
                let ctx = AssertUpgradeableLoaderStateContext::load(&mut accounts.iter())?;
                processor::assert_upgradeable_loader_state(ctx, &assertion, log_level)?;
            }
            LighthouseInstruction::AssertUpgradeableLoaderAccountMulti {
                log_level,
                assertions,
            } => {
                let ctx = AssertUpgradeableLoaderStateContext::load(&mut accounts.iter())?;
                processor::assert_upgradeable_loader_state_multi(ctx, &assertions, log_level)?;
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
                let ctx = AssertMerkleTreeAccountContext::load(&mut accounts.iter())?;
                processor::assert_merkle_tree_account(&ctx, &assertion, log_level)?;
            }
        }

        Ok(())
    }
}
