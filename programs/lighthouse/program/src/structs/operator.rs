use std::fmt::Debug;

use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

pub struct EvaluationResult {
    pub passed: bool,
    pub output: String,
}

impl Operator {
    pub fn evaluate<T: PartialEq + Eq + PartialOrd + Ord + Debug>(
        &self,
        actual: &T,
        expected: &T,
        output: bool,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                Operator::Equal => T::eq(actual, expected),
                Operator::NotEqual => T::ne(actual, expected),
                Operator::GreaterThan => T::gt(actual, expected),
                Operator::LessThan => T::lt(actual, expected),
                Operator::GreaterThanOrEqual => T::ge(actual, expected),
                Operator::LessThanOrEqual => T::le(actual, expected),
            },
            output: if output {
                format!("{:?} {:?} {:?}", actual, self, expected)
            } else {
                "".to_string()
            },
        })
    }

    pub fn format(&self) -> String {
        match self {
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThanOrEqual => "<=",
        }
        .to_string()
    }
}
