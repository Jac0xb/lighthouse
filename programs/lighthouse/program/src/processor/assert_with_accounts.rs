use crate::{
    error::LighthouseError,
    types::{Assert, AssertionConfigV1},
    utils::print_assertion_result,
    utils::Result,
};
use solana_program::account_info::{next_account_info, AccountInfo};
use std::{fmt::Debug, slice::Iter};

pub(crate) struct AssertWithAccountsContext<'info> {
    pub(crate) left_account: AccountInfo<'info>,
    pub(crate) right_account: AccountInfo<'info>,
}

impl<'info> AssertWithAccountsContext<'info> {
    pub(crate) fn load(account_iter: &mut Iter<AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            left_account: next_account_info(account_iter)?.clone(),
            right_account: next_account_info(account_iter)?.clone(),
        })
    }
}

pub(crate) fn assert_with_accounts<
    'info,
    T: Assert<(AccountInfo<'info>, AccountInfo<'info>)> + Debug,
>(
    assert_context: &AssertWithAccountsContext<'info>,
    assertion: &T,
    config: Option<AssertionConfigV1>,
) -> Result<()> {
    let include_output = match &config {
        Some(config) => config.verbose,
        None => false,
    };
    let evaluation_result = assertion.evaluate(
        &(
            assert_context.left_account.clone(),
            assert_context.right_account.clone(),
        ),
        include_output,
    )?;

    if include_output {
        print_assertion_result(assertion, 0, &evaluation_result);
    }

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
