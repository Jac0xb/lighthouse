use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct DrainTokenAccount<'info> {
    pub victim: Signer<'info>,
    pub bad_actor: UncheckedAccount<'info>,

    pub mint: AccountInfo<'info>,

    #[account(mut)]
    pub victim_ata: AccountInfo<'info>,
    #[account(mut)]
    pub bad_actor_ata: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
