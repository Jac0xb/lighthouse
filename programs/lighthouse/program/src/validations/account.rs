use std::collections::HashMap;

use crate::{error::LighthouseError, utils::Result};
use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use sokoban::node_allocator::ZeroCopy;
use solana_program::{
    account_info::AccountInfo, keccak, msg, program_error::ProgramError, pubkey::Pubkey,
};

// use super::status::{MarketStatus, SeatApprovalStatus};

/// This function returns the canonical discriminant of the given type. It is the result
/// of hashing together the program ID and the name of the type.
///
/// Suppose a program has an account type named `Foo` and another type named `Bar`.
/// A common attack vector would be to pass an account of type `Bar` to a function
/// expecting an account of type `Foo`, but by checking the discriminants, the function
/// would be able to detect that the `Bar` account is not of the expected type `Foo`.
pub fn get_discriminant<T>() -> Result<u64> {
    let type_name = std::any::type_name::<T>();
    let discriminant = u64::from_le_bytes(
        keccak::hashv(&[crate::ID.as_ref(), type_name.as_bytes()]).as_ref()[..8]
            .try_into()
            .map_err(|_| {
                // phoenix_log!("Failed to convert discriminant hash to u64");
                ProgramError::InvalidAccountData
            })?,
    );
    // phoenix_log!("Discriminant for {} is {}", type_name, discriminant);
    Ok(discriminant)
}

pub enum AccountValidation<'a> {
    IsEmpty,
    IsWritable,
    IsProgramDerivedAddress(&'a [&'a [u8]], Pubkey, Option<u8>),
    IsOwnedBy(Pubkey),
}

pub trait CheckedAccount<'info> {
    fn new(account: AccountInfo<'info>) -> Self;
    fn get_info(&self) -> &AccountInfo<'info>;
}

pub fn to_checked_account<'a, 'info, T: CheckedAccount<'info>>(
    account: AccountInfo<'info>,
    conditions: &Vec<AccountValidation<'_>>,
) -> Result<(T, HashMap<Pubkey, u8>)> {
    let mut bump_map = HashMap::new();

    for condition in conditions {
        match condition {
            AccountValidation::IsEmpty => {
                if account.lamports() != 0 || !account.data_is_empty() {
                    msg!("account is not empty");
                    return Err(LighthouseError::AccountValidaitonFailed.into());
                }
            }
            AccountValidation::IsWritable => {
                if !account.is_writable {
                    msg!("account is not writable");
                    return Err(LighthouseError::AccountValidaitonFailed.into());
                }
            }
            AccountValidation::IsProgramDerivedAddress(seeds, program_id, bump) => match bump {
                Some(bump) => {
                    // TODO: give pubkey create program address unwrap better error

                    let mut seeds_and_bump = vec![];
                    let bump_slice = [*bump];

                    for seed in seeds.iter() {
                        seeds_and_bump.push(*seed);
                    }
                    seeds_and_bump.push(bump_slice.as_ref());

                    let generated_pda =
                        Pubkey::create_program_address(seeds_and_bump.as_slice(), program_id)
                            .unwrap();

                    if account.key != &generated_pda {
                        msg!(
                            "account owner failed left {:?} right {:?}",
                            account.key,
                            generated_pda
                        );
                        return Err(LighthouseError::AccountValidaitonFailed.into());
                    }

                    bump_map.insert(*account.key, *bump);
                }
                None => {
                    let (generated_pda, bump) = Pubkey::find_program_address(seeds, program_id);

                    if account.key != &generated_pda {
                        msg!(
                            "account owner failed left {:?} right {:?}",
                            account.key,
                            generated_pda
                        );

                        return Err(LighthouseError::AccountValidaitonFailed.into());
                    }

                    bump_map.insert(*account.key, bump);
                }
            },
            AccountValidation::IsOwnedBy(owner) => {
                if !account.owner.eq(owner) {
                    msg!(
                        "account owner failed left {:?} right {:?}",
                        account.owner,
                        owner
                    );
                    return Err(LighthouseError::AccountValidaitonFailed.into());
                }
            }
        }
    }

    Ok((T::new(account), bump_map))
}

pub struct MemoryAccount<'info> {
    pub account_info: AccountInfo<'info>,
}

impl<'info> CheckedAccount<'info> for MemoryAccount<'info> {
    fn get_info(&self) -> &AccountInfo<'info> {
        &self.account_info
    }

    fn new(account: AccountInfo<'info>) -> Self {
        Self {
            account_info: account,
        }
    }
}
