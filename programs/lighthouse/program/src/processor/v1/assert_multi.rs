use std::slice::Iter;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
};

use crate::{
    error::LighthouseError,
    types::{Assertion, AssertionConfigV1},
    utils::{print_assertion_result, Result},
    validations::Program,
};

pub(crate) struct AssertMultiContext<'a, 'info> {
    pub(crate) lighthouse_program: Program<'a, 'info>,
    pub(crate) remaining_accounts: Vec<AccountInfo<'info>>,
}

impl<'a, 'info> AssertMultiContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        Ok(Self {
            lighthouse_program: Program::new(next_account_info(account_iter)?, &crate::id())?,
            remaining_accounts: account_iter.cloned().collect(),
        })
    }
}

pub(crate) fn assert_multi(
    context: AssertMultiContext,
    assertions: &[Assertion],
    config: Option<AssertionConfigV1>,
) -> Result<()> {
    let include_output = match &config {
        Some(config) => config.verbose,
        None => false,
    };

    if context.remaining_accounts.is_empty() {
        return Err(LighthouseError::NotEnoughAccounts.into());
    }

    for (assertion_index, assertion) in assertions.iter().enumerate() {
        let evaluation_result = assertion.evaluate(
            &context.remaining_accounts[assertion_index % context.remaining_accounts.len()],
            include_output,
        )?;

        if include_output {
            if assertion_index == 0 {
                msg!("[--] [-] Status | Assertion | Actual Value (Operator) Assertion Value");
            }

            print_assertion_result(assertion, assertion_index, &evaluation_result);
        }

        if !evaluation_result.passed {
            return Err(LighthouseError::AssertionFailed.into());
        }
    }

    Ok(())
}
