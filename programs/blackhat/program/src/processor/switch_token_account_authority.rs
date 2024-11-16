use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SwitchTokenAccountAuthority<'info> {
    /// CHECK: IM A BAD ACTOR
    #[account(mut)]
    pub token_program_owned_account: AccountInfo<'info>,

    /// CHECK: IM A BAD ACTOR
    pub current_authority: AccountInfo<'info>,

    /// CHECK: IM A BAD ACTOR
    pub token_program: AccountInfo<'info>,
}
