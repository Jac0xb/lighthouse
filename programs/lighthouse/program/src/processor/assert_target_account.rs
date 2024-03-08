use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::Result,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::{fmt::Debug, slice::Iter};

pub(crate) struct AssertTargetAccountContext<'a, 'info> {
    pub(crate) target_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> AssertTargetAccountContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            target_account: next_account_info(account_iter)?,
        })
    }
}

pub(crate) fn assert_target_account<'a, 'info, T: Assert<&'a AccountInfo<'info>> + Debug>(
    assert_context: AssertTargetAccountContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    let evaluation_result = assertion.evaluate(assert_context.target_account, log_level.clone())?;
    if !evaluation_result.passed {
        evaluation_result.log(log_level);
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
