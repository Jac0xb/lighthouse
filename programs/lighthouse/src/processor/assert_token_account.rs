use crate::{
    error::lighthausError,
    types::assert::{Assert, LogLevel},
    utils::{keys_equal, Result},
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::{fmt::Debug, slice::Iter};

#[derive(Clone)]
pub(crate) struct AssertTokenAccountContext<'a, 'info> {
    pub(crate) token_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> AssertTokenAccountContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        let token_account = next_account_info(account_iter)?;

        if !keys_equal(token_account.owner, &spl_token::ID)
            && !keys_equal(token_account.owner, &spl_token_2022::ID)
        {
            return Err(lighthausError::AccountOwnerMismatch.into());
        }

        Ok(Self { token_account })
    }
}

pub(crate) fn assert_token_account<'a, 'info, T: Assert<&'a AccountInfo<'info>> + Debug>(
    ctx: AssertTokenAccountContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    assertion.evaluate(ctx.token_account, log_level)
}

pub(crate) fn assert_token_account_multi<'a, 'info, T: Assert<&'a AccountInfo<'info>> + Debug>(
    ctx: AssertTokenAccountContext<'a, 'info>,
    assertions: &[T],
    log_level: LogLevel,
) -> Result<()> {
    for (i, assertion) in assertions.iter().enumerate() {
        assertion
            .evaluate(ctx.token_account, log_level)
            .map_err(|e| lighthausError::map_multi_err(e, i as u32))?;
    }

    Ok(())
}
