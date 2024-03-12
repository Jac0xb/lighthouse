use crate::{
    types::assert::{Assert, LogLevel},
    utils::Result,
};
use std::fmt::Debug;

pub(crate) fn assert_clock<T: Assert<()> + Debug>(
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    assertion.evaluate((), log_level)
}
