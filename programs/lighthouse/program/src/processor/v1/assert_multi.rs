use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AssertMultiV1<'info> {
    system_program: Program<'info, System>,
}
