use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DrainAccount<'info> {
    #[account(mut)]
    pub victim: Signer<'info>,

    #[account(mut)]
    pub bad_actor: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
