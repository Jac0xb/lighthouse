use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::state::memory::MemoryAccount;

#[derive(Accounts)]
#[instruction(memory_index: u8, memory_account_size: u64)]
pub struct CreateMemoryAccountV1<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init, 
        seeds=[
            b"memory".as_ref(),
            signer.key.as_ref(),
            &[memory_index],
        ],
        bump, 
        payer=signer, 
        space= 8 + memory_account_size as usize
    )]
    pub memory_account: AccountLoader<'info, MemoryAccount>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_memory_account<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateMemoryAccountV1<'info>>,
    _memory_index: u8,
    _memory_account_size: u64,

) -> Result<()> {
    let _ = &mut ctx.accounts.memory_account.load_init()?;

    Ok(())
}
