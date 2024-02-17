use std::slice::Iter;

use crate::{
    error::LighthouseError,
    types::{Assertion, AssertionConfigV1}, // utils::print_assertion_result,
    utils::{print_assertion_result, Result},
    validations::Program,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
};

pub(crate) struct AssertContext<'a, 'info> {
    pub(crate) lighthouse_program: Program<'a, 'info>,
    pub(crate) target_account: AccountInfo<'info>,
}

impl<'a, 'info> AssertContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            lighthouse_program: Program::new(next_account_info(account_iter)?, &crate::id())?,
            target_account: next_account_info(account_iter)?.clone(),
        })
    }
}

pub(crate) fn assert(
    assert_context: AssertContext,
    assertion: &Assertion,
    config: Option<AssertionConfigV1>,
) -> Result<()> {
    let include_output = match &config {
        Some(config) => config.verbose,
        None => false,
    };
    let evaluation_result = assertion.evaluate(&assert_context.target_account, include_output)?;

    if include_output {
        msg!("[--] [-] Status | Assertion | Actual Value (Operator) Assertion Value");
        print_assertion_result(assertion, 0, &evaluation_result);
    }

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
