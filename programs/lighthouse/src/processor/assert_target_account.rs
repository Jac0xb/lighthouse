use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::Result,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::{fmt::Debug, slice::Iter};

#[derive(Clone)]
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
    ctx: AssertTargetAccountContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    assertion.evaluate(ctx.target_account, log_level)
}

pub(crate) fn assert_target_account_multi<'a, 'info, T: Assert<&'a AccountInfo<'info>> + Debug>(
    ctx: AssertTargetAccountContext<'a, 'info>,
    assertions: &[T],
    log_level: LogLevel,
) -> Result<()> {
    for (i, assertion) in assertions.iter().enumerate() {
        assertion
            .evaluate(ctx.target_account, log_level)
            .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
    }

    Ok(())
}
