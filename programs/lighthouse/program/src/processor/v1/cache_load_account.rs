use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::state::cache::CacheAccount;

#[derive(Accounts)]
#[instruction(cache_index: u8)]
pub struct CacheLoadAccountV1<'info> {
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

    /// CHECK: Is this the correct way to load an account?
    pub source_account: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn cache_load_account<'info>(
    ctx: Context<'_, '_, '_, 'info, CacheLoadAccountV1<'info>>,
    cache_index: u8,
    cache_start: u16,
    dest_start: u16,
    slice_length: u16,
) -> Result<()> {
    // {
    let source_account_data = ctx.accounts.source_account.data.borrow();
    let cache_account_data = &mut ctx.accounts.cache_account.load_mut()?.data;

    msg!(
        "cache_start: {}, dest_start: {}, slice_length: {}",
        cache_start,
        dest_start,
        slice_length
    );

    msg!(
        "cache_account_data.len(): {}, source_account_data.len(): {}",
        cache_account_data.len(),
        source_account_data.len()
    );

    if ((cache_start + slice_length) as usize) < cache_account_data.len() {
        cache_account_data[cache_start as usize..(cache_start + slice_length) as usize]
            .copy_from_slice(
                &source_account_data[dest_start as usize..(dest_start + slice_length) as usize],
            );
    } else {
        // Handle the error: destination slice is not large enough
    }

    Ok(())
}
