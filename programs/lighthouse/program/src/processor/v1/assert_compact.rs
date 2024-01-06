use crate::{error::ProgramError, structs::Assertion};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AssertCompactV1<'info> {
    target_account: AccountInfo<'info>,
}

pub fn assert_compact<'info>(
    ctx: Context<'_, '_, '_, 'info, AssertCompactV1<'info>>,
    assertion: Assertion,
) -> Result<()> {
    let evaluation_result = assertion.evaluate(&ctx.accounts.target_account, false)?;

    if !evaluation_result.passed {
        return Err(ProgramError::AssertionFailed.into());
    }

    Ok(())
}
