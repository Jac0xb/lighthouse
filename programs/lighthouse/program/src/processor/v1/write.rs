use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::error::ProgramError;
use crate::structs::{AccountInfoData, WriteType, WriteTypeParameter};

#[derive(Accounts)]
#[instruction(cache_index: u8)]
pub struct WriteV1<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds=[
            b"cache".as_ref(),
            signer.key.as_ref(),
            &[cache_index],
        ],
        bump
    )]
    pub cache_account: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn write<'info>(
    ctx: Context<'_, '_, '_, 'info, WriteV1<'info>>,
    _: u8,
    write_type: WriteTypeParameter,
) -> Result<()> {
    let cache_ref = &mut ctx.accounts.cache_account.try_borrow_mut_data()?;
    let cache_data_length = cache_ref.len();

    let (mut cache_offset, write_type) = match write_type {
        WriteTypeParameter::WriteU8(cache_offset, write_type) => {
            (cache_offset as usize, write_type)
        }
        WriteTypeParameter::WriteU16(cache_offset, write_type) => {
            (cache_offset as usize, write_type)
        }
        WriteTypeParameter::WriteU32(cache_offset, write_type) => {
            (cache_offset as usize, write_type)
        }
    };
    cache_offset = cache_offset
        .checked_add(8)
        .ok_or(ProgramError::CacheOutOfRange)?;

    let data_length = write_type.size();

    match write_type {
        WriteType::MintAccount => {}
        WriteType::TokenAccount2022 => {}
        WriteType::TokenAccountLegacy => {}
        WriteType::Program => {}
        WriteType::DataValue(borsh_value) => {
            if (cache_offset + data_length) < cache_data_length {
                let data_slice = &(borsh_value.serialize())[0..data_length];

                cache_ref[cache_offset..(cache_offset + data_length)]
                    .copy_from_slice(data_slice.as_ref());
            } else {
                return Err(ProgramError::NotEnoughAccounts.into());
            }
        }
        WriteType::AccountBalance => {
            let source_account = ctx.remaining_accounts.first();

            if let Some(target_account) = source_account {
                if (cache_offset + data_length) < cache_data_length {
                    let data = target_account.lamports();
                    let data_slice = &data.to_le_bytes();

                    cache_ref[cache_offset..(cache_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    return Err(ProgramError::NotEnoughAccounts.into());
                }
            } else {
                return Err(ProgramError::NotEnoughAccounts.into());
            }
        }
        WriteType::AccountData(account_offset, data_length) => {
            let target_account = ctx.remaining_accounts.first();
            let data_length = data_length as usize;
            let account_offset = account_offset as usize;

            if let Some(target_account) = target_account {
                if !write_type.account_validation(target_account) {
                    msg!("Could not validation account");
                    return Err(ProgramError::InvalidAccount.into());
                }

                if (cache_offset + data_length) < cache_data_length {
                    let data = target_account.try_borrow_data().map_err(|err| {
                        msg!("Error: {:?}", err);
                        ProgramError::AccountBorrowFailed
                    })?;
                    let data_slice = &data[account_offset..(account_offset + data_length)];

                    cache_ref[cache_offset..(cache_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    return Err(ProgramError::NotEnoughAccounts.into());
                }
            } else {
                return Err(ProgramError::NotEnoughAccounts.into());
            }
        }
        WriteType::AccountInfo => {
            let target_account = ctx.remaining_accounts.first();

            if let Some(target_account) = target_account {
                if (cache_offset + data_length) < cache_data_length {
                    let account_info = AccountInfoData {
                        key: *target_account.key,
                        is_signer: target_account.is_signer,
                        is_writable: target_account.is_writable,
                        executable: target_account.executable,
                        lamports: **target_account.try_borrow_lamports()?, // TODO: make this unwrap nicer
                        data_length: target_account.try_borrow_data()?.len() as u64, // TODO: make this unwrap nicer
                        owner: *target_account.owner,
                        rent_epoch: target_account.rent_epoch,
                    };

                    let data = account_info.try_to_vec()?; // TODO: map this unwrap error
                    let data_slice = &data[0..data_length];

                    cache_ref[cache_offset..(cache_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    return Err(ProgramError::NotEnoughAccounts.into());
                }
            } else {
                return Err(ProgramError::NotEnoughAccounts.into());
            }
        }
    }

    Ok(())
}
