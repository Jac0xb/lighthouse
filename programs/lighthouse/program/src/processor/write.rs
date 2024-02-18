use std::slice::Iter;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT};
use solana_program::msg;

use crate::error::LighthouseError;

use crate::types::{AccountInfoData, WriteType, WriteTypeParameter};
use crate::utils::Result;
use crate::validations::{
    to_checked_account, AccountValidation, CheckedAccount, MemoryAccount, Program, Signer,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub(crate) struct WriteParameters {
    pub(crate) memory_index: u8,
    pub(crate) memory_account_bump: u8,
    pub(crate) write_type: WriteTypeParameter,
}

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

        let memory_account_seeds = [
            b"memory".as_ref(),
            payer.key.as_ref(),
            &[parameters.memory_index],
        ];

        let (memory_account, _) = to_checked_account::<MemoryAccount>(
            next_account_info(account_iter)?.clone(),
            &vec![
                AccountValidation::IsWritable,
                AccountValidation::IsProgramDerivedAddress(
                    &memory_account_seeds,
                    *lighthouse_program.key,
                    Some(parameters.memory_account_bump),
                ),
            ],
        )?;
        let source_account = next_account_info(account_iter)?.clone();

        if source_account.key.eq(&memory_account.key()) {
            // TODO: return with better error
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

    let memory_ref = &mut memory_account.account_info.try_borrow_mut_data()?;
    let memory_data_length = memory_ref.len();

    let (memory_offset, write_type) = match write_type {
        WriteTypeParameter::WriteU8(memory_offset, write_type) => {
            (memory_offset as usize, write_type)
        }
        WriteTypeParameter::WriteU16(memory_offset, write_type) => {
            (memory_offset as usize, write_type)
        }
        WriteTypeParameter::WriteU32(memory_offset, write_type) => {
            (memory_offset as usize, write_type)
        }
    };

    // let data_length = write_type
    //     .size(Some(&source_account))
    //     .ok_or(LighthouseError::OutOfRange)?;

    // if memory_data_length < (memory_offset + data_length) {
    //     return Err(LighthouseError::OutOfRange.into());
    // }

    match write_type {
        WriteType::Program => {
            return Err(LighthouseError::Unimplemented.into());
        }
        WriteType::DataValue(borsh_value) => {
            let bytes = borsh_value.serialize();

            if (memory_offset + bytes.len()) <= memory_data_length {
                memory_ref[memory_offset..(memory_offset + bytes.len())].copy_from_slice(&bytes);
            } else {
                msg!("DataValue write out of range");
                return Err(LighthouseError::OutOfRange.into());
            }
        }
        WriteType::AccountBalance => {
            let data = source_account.lamports();
            let data_slice = &data.to_le_bytes();
            let data_length = data_slice.len();

            memory_ref[memory_offset..(memory_offset + data_length)]
                .copy_from_slice(data_slice.as_ref());
        }
        WriteType::AccountData(account_offset, data_length) => {
            let account_offset = account_offset as usize;
            let data_length = if let Some(data_length) = data_length {
                data_length as usize
            } else {
                source_account
                    .data_len()
                    .checked_sub(account_offset)
                    .ok_or(LighthouseError::OutOfRange)?
            };

            let data = source_account.try_borrow_data().map_err(|err| {
                msg!("Error: {:?}", err);
                LighthouseError::AccountBorrowFailed
            })?;
            let data_slice = data.get(account_offset..(account_offset + data_length));

            if let Some(data_slice) = data_slice {
                memory_ref[memory_offset..(memory_offset + data_length)]
                    .copy_from_slice(data_slice);
            } else {
                return Err(LighthouseError::OutOfRange.into());
            }
        }
        WriteType::AccountInfo => {
            let account_info = AccountInfoData {
                key: *source_account.key,
                is_signer: source_account.is_signer,
                is_writable: source_account.is_writable,
                executable: source_account.executable,
                lamports: **source_account.try_borrow_lamports()?, // TODO: make this unwrap nicer
                data_length: source_account.try_borrow_data()?.len() as u64, // TODO: make this unwrap nicer
                owner: *source_account.owner,
                rent_epoch: source_account.rent_epoch,
            };

            let data = account_info.try_to_vec()?; // TODO: map this unwrap error
            let data_length = data.len();

            let data_slice = &data[0..data_length];

            memory_ref[memory_offset..(memory_offset + data_length)]
                .copy_from_slice(data_slice.as_ref());
        }
    };

    Ok(())
}

// TODO write tests
