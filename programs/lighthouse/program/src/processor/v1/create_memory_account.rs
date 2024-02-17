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
    validations::{
        to_checked_account, AccountValidation, CheckedAccount, MemoryAccount, Program, Signer,
    },
};

// use crate::state::memory::MemoryAccount;

// #[derive(Accounts)]
// #[instruction(memory_index: u8, memory_account_size: u64)]
// pub struct CreateMemoryAccountV1<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     pub system_program: Program<'info, System>,
//     #[account(
//         init,
//         seeds=[
//             b"memory".as_ref(),
//             signer.key.as_ref(),
//             &[memory_index],
//         ],
//         bump,
//         payer=signer,
//         space= 8 + memory_account_size as usize
//     )]
//     pub memory_account: AccountLoader<'info, MemoryAccount>,
//     pub rent: Sysvar<'info, Rent>,
// }

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub(crate) struct CreateMemoryAccountParameters {
    pub(crate) memory_index: u8,
    pub(crate) memory_account_size: u64,
}

pub(crate) struct CreateMemoryAccountContext<'a, 'info> {
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
        let lighthouse_program = Program::new(next_account_info(account_iter)?, &crate::id())?;
        let payer = Signer::new(next_account_info(account_iter)?)?;
        let memory_account_seeds = [b"memory", payer.key.as_ref(), &[parameters.memory_index]];
        let (memory_account, bump_map) = to_checked_account(
            next_account_info(account_iter)?.clone(),
            &vec![
                AccountValidation::IsEmpty,
                AccountValidation::IsWritable,
                AccountValidation::IsProgramDerivedAddress(
                    &memory_account_seeds,
                    crate::id(),
                    None,
                ),
            ],
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

pub(crate) fn create_memory_account<'a, 'info>(
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

    let bump = *bump_map.get(&memory_account.account_info.key).unwrap();

    create_account(
        payer.as_ref(),
        &memory_account.account_info,
        system_program.as_ref(),
        &crate::id(),
        &Rent::get().unwrap(),
        memory_account_size,
        vec![
            b"memory".to_vec(),
            payer.key.try_to_vec().unwrap(),
            vec![memory_index, bump],
        ],
    )?;

    Ok(())
}

// pub fn get_memory_account(payer: &Pubkey, memory_index: u8) -> (Pubkey, u8) {
//     Pubkey::find_program_address(&[b"memory", payer.as_ref(), &[memory_index]], &crate::ID)
// }

pub fn get_memory_account_with_bump(payer: &Pubkey, memory_index: u8, bump: u8) -> Pubkey {
    // TODO: give better error warning
    Pubkey::create_program_address(
        &[b"memory", payer.as_ref(), &[memory_index], &[bump]],
        &crate::ID,
    )
    .unwrap()
}
