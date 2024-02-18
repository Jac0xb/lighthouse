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
        processor::{
            AssertWithTargetContext, CreateMemoryAccountContext, CreateMemoryAccountParameters,
            WriteContext, WriteParameters,
        },
        types::{
            AccountDataAssertionTuple, AccountDataHashAssertionTuple, AccountInfoFieldAssertion,
            Assertion, AssertionConfigV1, MintAccountFieldAssertion, SysvarClockFieldAssertion,
            TokenAccountFieldAssertion,
        },
    };
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
        let (tag, data) = instruction_data
            .split_first()
            .ok_or(LighthouseError::InvalidInstructionData)?;

        let instruction = LighthouseInstruction::try_from(*tag)
            .or(Err(LighthouseError::InvalidInstructionData))?;

        // TODO: printing the instruction name is 500 Compute Units.
        msg!("Lighthouse instruction: {:?}", instruction);

        match instruction {
            LighthouseInstruction::AssertAccountData => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;
                let (offset, assertion) = AccountDataAssertionTuple::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::assert(
                    context,
                    &Assertion::AccountData(offset, assertion),
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertAccountInfo => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;
                let field = AccountInfoFieldAssertion::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::assert(
                    context,
                    &Assertion::AccountInfoField(field),
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertDataHash => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;
                let (hash, operator, start, end) =
                    AccountDataHashAssertionTuple::try_from_slice(data)
                        .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::assert(
                    context,
                    &Assertion::AccountDataHash(hash, operator, start, end),
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertMintAccountField => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;
                let field_assertion = MintAccountFieldAssertion::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::assert(
                    context,
                    &Assertion::MintAccountField(field_assertion),
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertTokenAccountField => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;
                let field = TokenAccountFieldAssertion::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::assert(
                    context,
                    &Assertion::TokenAccountField(field),
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertSysvarClockField => {
                let context = AssertWithTargetContext::load(&mut accounts.iter())?;
                let field = SysvarClockFieldAssertion::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                processor::assert(
                    context,
                    &Assertion::SysvarClockField(field),
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::CreateMemoryAccount => {
                let parameters = CreateMemoryAccountParameters::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;
                let (context, bump_map) =
                    CreateMemoryAccountContext::load(&mut accounts.iter(), &parameters)?;

                processor::create_memory_account(context, parameters, bump_map)?;
            }
            LighthouseInstruction::Write => {
                let parameters = WriteParameters::try_from_slice(data)
                    .or(Err(LighthouseError::InvalidInstructionData))?;

                let context = WriteContext::load(&mut accounts.iter(), &parameters)?;

                processor::write(context, parameters)?;
            }
        }

        Ok(())
    }
}
