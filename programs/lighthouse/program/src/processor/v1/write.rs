use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use borsh::BorshDeserialize;

use crate::error::ProgramError;
use crate::structs::{AccountInfoData, WriteType};

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
    write_type: WriteType,
) -> Result<()> {
    let cache_ref = &mut ctx.accounts.cache_account.try_borrow_mut_data()?;
    let cache_data_length = cache_ref.len();

    let mut cache_offset: usize;
    let account_offset: usize;
    let data_length: usize;

    // TODO: make less messy but the main point is to allow more compact instruction data.
    (cache_offset, account_offset, data_length) = match write_type {
        WriteType::AccountBalanceU8(_cache_offset) => (_cache_offset as usize, 0, 8),
        WriteType::AccountBalanceU16(_cache_offset) => (_cache_offset as usize, 0, 16),
        WriteType::AccountBalanceU32(_cache_offset) => (_cache_offset as usize, 0, 32),
        WriteType::AccountDataU8(_cache_offset, account_offset, data_length) => (
            _cache_offset as usize,
            account_offset as usize,
            data_length as usize,
        ),
        WriteType::AccountDataU16(_cache_offset, account_offset, data_length) => (
            _cache_offset as usize,
            account_offset as usize,
            data_length as usize,
        ),
        WriteType::AccountDataU32(_cache_offset, account_offset, data_length) => (
            _cache_offset as usize,
            account_offset as usize,
            data_length as usize,
        ),
        WriteType::BorshFieldU8(_cache_offset, _) => (_cache_offset as usize, 0, 0),
        WriteType::BorshFieldU16(_cache_offset, _) => (_cache_offset as usize, 0, 0),
        WriteType::MintAccount => (0, 0, 0),
        WriteType::TokenAccount(_cache_offset) => (_cache_offset as usize, 0, TokenAccount::LEN),
        WriteType::TokenAccountOwner(_cache_offset) => (_cache_offset as usize, 0, 32),
        WriteType::TokenAccountBalance(_cache_offset) => (_cache_offset as usize, 0, 8),
        WriteType::AccountInfoU8(_cache_offset) => {
            (_cache_offset as usize, 0, AccountInfoData::size() as usize)
        }
        WriteType::AccountInfoU16(_cache_offset) => {
            (_cache_offset as usize, 0, AccountInfoData::size() as usize)
        }
        WriteType::AccountInfoU32(_cache_offset) => {
            (_cache_offset as usize, 0, AccountInfoData::size() as usize)
        }
    };

    // Cache offset can never write to the first 8 bytes of the cache account
    cache_offset = cache_offset
        .checked_add(8)
        .ok_or(ProgramError::CacheOutOfRange)?;

    match write_type {
        WriteType::AccountBalanceU8(_)
        | WriteType::AccountBalanceU16(_)
        | WriteType::AccountBalanceU32(_) => {
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
        WriteType::AccountDataU8(_, _, _)
        | WriteType::AccountDataU16(_, _, _)
        | WriteType::AccountDataU32(_, _, _) => {
            msg!("write_type: AccountData");
            let source_account = ctx.remaining_accounts.first();

            if let Some(target_account) = source_account {
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
        WriteType::TokenAccount(_) => {
            // TODO: Not sure we really need this, could be extracted by user

            msg!("write_type: TokenAccount");
            let source_account = ctx.remaining_accounts.first();

            if let Some(source_account) = source_account {
                // TODO: add validation to token account

                if (cache_offset + data_length) < cache_data_length {
                    let data = source_account.try_borrow_data()?;
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
        WriteType::TokenAccountBalance(_) => {
            msg!("write_type: TokenAccountBalance");
            let source_account = ctx.remaining_accounts.first();

            if let Some(target_account) = source_account {
                if (cache_offset + data_length) < cache_data_length {
                    let data = target_account.try_borrow_data()?;
                    let token_account = TokenAccount::try_deserialize(&mut data.as_ref())?;
                    let data_slice = token_account.amount.to_le_bytes();

                    cache_ref[cache_offset..(cache_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    return Err(ProgramError::NotEnoughAccounts.into());
                }
            } else {
                return Err(ProgramError::NotEnoughAccounts.into());
            }
        }
        WriteType::TokenAccountOwner(_) => {
            msg!("write_type: TokenAccountOwner");
            let source_account = ctx.remaining_accounts.first();

            if let Some(target_account) = source_account {
                if (cache_offset + data_length) < cache_data_length {
                    let data = target_account.try_borrow_data()?;
                    let token_account = TokenAccount::try_deserialize(&mut data.as_ref())?;
                    let data_slice = token_account.owner.to_bytes();

                    cache_ref[cache_offset..(cache_offset + data_length)]
                        .copy_from_slice(data_slice.as_ref());
                } else {
                    return Err(ProgramError::NotEnoughAccounts.into());
                }
            } else {
                return Err(ProgramError::NotEnoughAccounts.into());
            }
        }
        WriteType::AccountInfoU8(_)
        | WriteType::AccountInfoU16(_)
        | WriteType::AccountInfoU32(_) => {
            msg!("write_type: AccountInfoU8");
            let source_account = ctx.remaining_accounts.first();

            if let Some(target_account) = source_account {
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
        _ => {
            // TODO: MAKE A BETTER ERROR
            return Err(ProgramError::NotEnoughAccounts.into());
        }
    }

    Ok(())
}

// msg!(
//     "cache_offset: {}, dest_start: {}, slice_length: {}",
//     cache_offset,
//     dest_start,
//     slice_length
// );

// msg!(
//     "cache_account_data.len(): {}, source_account_data.len(): {}",
//     cache_account_data.len(),
//     source_account_data.len()
// );

// if ((cache_offset + slice_length) as usize) < cache_account_data.len() {
//     cache_account_data[cache_offset as usize..(cache_offset + slice_length) as usize]
//         .copy_from_slice(
//             &source_account_data[dest_start as usize..(dest_start + slice_length) as usize],
//         );
// } else {
//     // Handle the error: destination slice is not large enough
// }

// msg!("cache_account_data: {:?}", cache_account_data);
