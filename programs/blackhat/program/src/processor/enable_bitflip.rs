use anchor_lang::prelude::*;

use crate::state::BitFlipper;

#[derive(Accounts)]
#[instruction(pda_bytes: [u8; 32])]
pub struct EnableBitflip<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space = 8,
        payer=signer,
    )]
    pub bit_fipper: Account<'info, BitFlipper>,

    /// CHECK: ...
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
