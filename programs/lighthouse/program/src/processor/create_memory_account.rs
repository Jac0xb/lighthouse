use std::{collections::HashMap, slice::Iter};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use crate::{
    utils::{create_account, Result},
    validations::{to_checked_account, AccountValidation, MemoryAccount, Program, Signer},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct CreateMemoryAccountParameters {
    pub memory_index: u8,
    pub memory_account_size: u64,
}

pub(crate) struct CreateMemoryAccountContext<'a, 'info> {
    #[allow(dead_code)]
    pub(crate) lighthouse_program: Program<'a, 'info>,
    pub(crate) payer: Signer<'a, 'info>,
    pub(crate) memory_account: MemoryAccount<'info>,
    pub(crate) system_program: Program<'a, 'info>,
}

impl<'a, 'info> CreateMemoryAccountContext<'a, 'info> {
    pub(crate) fn load(
        account_iter: &mut Iter<'a, AccountInfo<'info>>,
        parameters: &CreateMemoryAccountParameters,
    ) -> Result<(Self, HashMap<Pubkey, u8>)> {
        let mut bump_map = HashMap::<Pubkey, u8>::new();

        let lighthouse_program = Program::new(next_account_info(account_iter)?, &crate::id())?;
        let payer = Signer::new(next_account_info(account_iter)?)?;
        let memory_account = to_checked_account(
            next_account_info(account_iter)?,
            vec![
                AccountValidation::IsNotInited,
                AccountValidation::IsWritable,
                AccountValidation::IsProgramDerivedAddress(
                    MemoryAccount::get_seeds(payer.key, parameters.memory_index, None),
                    crate::id(),
                    None,
                ),
            ],
            &mut bump_map,
        )?;

        Ok((
            Self {
                lighthouse_program,
                payer,
                memory_account,
                system_program: Program::new(
                    next_account_info(account_iter)?,
                    &solana_program::system_program::id(),
                )?,
            },
            bump_map,
        ))
    }
}

pub(crate) fn create_memory_account(
    context: CreateMemoryAccountContext,
    parameters: CreateMemoryAccountParameters,
    bump_map: HashMap<Pubkey, u8>,
) -> Result<()> {
    let CreateMemoryAccountContext {
        lighthouse_program: _,
        payer,
        memory_account,
        system_program,
    } = context;

    let CreateMemoryAccountParameters {
        memory_index,
        memory_account_size,
    } = parameters;

    // TODO: better error handling
    let bump = *bump_map.get(memory_account.account_info.key).unwrap();

    create_account(
        payer.as_ref(),
        &memory_account.account_info,
        system_program.as_ref(),
        &crate::id(),
        &Rent::get().unwrap(),
        memory_account_size,
        MemoryAccount::get_seeds(payer.key, memory_index, Some(bump)),
    )?;

    Ok(())
}
