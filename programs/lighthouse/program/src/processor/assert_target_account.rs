use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::Result,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::{fmt::Debug, slice::Iter};

pub(crate) struct AssertTargetAccountContext<'info> {
    pub(crate) target_account: AccountInfo<'info>,
}

impl<'info> AssertTargetAccountContext<'info> {
    pub(crate) fn load(account_iter: &mut Iter<AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            target_account: next_account_info(account_iter)?.clone(),
        })
    }
}

pub(crate) fn assert_target_account<'info, T: Assert<AccountInfo<'info>> + Debug>(
    assert_context: &AssertTargetAccountContext<'info>,
    assertion: &T,
    log_level: &LogLevel,
) -> Result<()> {
    let evaluation_result = assertion.evaluate(&assert_context.target_account, log_level)?;

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
