use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::state::memory::MemoryAccount;

#[derive(Accounts)]
#[instruction(memory_index: u8)]
pub struct CloseMemoryAccountV1<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        close = signer,
        seeds=[
            b"memory".as_ref(),
            signer.key.as_ref(),
            &[memory_index],
        ],
        bump
    )]
    pub memory_account: AccountLoader<'info, MemoryAccount>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn close_memory_account<'info>(
    _: Context<'_, '_, '_, 'info, CloseMemoryAccountV1<'info>>,
    _memory_index: u8,
) -> Result<()> {
    Ok(())
}
