use crate::utils::{close, Result};
use crate::validation::{
    AccountValidation, CheckedAccount, DerivedAddress, LighthouseProgram, MemoryAccount,
    MemoryAccountSeeds, Program, Signer,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::slice::Iter;

#[allow(dead_code)]
pub(crate) struct MemoryCloseContext<'a, 'info> {
    pub lighthouse_program: Program<'a, 'info, LighthouseProgram>,
    pub payer: Signer<'a, 'info>,
    pub memory_account: MemoryAccount<'a, 'info>,
}

impl<'a, 'info> MemoryCloseContext<'a, 'info> {
    pub(crate) fn load(
        account_iter: &mut Iter<'a, AccountInfo<'info>>,
        memory_index: u8,
        memory_account_bump: u8,
    ) -> Result<Self> {
        let lighthouse_program = Program::new_checked(next_account_info(account_iter)?, None)?;
        let payer = Signer::new_checked(next_account_info(account_iter)?, None)?;

        let seeds = &MemoryAccount::get_seeds(MemoryAccountSeeds {
            payer: payer.key,
            memory_index,
            bump: Some(memory_account_bump),
        });

        let memory_account = MemoryAccount::new_checked(
            next_account_info(account_iter)?,
            Some(&vec![
                AccountValidation::IsWritable,
                AccountValidation::IsProgramDerivedAddress {
                    seeds,
                    program_id: lighthouse_program.key,
                    find_bump: false,
                },
                AccountValidation::IsProgramOwned(crate::ID),
            ]),
        )?;

        Ok(Self {
            lighthouse_program,
            payer,
            memory_account,
        })
    }
}

pub(crate) fn memory_close(context: &MemoryCloseContext) -> Result<()> {
    close(
        context.memory_account.info_as_owned(),
        context.payer.info_as_owned(),
    )?;

    Ok(())
}
