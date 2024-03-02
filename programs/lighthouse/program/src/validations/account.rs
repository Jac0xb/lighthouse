use std::collections::HashMap;

use crate::{error::LighthouseError, utils::Result};
use solana_program::{account_info::AccountInfo, msg, pubkey::Pubkey};

pub enum AccountValidation {
    IsNotInited,
    IsInited,
    IsWritable,
    IsProgramDerivedAddress(Vec<Vec<u8>>, Pubkey, Option<u8>),
    IsOwnedBy(Pubkey),
}

pub trait CheckedAccount<'info> {
    fn new(account: AccountInfo<'info>) -> Self;
    fn get_info(&self) -> &AccountInfo<'info>;
    fn key(&self) -> Pubkey;
}

pub trait DerivedAccount<'info> {
    fn new(account: AccountInfo<'info>) -> Self;
    fn get_info(&self) -> &AccountInfo<'info>;
    fn key(&self) -> Pubkey;
}

pub fn to_checked_account<'a, 'info, T: CheckedAccount<'info>>(
    account: &'a AccountInfo<'info>,
    conditions: Vec<AccountValidation>,
    bump_map: &mut HashMap<Pubkey, u8>,
) -> Result<T> {
    for condition in conditions {
        match condition {
            AccountValidation::IsInited => {
                if account.data_is_empty() || account.owner.eq(&solana_program::system_program::ID)
                {
                    msg!("account inited condition failed: {:?}", account.key);
                    return Err(LighthouseError::AccountValidaitonFailed.into());
                }
            }
            AccountValidation::IsNotInited => {
                if account.lamports() != 0 || !account.owner.eq(&solana_program::system_program::ID)
                {
                    msg!("account not inited condition failed: {:?}", account.key);
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
                    let bump_slice = [bump];

                    seeds_and_bump.extend(seeds.iter().map(|seed| seed.as_slice()));
                    seeds_and_bump.push(&bump_slice);

                    let derived_address =
                        Pubkey::create_program_address(seeds_and_bump.as_slice(), &program_id)
                            .map_err(|err| {
                                msg!(
                                    "failed to create program address: {:?} {:?}",
                                    seeds_and_bump,
                                    err
                                );
                                LighthouseError::AccountValidaitonFailed
                            })?;

                    if account.key != &derived_address {
                        msg!(
                            "program derived address condition failed: expected {:?} actual {:?}",
                            account.key,
                            derived_address
                        );
                        return Err(LighthouseError::AccountValidaitonFailed.into());
                    }

                    bump_map.insert(*account.key, bump);
                }
                None => {
                    let seeds = seeds.iter().map(|seed| seed.as_slice()).collect::<Vec<_>>();

                    let (generated_pda, bump) =
                        Pubkey::find_program_address(seeds.as_slice(), &program_id);

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
                if !account.owner.eq(&owner) {
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

    Ok(T::new(account.clone()))
}

pub struct MemoryAccount<'info> {
    pub account_info: AccountInfo<'info>,
}

impl<'info> MemoryAccount<'info> {
    pub fn get_seeds(payer: &Pubkey, memory_index: u8, bump: Option<u8>) -> Vec<Vec<u8>> {
        vec![
            b"memory".to_vec(),
            payer.to_bytes().to_vec(),
            if let Some(bump) = bump {
                vec![memory_index, bump]
            } else {
                vec![memory_index]
            },
        ]
    }
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
