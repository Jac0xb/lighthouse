use crate::structs::{Assertion, AssertionConfig, AssertionState, Expression};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AssertExpressionV1<'info> {
    system_program: Program<'info, System>,
}

// WIP
pub fn assert_expression<'info>(
    ctx: Context<'_, '_, '_, 'info, AssertExpressionV1<'info>>,
    assertions: Vec<Assertion>,
    logical_expression: Vec<Expression>,
    config: Option<AssertionConfig>,
) -> Result<()> {
    let remaining_accounts = ctx.remaining_accounts;
    let mut assertion_state = AssertionState::new(assertions.clone(), logical_expression)?;
    let include_input = config.map_or(false, |config| config.verbose);

    for (i, assertion) in assertions.into_iter().enumerate() {
        let assertion_result = assertion.evaluate(&remaining_accounts[i], include_input)?;
        assertion_state.record_result(i, assertion_result.passed)?;
    }

    msg!("assertion_state: {:?}", assertion_state);
    assertion_state.evaluate()?;

    Ok(())
}

pub fn truncate_pubkey(pubkey: &Pubkey) -> String {
    let mut pubkey_str = pubkey.to_string();
    pubkey_str.truncate(5);
    pubkey_str.push_str("...");

    pubkey_str
}
