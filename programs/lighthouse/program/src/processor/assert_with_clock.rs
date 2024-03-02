use crate::{
    error::LighthouseError,
    types::{Assert, AssertionConfigV1},
    utils::print_assertion_result,
    utils::Result,
};
use solana_program::{clock::Clock, sysvar::Sysvar};
use std::fmt::Debug;

pub(crate) fn assert_with_clock<T: Assert<Clock> + Debug>(
    assertion: &T,
    config: Option<AssertionConfigV1>,
) -> Result<()> {
    let include_output = match &config {
        Some(config) => config.verbose,
        None => false,
    };
    let evaluation_result = assertion.evaluate(&Clock::get()?, include_output)?;

    if include_output {
        print_assertion_result(assertion, 0, &evaluation_result);
    }

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
