use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::state::cache::CacheAccount;

#[derive(Accounts)]
#[instruction(cache_index: u8, cache_account_size: u64)]
pub struct CreateCacheAccountV1<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init, 
        seeds=[
            b"cache".as_ref(),
            signer.key.as_ref(),
            &[cache_index],
        ],
        bump, 
        payer=signer, 
        space= 8 + cache_account_size as usize
    )]
    pub cache_account: AccountLoader<'info, CacheAccount>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_cache_account<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateCacheAccountV1<'info>>,
    _cache_index: u8,
    _cache_account_size: u64,

) -> Result<()> {
    let _ = &mut ctx.accounts.cache_account.load_init()?;

    Ok(())
}
