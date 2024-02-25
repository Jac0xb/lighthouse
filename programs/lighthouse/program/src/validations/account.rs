use std::collections::HashMap;

use crate::{error::LighthouseError, utils::Result};
use solana_program::{account_info::AccountInfo, msg, pubkey::Pubkey};
pub enum AccountValidation<'a> {
    IsEmpty,
    IsWritable,
    IsProgramDerivedAddress(&'a [&'a [u8]], Pubkey, Option<u8>),
    IsOwnedBy(Pubkey),
}

pub trait CheckedAccount<'info> {
    fn new(account: AccountInfo<'info>) -> Self;
    fn get_info(&self) -> &AccountInfo<'info>;
    fn key(&self) -> Pubkey;
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
                    msg!("account empty condition failed: {:?}", account.key);
                    return Err(LighthouseError::AccountValidaitonFailed.into());
                }
            }
            AccountValidation::IsWritable => {
                if !account.is_writable {
                    msg!("account is writable condition failed: {:?}", account.key);
                    return Err(LighthouseError::AccountValidaitonFailed.into());
                }
            }
            AccountValidation::IsProgramDerivedAddress(seeds, program_id, bump) => match bump {
                Some(bump) => {
                    let mut seeds_and_bump = vec![];
                    let bump_slice = [*bump];

                    for seed in seeds.iter() {
                        seeds_and_bump.push(*seed);
                    }
                    seeds_and_bump.push(bump_slice.as_ref());

                    // TODO: give pubkey create program address unwrap better error
                    let derived_address =
                        Pubkey::create_program_address(seeds_and_bump.as_slice(), program_id)
                            .unwrap();

                    if account.key != &derived_address {
                        msg!(
                            "program derived address condition failed: expected {:?} actual {:?}",
                            account.key,
                            derived_address
                        );
                        return Err(LighthouseError::AccountValidaitonFailed.into());
                    }

                    bump_map.insert(*account.key, *bump);
                }
                None => {
                    let (generated_pda, bump) = Pubkey::find_program_address(seeds, program_id);

                    if account.key != &generated_pda {
                        msg!(
                            "program derived address condition failed: expected {:?} actual {:?}",
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
                        "account owner condition failed: expected {:?} actual {:?}",
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

    fn key(&self) -> Pubkey {
        *self.account_info.key
    }

    fn new(account: AccountInfo<'info>) -> Self {
        Self {
            account_info: account,
        }
    }
}
