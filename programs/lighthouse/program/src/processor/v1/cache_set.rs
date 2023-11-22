use crate::state::cache::CacheAccount;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Accounts)]
#[instruction(cache_index: u8, cache_account_size: u64)]
pub struct CacheSetV1<'info> {
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
    pub cache_account: AccountLoader<'info, CacheAccount>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn cache_set_borsh<'info>(
    ctx: Context<'_, '_, '_, 'info, CacheSetV1<'info>>,
    slice: Vec<u8>,
) -> Result<()> {
    Ok(())
}
