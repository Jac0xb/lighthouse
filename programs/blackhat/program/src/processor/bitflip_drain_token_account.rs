use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct BitflipDrainTokenAccount<'info> {
    #[account(mut)]
    pub victim: Signer<'info>,
    /// CHECK: ...
    pub bad_actor: UncheckedAccount<'info>,

    /// CHECK: ...
    pub bit_flipper: UncheckedAccount<'info>,

    /// CHECK: ...
    pub mint: Account<'info, Mint>,

    /// CHECK: ...
    #[account(mut)]
    pub victim_ata: Account<'info, TokenAccount>,

    /// CHECK: ...
    pub bad_actor_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
