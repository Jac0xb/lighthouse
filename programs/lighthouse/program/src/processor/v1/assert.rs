use crate::{
    error::LighthouseError,
    structs::{Assertion, AssertionConfig},
    utils::print_assertion_result,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AssertV1<'info> {
    pub target_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AssertCompactV1<'info> {
    pub target_account: AccountInfo<'info>,
}

pub fn assert(
    target_account: &AccountInfo<'_>,
    assertion: &Assertion,
    config: Option<AssertionConfig>,
) -> Result<()> {
    let include_output = match &config {
        Some(config) => config.verbose,
        None => false,
    };
    let evaluation_result = assertion.evaluate(target_account, include_output)?;

    if include_output {
        msg!("[--] [-] Status | Assertion | Actual Value (Operator) Assertion Value");
        print_assertion_result(assertion, 0, &evaluation_result);
    }

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
