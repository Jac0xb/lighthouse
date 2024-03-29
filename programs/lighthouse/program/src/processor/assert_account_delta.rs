use crate::{
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
    ctx: &AssertAccountDeltaContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    assertion.evaluate((ctx.left_account, ctx.right_account), log_level)
}
