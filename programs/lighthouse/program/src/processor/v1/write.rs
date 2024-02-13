use anchor_lang::prelude::*;
use borsh::BorshDeserialize;
use solana_program::instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT};

use crate::error::LighthouseError;
use crate::types::{AccountInfoData, WriteType, WriteTypeParameter};

#[derive(Accounts)]
#[instruction(memory_index: u8, memory_account_bump: u8)]
pub struct WriteV1<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: ...
    #[account(
        mut,
        seeds=[
            b"memory".as_ref(),
            signer.key.as_ref(),
            &[memory_index],
        ],
        bump = memory_account_bump,
    )]
    pub memory_account: UncheckedAccount<'info>,
}

pub fn write<'info>(
    ctx: Context<'_, '_, '_, 'info, WriteV1<'info>>,
    _memory_index: u8,
    _memory_account_bump: u8,
    write_type: WriteTypeParameter,
) -> Result<()> {
    if get_stack_height() > TRANSACTION_LEVEL_STACK_HEIGHT {
        msg!("Stack height is greater than transaction level stack height");
        return Err(LighthouseError::UnauthorizedIxEntry.into());
    }

    let memory_ref = &mut ctx.accounts.memory_account.try_borrow_mut_data()?;
    let memory_data_length = memory_ref.len();

    let (mut memory_offset, write_type) = match write_type {
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

    memory_offset = memory_offset.checked_add(8).ok_or_else(|| {
        msg!("Memory offset overflow");
        LighthouseError::OutOfRange
    })?;

    let data_length = write_type
        .size(ctx.remaining_accounts.first())
        .ok_or(LighthouseError::InvalidDataLength)?;

    if memory_data_length < (memory_offset + data_length) {
        msg!(
            "Memory offset overflowed {} < {} + {}",
            memory_data_length,
            memory_offset,
            data_length
        );
        return Err(LighthouseError::OutOfRange.into());
    }

    match write_type {
        WriteType::Program => {
            return Err(LighthouseError::Unimplemented.into());
        }
        WriteType::DataValue(borsh_value) => {
            let data_slice = &(borsh_value.serialize())[0..data_length];
            memory_ref[memory_offset..(memory_offset + data_length)]
                .copy_from_slice(data_slice.as_ref());
        }
        WriteType::AccountBalance => {
            let source_account = ctx.remaining_accounts.first();

            if let Some(target_account) = source_account {
                if (memory_offset + data_length) <= memory_data_length {
                    let data = target_account.lamports();
                    let data_slice = &data.to_le_bytes();

                    memory_ref[memory_offset..(memory_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    msg!("Not enough memory to write account balance");
                    return Err(LighthouseError::NotEnoughAccounts.into());
                }
            } else {
                msg!("No account to write balance from");
                return Err(LighthouseError::NotEnoughAccounts.into());
            }
        }
        WriteType::AccountData(account_offset, _, account_validation) => {
            let target_account = ctx.remaining_accounts.first();
            let account_offset = account_offset as usize;

            // Additional validation on account that's been written to.
            if let Some(target_account) = target_account {
                if let Some(account_validation) = account_validation {
                    if let Some(owner) = account_validation.owner {
                        if owner != *target_account.owner {
                            return Err(LighthouseError::AccountOwnerValidationFailed.into());
                        }
                    }

                    if let Some(assert_is_funded) = account_validation.is_funded {
                        let is_funded = target_account.lamports() == 0;
                        if assert_is_funded != is_funded {
                            return Err(LighthouseError::AccountFundedValidationFailed.into());
                        }
                    }

                    if let Some(discriminator) = account_validation.discriminator {
                        let data = target_account.try_borrow_data().map_err(|err| {
                            msg!("Error: {:?}", err);
                            LighthouseError::AccountBorrowFailed
                        })?;

                        if discriminator.len() > data.len() {
                            msg!("Discriminator length is greater than account data length");
                            return Err(LighthouseError::AccountOutOfRange.into());
                        }

                        let data_slice = &data[0..discriminator.len()];

                        if !data_slice.eq(discriminator.as_slice()) {
                            return Err(
                                LighthouseError::AccountDiscriminatorValidationFailed.into()
                            );
                        }
                    }
                }

                let data = target_account.try_borrow_data().map_err(|err| {
                    msg!("Error: {:?}", err);
                    LighthouseError::AccountBorrowFailed
                })?;

                let data_slice = &data.get(account_offset..(account_offset + data_length));

                if let Some(data_slice) = data_slice {
                    memory_ref[memory_offset..(memory_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    msg!("Not enough memory to write account data");
                    return Err(LighthouseError::NotEnoughAccounts.into());
                }
            } else {
                msg!("No account to write data from");
                return Err(LighthouseError::NotEnoughAccounts.into());
            }
        }
        WriteType::AccountInfo => {
            let target_account = ctx.remaining_accounts.first();

            if let Some(target_account) = target_account {
                if (memory_offset + data_length) <= memory_data_length {
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

                    memory_ref[memory_offset..(memory_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    msg!("Not enough memory to write account info");
                    return Err(LighthouseError::NotEnoughAccounts.into());
                }
            } else {
                msg!("No account to write info from");
                return Err(LighthouseError::NotEnoughAccounts.into());
            }
        }
    };

    Ok(())
}
