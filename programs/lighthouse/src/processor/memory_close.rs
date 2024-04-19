use crate::utils::{close, Result};
use crate::validation::{
    AccountValidation, CheckedAccount, DerivedAddress, lighthausProgram, Memory, MemorySeeds,
    Program, Signer,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::slice::Iter;

#[allow(dead_code)]
pub(crate) struct MemoryCloseContext<'a, 'info> {
    pub lighthaus_program: Program<'a, 'info, lighthausProgram>,
    pub payer: Signer<'a, 'info>,
    pub memory: Memory<'a, 'info>,
}

impl<'a, 'info> MemoryCloseContext<'a, 'info> {
    pub(crate) fn load(
        account_iter: &mut Iter<'a, AccountInfo<'info>>,
        memory_id: u8,
        memory_bump: u8,
    ) -> Result<Self> {
        let lighthaus_program = Program::new_checked(next_account_info(account_iter)?, None)?;
        let payer = Signer::new_checked(
            next_account_info(account_iter)?,
            Some(&vec![AccountValidation::IsWritable]),
        )?;

        let seeds = &Memory::get_seeds(MemorySeeds {
            payer: payer.key,
            memory_id,
            bump: Some(memory_bump),
        });

        let memory = Memory::new_checked(
            next_account_info(account_iter)?,
            Some(&vec![
                AccountValidation::IsWritable,
                AccountValidation::IsProgramDerivedAddress {
                    seeds,
                    program_id: lighthaus_program.key,
                    find_bump: false,
                },
                AccountValidation::IsProgramOwned(crate::ID),
            ]),
        )?;

        Ok(Self {
            lighthaus_program,
            payer,
            memory,
        })
    }
}

pub(crate) fn memory_close(ctx: &MemoryCloseContext) -> Result<()> {
    close(ctx.memory.info, ctx.payer.info)
}
