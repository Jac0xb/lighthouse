use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::Result,
};
use solana_program::{clock::Clock, sysvar::Sysvar};
use std::fmt::Debug;

pub(crate) fn assert_clock<T: Assert<Clock> + Debug>(
    assertion: &T,
    log_level: &LogLevel,
) -> Result<()> {
    let evaluation_result = assertion.evaluate(&Clock::get()?, log_level)?;

    evaluation_result.log(log_level, assertion);

    if !evaluation_result.passed {
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
