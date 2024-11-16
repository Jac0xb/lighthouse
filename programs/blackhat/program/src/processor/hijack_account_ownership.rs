use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct HijackAccountOwnership<'info> {
    /// CHECK: IM A BAD ACTOR
    #[account(mut)]
    pub victim: Signer<'info>,

    /// CHECK: IM A BAD ACTOR
    pub system_program: AccountInfo<'info>,

    /// CHECK: IM A BAD ACTOR
    #[account(constraint = program.key == &crate::id())]
    pub program: AccountInfo<'info>,
}
