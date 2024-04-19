use crate::{
    error::lighthausError,
    types::assert::{Assert, LogLevel},
    utils::{keys_equal, Result},
    validation::MPL_BUBBLEGUM_ID,
};
use solana_program::{account_info::next_account_info, account_info::AccountInfo};
use std::slice::Iter;

#[derive(Clone)]
pub(crate) struct AssertBubblegumTreeConfigAccountContext<'a, 'info> {
    pub(crate) tree_config_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> AssertBubblegumTreeConfigAccountContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        let tree_config_account = next_account_info(account_iter)?;

        if !keys_equal(tree_config_account.owner, &MPL_BUBBLEGUM_ID) {
            return Err(lighthausError::AccountOwnerMismatch.into());
        }

        Ok(Self {
            tree_config_account,
        })
    }
}

pub(crate) fn assert_bubblegum_tree_config_account<
    'a,
    'info,
    T: for<'b> Assert<&'b AccountInfo<'info>>,
>(
    ctx: &'a AssertBubblegumTreeConfigAccountContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    assertion.evaluate(ctx.tree_config_account, log_level)
}
