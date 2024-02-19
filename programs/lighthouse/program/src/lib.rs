#![allow(clippy::too_many_arguments)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod types;
pub mod utils;
pub mod validations;

pub use crate::instruction::LighthouseInstruction;
use solana_program::declare_id;

declare_id!("L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK");

pub mod lighthouse {
    use self::error::LighthouseError;
    use super::*;
    use crate::{
        instruction::LighthouseInstruction,
        processor::{AssertWithTargetContext, CreateMemoryAccountContext, WriteContext},
        types::AssertionConfigV1,
    };
    use borsh::BorshDeserialize;
    use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

    #[cfg(not(feature = "no-entrypoint"))]
    solana_program::entrypoint!(process_instruction);
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // let (tag, data) = instruction_data
        //     .split_first()
        //     .ok_or(LighthouseError::InvalidInstructionData)?;

        let instruction = LighthouseInstruction::try_from_slice(instruction_data)
            .or(Err(LighthouseError::InvalidInstructionData))?;

        // TODO: printing the instruction name is 500 Compute Units.
        // msg!("Lighthouse instruction: {:?}", instruction);

        match instruction {
            LighthouseInstruction::CreateMemoryAccount(parameters) => {
                let (context, bump_map) =
                    CreateMemoryAccountContext::load(&mut accounts.iter(), &parameters)?;

                processor::create_memory_account(context, parameters, bump_map)?;
            }
            LighthouseInstruction::Write(parameters) => {
                let context = WriteContext::load(&mut accounts.iter(), &parameters)?;

                processor::write(context, parameters)?;
            }
            LighthouseInstruction::AssertAccountData(assertion) => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertAccountInfo(assertion) => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertDataHash(assertion) => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;

                // processor::assert(
                //     context,
                //     &Assertion::AccountDataHash(hash, operator, start, end),
                //     Some(AssertionConfigV1 { verbose: false }),
                // )?;
            }
            LighthouseInstruction::AssertMintAccountField(assertion) => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertTokenAccountField(assertion) => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertSysvarClockField(assertion) => {
                processor::assert(&assertion, Some(AssertionConfigV1 { verbose: false }))?;
            }
        }

        Ok(())
    }
}
