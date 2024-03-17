use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::{keys_equal, Result},
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::{fmt::Debug, slice::Iter};

#[derive(Clone)]
pub(crate) struct AssertMintAccountContext<'a, 'info> {
    pub(crate) mint_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> AssertMintAccountContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        let mint_account = next_account_info(account_iter)?;

        if !keys_equal(mint_account.owner, &spl_token::ID)
            && !keys_equal(mint_account.owner, &spl_token_2022::ID)
        {
            return Err(LighthouseError::AccountOwnerMismatch.into());
        }

        Ok(Self { mint_account })
    }
}

pub(crate) fn assert_mint_account<'a, 'info, T: Assert<&'a AccountInfo<'info>> + Debug>(
    ctx: AssertMintAccountContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    assertion.evaluate(ctx.mint_account, log_level)
}

pub(crate) fn assert_mint_account_multi<'a, 'info, T: Assert<&'a AccountInfo<'info>> + Debug>(
    ctx: AssertMintAccountContext<'a, 'info>,
    assertions: &[T],
    log_level: LogLevel,
) -> Result<()> {
    for (i, assertion) in assertions.iter().enumerate() {
        assertion
            .evaluate(ctx.mint_account, log_level)
            .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
    }

    Ok(())
}
