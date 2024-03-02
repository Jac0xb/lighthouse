use std::{fmt::Debug, slice::Iter};

use crate::{
    error::LighthouseError,
    types::{Assert, AssertionConfigV1},
    utils::print_assertion_result,
    utils::Result,
};
use solana_program::account_info::{next_account_info, AccountInfo};

pub(crate) struct AssertWithAccountContext<'info> {
    pub(crate) target_account: AccountInfo<'info>,
}

impl<'info> AssertWithAccountContext<'info> {
    pub(crate) fn load(account_iter: &mut Iter<AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            target_account: next_account_info(account_iter)?.clone(),
        })
    }
}

pub(crate) fn assert_with_account<'info, T: Assert<AccountInfo<'info>> + Debug>(
    assert_context: &AssertWithAccountContext<'info>,
    assertion: &T,
    config: Option<AssertionConfigV1>,
) -> Result<()> {
    let include_output = match &config {
        Some(config) => config.verbose,
        None => false,
    };
    let evaluation_result = assertion.evaluate(&assert_context.target_account, include_output)?;

    if include_output {
        print_assertion_result(assertion, 0, &evaluation_result);
    }

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
