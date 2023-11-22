use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::state::cache::CacheAccount;

#[derive(Accounts)]
#[instruction(cache_index: u8, cache_account_size: u64)]
pub struct CreateCacheAccountV1<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // #[account(
    //     init,
    //     seeds=[
    //         b"cache".as_ref(),
    //         signer.key.as_ref(),
    //         &[cache_index],
    //     ],
    //     space=8 + 8 + cache_account_size as usize,
    //     payer=signer,
    //     bump
    // )]
    #[account(
        init, 
        seeds=[
            b"cache".as_ref(),
            signer.key.as_ref(),
            &[cache_index],
        ],
        bump, 
        payer=signer, 
        space= 8 + 1024
    )]
    pub cache_account: AccountLoader<'info, CacheAccount>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_cache_account<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateCacheAccountV1<'info>>,
    // cache_index: u8,
    // cache_account_size: u64,
) -> Result<()> {
    let cache_account = &mut ctx.accounts.cache_account.load_init()?;
    msg!("Account data size: {}", cache_account.data.len());

    Ok(())
}
