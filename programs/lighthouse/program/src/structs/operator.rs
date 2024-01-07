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
        actual_value: &T,
        assertion_value: &T,
        output: bool,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                Operator::Equal => T::eq(actual_value, assertion_value),
                Operator::NotEqual => T::ne(actual_value, assertion_value),
                Operator::GreaterThan => T::gt(actual_value, assertion_value),
                Operator::LessThan => T::lt(actual_value, assertion_value),
                Operator::GreaterThanOrEqual => T::ge(actual_value, assertion_value),
                Operator::LessThanOrEqual => T::le(actual_value, assertion_value),
            },
            output: if output {
                format!("{:?} {} {:?}", actual_value, self.format(), assertion_value)
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
