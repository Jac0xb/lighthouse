use crate::{types::EvaluationResult, utils::Result};
use borsh::{BorshDeserialize, BorshSerialize};

pub trait Assert<T: core::fmt::Debug> {
    fn evaluate(&self, parameters: &T, log_level: &LogLevel) -> Result<Box<EvaluationResult>>;
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, PartialOrd, Clone)]
#[repr(u8)]
pub enum LogLevel {
    Silent = 0,
    PlaintextLog = 1,
    EventLog = 2,
    CpiLog = 3,
}
