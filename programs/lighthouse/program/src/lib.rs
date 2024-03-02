pub mod constants;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod types;
pub mod utils;
pub mod validations;

#[cfg(test)]
pub mod test_utils;

use solana_program::declare_id;

declare_id!("L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK");

pub mod lighthouse {
    use self::{
        error::LighthouseError,
        processor::{
            AssertWithAccountContext, AssertWithAccountsContext, CreateMemoryAccountContext,
            WriteContext,
        },
    };
    use super::*;
    use crate::{instruction::LighthouseInstruction, types::AssertionConfigV1};
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
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    &context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertAccountDataDiff(assertion) => {
                let context = AssertWithAccountsContext::load(&mut accounts.iter())?;

                processor::assert_with_accounts(
                    &context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertAccountInfo(assertion) => {
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    &context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertMintAccount(assertion) => {
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    &context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertMintAccountMulti(assertions) => {
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                for assertion in assertions.iter() {
                    invoke(
                        &Instruction {
                            program_id: crate::ID,
                            accounts: vec![AccountMeta::new_readonly(
                                *context.target_account.key,
                                false,
                            )],
                            data: LighthouseInstruction::AssertMintAccount(assertion.clone())
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
            LighthouseInstruction::AssertTokenAccount(assertion) => {
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    &context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertTokenAccountMulti(assertions) => {
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                for assertion in assertions.iter() {
                    invoke(
                        &Instruction {
                            program_id: crate::ID,
                            accounts: vec![AccountMeta::new_readonly(
                                *context.target_account.key,
                                false,
                            )],
                            data: LighthouseInstruction::AssertTokenAccount(assertion.clone())
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
            LighthouseInstruction::AssertStakeAccount(assertion) => {
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    &context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
            LighthouseInstruction::AssertUpgradeableLoaderAccount(assertion) => {
                let context = AssertWithAccountContext::load(&mut accounts.iter())?;

                processor::assert_with_account(
                    &context,
                    &assertion,
                    Some(AssertionConfigV1 { verbose: true }),
                )?;
            }
            LighthouseInstruction::AssertSysvarClock(assertion) => {
                processor::assert_with_clock(
                    &assertion,
                    Some(AssertionConfigV1 { verbose: false }),
                )?;
            }
        }

        Ok(())
    }
}
