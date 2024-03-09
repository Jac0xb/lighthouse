use crate::error::LighthouseError;
use crate::types::write::{AccountInfoField, DataValue, WriteType};
use crate::utils::Result;
use crate::validation::{
    AccountValidation, CheckedAccount, DerivedAddress, InitializeType, LighthouseProgram,
    MemoryAccount, MemoryAccountSeeds, Program, Signer, SystemProgram,
};
use crate::{err, err_msg};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT},
    msg,
    program_error::ProgramError,
};
use std::slice::Iter;

#[allow(dead_code)]
pub(crate) struct MemoryWriteContext<'a, 'info> {
    pub lighthouse_program: Program<'a, 'info, LighthouseProgram>,
    pub system_program: Program<'a, 'info, SystemProgram>,
    pub payer: Signer<'a, 'info>,
    pub memory_account: MemoryAccount<'a, 'info>,
    pub source_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> MemoryWriteContext<'a, 'info> {
    pub(crate) fn load(
        account_iter: &mut Iter<'a, AccountInfo<'info>>,
        memory_index: u8,
        memory_offset: u16,
        memory_account_bump: u8,
        write_type: &WriteType,
    ) -> Result<Self> {
        let lighthouse_program = Program::new_checked(next_account_info(account_iter)?, None)?;
        let system_program = Program::new_checked(next_account_info(account_iter).unwrap(), None)?;
        let payer = Signer::new_checked(next_account_info(account_iter)?, None)?;

        let seeds = &MemoryAccount::get_seeds(MemoryAccountSeeds {
            payer: payer.key,
            memory_index,
            bump: Some(memory_account_bump),
        });

        let required_space = (memory_offset as usize) + write_type.data_length();

        let memory_account_info = next_account_info(account_iter)?;
        let memory_account = if memory_account_info.try_data_len()? < required_space {
            MemoryAccount::new_init_checked(
                memory_account_info,
                InitializeType::InitOrReallocIfNeeded {
                    space: required_space,
                    payer: &payer,
                    program_owner: lighthouse_program.key,
                    system_program: &system_program,
                    seeds,
                },
                Some(&vec![
                    AccountValidation::IsWritable,
                    AccountValidation::IsProgramDerivedAddress {
                        seeds,
                        program_id: lighthouse_program.key,
                        find_bump: false,
                    },
                ]),
            )?
        } else {
            MemoryAccount::new_checked(
                memory_account_info,
                Some(&vec![
                    AccountValidation::IsWritable,
                    AccountValidation::IsProgramDerivedAddress {
                        seeds,
                        program_id: lighthouse_program.key,
                        find_bump: false,
                    },
                ]),
            )?
        };

        let source_account = next_account_info(account_iter)?;

        Ok(Self {
            system_program,
            lighthouse_program,
            payer,
            memory_account,
            source_account,
        })
    }
}

pub(crate) fn memory_write(
    context: &MemoryWriteContext,
    offset: u16,
    write_type: &WriteType,
) -> Result<()> {
    if get_stack_height() > TRANSACTION_LEVEL_STACK_HEIGHT {
        msg!("Cross-program invocation violation");
        return Err(LighthouseError::CrossProgramInvokeViolation.into());
    }

    let memory_account = context.memory_account.clone();
    let source_account = context.source_account;

    let memory_offset = offset as usize;
    let memory_ref = &mut memory_account.info().try_borrow_mut_data()?;

    match write_type {
        WriteType::DataValue(data_value) => {
            let err_map: fn(e: std::io::Error) -> ProgramError = |e| {
                err_msg!("Failed to serialize data value", e);
                err!(LighthouseError::FailedToSerialize)
            };

            let bytes = match data_value {
                DataValue::Bool(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::U8(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::I8(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::U16(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::I16(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::U32(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::I32(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::U64(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::I64(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::U128(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::I128(value) => value.try_to_vec().map_err(err_map)?,
                DataValue::Bytes(value) => value.clone(),
                DataValue::Pubkey(value) => value.to_bytes().to_vec(),
            };
            let data_length = write_type.data_length();

            let memory_write_range = memory_offset..(memory_offset + data_length);
            let memory_write_slice =
                memory_ref
                    .get_mut(memory_write_range.clone())
                    .ok_or_else(|| {
                        msg!(
                            "DataValue write - range out of bounds {:?} write length {:?}",
                            memory_write_range,
                            memory_account.info().data_len()
                        );
                        LighthouseError::RangeOutOfBounds
                    })?;

            memory_write_slice.copy_from_slice(&bytes);
        }
        WriteType::AccountData {
            offset: data_offset,
            data_length: _,
        } => {
            let data_offset = *data_offset as usize;
            let data_length = write_type.data_length();

            let data = source_account.try_borrow_data().map_err(|err| {
                msg!("Failed to borrow target account: {:?}", err);
                LighthouseError::AccountBorrowFailed
            })?;

            let data_range = data_offset..(data_offset + data_length);
            let data_slice = data.get(data_range.clone()).ok_or_else(|| {
                msg!("AccountData - read range out of bounds {:?}", data_range);
                LighthouseError::RangeOutOfBounds
            })?;

            let memory_write_range = memory_offset..(memory_offset + data_length);
            let memory_write_slice =
                memory_ref
                    .get_mut(memory_write_range.clone())
                    .ok_or_else(|| {
                        msg!(
                            "AccountData - write range out of bounds {:?}",
                            memory_write_range
                        );
                        LighthouseError::RangeOutOfBounds
                    })?;

            memory_write_slice.copy_from_slice(data_slice);
        }
        WriteType::AccountInfoField(field) => {
            let bytes = match field {
                AccountInfoField::Key => source_account.key.try_to_vec().map_err(|err| {
                    err_msg!("Failed to serialize AccountInfo.key", err);
                    LighthouseError::FailedToSerialize
                })?,
                AccountInfoField::Lamports => {
                    source_account.lamports().try_to_vec().map_err(|err| {
                        err_msg!("Failed to serialize AccountInfo.lamports", err);
                        LighthouseError::FailedToSerialize
                    })?
                }
                AccountInfoField::Owner => source_account.owner.try_to_vec().map_err(|err| {
                    err_msg!("Failed to serialize AccountInfo.owner", err);
                    LighthouseError::FailedToSerialize
                })?,
                AccountInfoField::RentEpoch => {
                    source_account.rent_epoch.try_to_vec().map_err(|err| {
                        err_msg!("Failed to serialize AccountInfo.rent_epoch", err);
                        LighthouseError::FailedToSerialize
                    })?
                }
                AccountInfoField::DataLength => {
                    source_account.data_len().try_to_vec().map_err(|err| {
                        err_msg!("Failed to serialize AccountInfo.data_len", err);
                        LighthouseError::FailedToSerialize
                    })?
                }
                AccountInfoField::Executable => {
                    source_account.executable.try_to_vec().map_err(|err| {
                        err_msg!("Failed to serialize AccountInfo.executable", err);
                        LighthouseError::FailedToSerialize
                    })?
                }
            };
            let data_length = write_type.data_length();

            let memory_write_range = memory_offset..(memory_offset + data_length);
            let memory_write_slice =
                memory_ref
                    .get_mut(memory_write_range.clone())
                    .ok_or_else(|| {
                        msg!(
                            "AccountInfo write - range out of bounds {:?}",
                            memory_write_range
                        );
                        LighthouseError::RangeOutOfBounds
                    })?;

            memory_write_slice.copy_from_slice(&bytes);
        }
    };

    Ok(())
}
