use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct DrainTokenAccount<'info> {
    pub victim: Signer<'info>,
    /// CHECK: ...
    pub bad_actor: UncheckedAccount<'info>,

    /// CHECK: ...
    pub mint: AccountInfo<'info>,

    /// CHECK: ...
    #[account(mut)]
    pub victim_ata: AccountInfo<'info>,

    /// CHECK: ...
    #[account(mut)]
    pub bad_actor_ata: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
