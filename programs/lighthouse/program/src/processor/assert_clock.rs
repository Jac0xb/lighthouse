use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::Result,
};
use std::fmt::Debug;

pub(crate) fn assert_clock<T: Assert<()> + Debug>(
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    let evaluation_result = assertion.evaluate((), log_level)?;
    if !evaluation_result.passed {
        evaluation_result.log(log_level);
        return Err(LighthouseError::AssertionFailed.into());
    }

    Ok(())
}
