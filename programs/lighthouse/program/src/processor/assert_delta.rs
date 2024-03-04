use crate::{
    error::LighthouseError,
    types::{Assert, LogLevel},
    utils::print_assertion_result,
    utils::Result,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::{fmt::Debug, slice::Iter};

pub(crate) struct AssertDeltaContext<'info> {
    pub(crate) left_account: AccountInfo<'info>,
    pub(crate) right_account: AccountInfo<'info>,
}

impl<'info> AssertDeltaContext<'info> {
    pub(crate) fn load(account_iter: &mut Iter<AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            left_account: next_account_info(account_iter)?.clone(),
            right_account: next_account_info(account_iter)?.clone(),
        })
    }
}

pub(crate) fn assert_delta<'info, T: Assert<(AccountInfo<'info>, AccountInfo<'info>)> + Debug>(
    assert_context: &AssertDeltaContext<'info>,
    assertion: &T,
    log_level: &LogLevel,
) -> Result<()> {
    let evaluation_result = assertion.evaluate(
        &(
            assert_context.left_account.clone(),
            assert_context.right_account.clone(),
        ),
        log_level,
    )?;

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
