use std::collections::HashMap;
use std::slice::Iter;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT};
use solana_program::{msg, pubkey::Pubkey};

use crate::err_msg;
use crate::error::LighthouseError;

use crate::types::{AccountInfoData, WriteType, WriteTypeParameter};
use crate::utils::Result;
use crate::validations::{
    to_checked_account, AccountValidation, CheckedAccount, MemoryAccount, Program, Signer,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct WriteParameters {
    pub memory_index: u8,
    pub memory_account_bump: u8,
    pub write_type: WriteTypeParameter,
}

#[allow(dead_code)]
pub(crate) struct WriteContext<'a, 'info> {
    pub lighthouse_program: Program<'a, 'info>,
    pub payer: Signer<'a, 'info>,
    pub memory_account: MemoryAccount<'info>,
    pub source_account: AccountInfo<'info>,
}

impl<'a, 'info> WriteContext<'a, 'info> {
    pub(crate) fn load(
        account_iter: &mut Iter<'a, AccountInfo<'info>>,
        parameters: &WriteParameters,
    ) -> Result<Self> {
        let lighthouse_program = Program::new(next_account_info(account_iter)?, &crate::id())?;
        let payer = Signer::new(next_account_info(account_iter)?)?;

        let bump_map = &mut HashMap::<Pubkey, u8>::new();
        let memory_account = to_checked_account::<MemoryAccount>(
            next_account_info(account_iter)?,
            vec![
                AccountValidation::IsWritable,
                AccountValidation::IsInited,
                AccountValidation::IsProgramDerivedAddress(
                    MemoryAccount::get_seeds(payer.key, parameters.memory_index, None),
                    *lighthouse_program.key,
                    Some(parameters.memory_account_bump),
                ),
            ],
            bump_map,
        )?;
        let source_account = next_account_info(account_iter)?.clone();

        if source_account.key.eq(&memory_account.key()) {
            return Err(LighthouseError::UnauthorizedIxEntry.into());
        }

        Ok(Self {
            lighthouse_program,
            payer,
            memory_account,
            source_account,
        })
    }
}

pub(crate) fn write(context: WriteContext, parameters: WriteParameters) -> Result<()> {
    if get_stack_height() > TRANSACTION_LEVEL_STACK_HEIGHT {
        msg!("Stack height is greater than transaction level stack height");
        return Err(LighthouseError::UnauthorizedIxEntry.into());
    }

    let WriteContext {
        lighthouse_program: _,
        payer: _,
        memory_account,
        source_account,
    } = context;

    let WriteParameters {
        memory_index: _,
        memory_account_bump: _,
        write_type,
    } = parameters;

    let (memory_offset, write_type) = match write_type {
        WriteTypeParameter::WriteU8 {
            offset: memory_offset,
            write_type,
        } => (memory_offset as usize, write_type),
        WriteTypeParameter::WriteU16 {
            offset: memory_offset,
            write_type,
        } => (memory_offset as usize, write_type),
        WriteTypeParameter::WriteU32 {
            offset: memory_offset,
            write_type,
        } => (memory_offset as usize, write_type),
    };

    let memory_ref = &mut memory_account.account_info.try_borrow_mut_data()?;

    match write_type {
        WriteType::Program => {
            return Err(LighthouseError::Unimplemented.into());
        }
        WriteType::DataValue(borsh_value) => {
            let bytes = borsh_value.serialize()?;

            let slice_range = memory_offset..(memory_offset + bytes.len());
            let memory_ref_slice = memory_ref.get_mut(slice_range.clone()).ok_or_else(|| {
                msg!("DataValue write - range out of bounds {:?}", slice_range);
                LighthouseError::RangeOutOfBounds
            })?;

            memory_ref_slice.copy_from_slice(&bytes);
        }
        WriteType::AccountBalance => {
            unimplemented!("");
        }
        WriteType::AccountData {
            offset: data_offset,
            data_length,
        } => {
            let data_offset = data_offset as usize;
            let data_length = if let Some(data_length) = data_length {
                data_length as usize
            } else {
                source_account
                    .data_len()
                    .checked_sub(data_offset)
                    .ok_or_else(|| {
                        msg!("Account data offset index out of bounds");
                        LighthouseError::RangeOutOfBounds
                    })?
            };

            let data = source_account.try_borrow_data().map_err(|err| {
                msg!("Failed to borrow target account: {:?}", err);
                LighthouseError::AccountBorrowFailed
            })?;

            let data_range = data_offset..(data_offset + data_length);
            let data_slice = data.get(data_range.clone()).ok_or_else(|| {
                msg!("AccountData - read range out of bounds {:?}", data_range);
                LighthouseError::RangeOutOfBounds
            })?;

            let memory_range = memory_offset..(memory_offset + data_length);
            let memory_ref_slice = memory_ref.get_mut(memory_range.clone()).ok_or_else(|| {
                msg!("AccountData - write range out of bounds {:?}", memory_range);
                LighthouseError::RangeOutOfBounds
            })?;

            memory_ref_slice.copy_from_slice(data_slice);
        }
        WriteType::AccountInfo => {
            let account_info = AccountInfoData {
                key: *source_account.key,
                executable: source_account.executable,
                lamports: **source_account.try_borrow_lamports()?,
                data_length: source_account.try_borrow_data()?.len() as u64,
                owner: *source_account.owner,
                rent_epoch: source_account.rent_epoch,
            };

            let data = account_info.try_to_vec().map_err(|err| {
                err_msg!("Failed serialize AccountInfo", err);
                LighthouseError::FailedToSerialize
            })?;

            let memory_range = memory_offset..(memory_offset + data.len());
            let memory_ref_slice = memory_ref.get_mut(memory_range.clone()).ok_or_else(|| {
                msg!("AccountInfo write - range out of bounds {:?}", memory_range);
                LighthouseError::RangeOutOfBounds
            })?;

            memory_ref_slice.copy_from_slice(&data);
        }
    };

    Ok(())
}
