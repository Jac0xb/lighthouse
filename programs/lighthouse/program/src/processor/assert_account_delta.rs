use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    Result,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::slice::Iter;

pub(crate) struct AssertAccountDeltaContext<'a, 'info> {
    pub(crate) left_account: &'a AccountInfo<'info>,
    pub(crate) right_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> AssertAccountDeltaContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            left_account: next_account_info(account_iter)?,
            right_account: next_account_info(account_iter)?,
        })
    }
}

pub(crate) fn assert_account_delta<
    'a,
    'info,
    T: Assert<(&'a AccountInfo<'info>, &'a AccountInfo<'info>)>,
>(
    assert_context: &AssertAccountDeltaContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    let evaluation_result = assertion.evaluate(
        (assert_context.left_account, assert_context.right_account),
        log_level,
    )?;
    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
