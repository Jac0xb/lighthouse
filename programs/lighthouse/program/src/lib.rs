#![allow(clippy::result_large_err)]
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
    use borsh::BorshDeserialize;
    use solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
    };

    use crate::{
        instruction::LighthouseInstruction,
        processor::{
            AssertContext, AssertMultiContext, CreateMemoryAccountContext,
            CreateMemoryAccountParameters, WriteContext, WriteParameters,
        },
        types::{Assertion, AssertionConfigV1},
    };

    use self::error::LighthouseError;
    use super::*;

    #[cfg(not(feature = "no-entrypoint"))]
    solana_program::entrypoint!(process_instruction);
    pub(crate) fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let (tag, data) = instruction_data
            .split_first()
            .ok_or(LighthouseError::InvalidInstructionData)?;

        let instruction = LighthouseInstruction::try_from(*tag)
            .or(Err(LighthouseError::InvalidInstructionData))?;

        msg!("Lighthouse instruction: {:?}", instruction);

        match instruction {
            LighthouseInstruction::Assert => {
                let context = AssertContext::load(&mut accounts.iter())?;
                let assertion = Assertion::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::v1::assert(
                    context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: true }),
                )?;
            }
            LighthouseInstruction::MultiAssert => {
                let context = AssertMultiContext::load(&mut accounts.iter())?;
                let assertions: Vec<Assertion> = Vec::<Assertion>::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::v1::assert_multi(
                    context,
                    &assertions,
                    Some(AssertionConfigV1 { verbose: true }),
                )?;
            }
            LighthouseInstruction::CreateMemoryAccount => {
                let parameters = CreateMemoryAccountParameters::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;
                let (context, bump_map) =
                    CreateMemoryAccountContext::load(&mut accounts.iter(), &parameters)?;

                processor::v1::create_memory_account(context, parameters, bump_map)?;
            }
            LighthouseInstruction::Write => {
                let parameters = WriteParameters::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                let context = WriteContext::load(&mut accounts.iter(), &parameters)?;

                processor::v1::write(context, parameters)?;
            }
        }

        Ok(())
    }
}
