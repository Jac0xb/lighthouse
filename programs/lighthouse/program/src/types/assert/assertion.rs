#![allow(non_snake_case)]

use crate::{types::EvaluationResult, utils::Result};
use borsh::{BorshDeserialize, BorshSerialize};

pub trait Assert<T: core::fmt::Debug> {
    fn evaluate(&self, parameters: &T, include_output: bool) -> Result<Box<EvaluationResult>>;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AssertionConfigV1 {
    pub verbose: bool,
}
