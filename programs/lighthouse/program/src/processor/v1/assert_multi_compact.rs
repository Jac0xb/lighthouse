use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AssertMultiCompactV1<'info> {
    system_program: Program<'info, System>,
}
