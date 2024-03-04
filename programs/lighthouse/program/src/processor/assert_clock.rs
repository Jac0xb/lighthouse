use crate::{
    error::LighthouseError,
    types::{Assert, LogLevel},
    utils::{print_assertion_result, Result},
};
use solana_program::{clock::Clock, log, sysvar::Sysvar};
use std::fmt::Debug;

pub(crate) fn assert_clock<T: Assert<Clock> + Debug>(
    assertion: &T,
    log_level: &LogLevel,
) -> Result<()> {
    let evaluation_result = assertion.evaluate(&Clock::get()?, log_level)?;

    // if include_output {
    //     print_assertion_result(assertion, 0, &evaluation_result);
    // }

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
