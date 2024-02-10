use anchor_lang::prelude::*;

use crate::{
    error::LighthouseError,
    types::{Assertion, AssertionConfigV1},
    utils::print_assertion_result,
};

#[derive(Accounts)]
pub struct AssertMultiCompactV1<'info> {
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssertMultiV1<'info> {
    system_program: Program<'info, System>,
}

///
///  Remaining account modding for assertion association with remaining accounts
///  Reasoning: there are a few ways to associate assertions with accounts
///  1. You could store the remaining account index in the assertion instruction data (1 byte)
///  2. You could group assertions by account through a vector<vector<assertion> (4 * (4 bytes * unique accounts)), struct (1 byte), or enough
///  3. Sort assertions such that their remainder is equal to the index of the remaining account associated with the assertion (<1 byte of transaction data for best case)
///     - This sorting will be handled by the client and can be as inefficient or as efficient as the client wants
///     - IE a client could pass in a remaining account of [A, A, A, A] or [A] and assertions will be properly associated with the remaining accounts
///
///  Account A, B, C
///
///  remaining accounts = [A]
///  Assertion 1 (A)
///  Assertion 2 (A)
///  Assertion 3 (A)
///  (A [0%1=0], A [1%1=0], A [2%1=0])
///  (1, 2, 3)
///
///  remaining accounts = [A, B]
///  Assertion 1 (A)
///  Assertion 2 (A) <- Reorder to third position
///  Assertion 3 (B)
///  (A [0%2=0], B [1%2=1], A [2%2=0])
///  (1, 3, 2)
///
///  remaining accounts = [A, B, C]
///  Assertion 1 (A)
///  Assertion 2 (B)
///  Assertion 3 (C)
///  (A [0%3=0], B [1%3=1], C [2%3=2])
///  (1, 2, 3)
///
///  remaining accounts = [A, B, A]
///  Assertion 1 (A)
///  Assertion 2 (B)
///  Assertion 4 (A)
///  Assertion 3 (A)
///  (A [0%4=0], B [1%4=1], A [2%3=2], A [3%3=0])
///  (1, 2, 4, 3)
pub fn assert_multi(
    remaining_accounts: &[AccountInfo<'_>],
    assertions: &[Assertion],
    config: Option<AssertionConfigV1>,
) -> Result<()> {
    let include_output = match &config {
        Some(config) => config.verbose,
        None => false,
    };

    if remaining_accounts.is_empty() {
        return Err(LighthouseError::NotEnoughAccounts.into());
    }

    for (assertion_index, assertion) in assertions.iter().enumerate() {
        let evaluation_result = assertion.evaluate(
            &remaining_accounts[assertion_index % remaining_accounts.len()],
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
