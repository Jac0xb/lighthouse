use std::fmt::Debug;

use borsh::{BorshDeserialize, BorshSerialize};

pub trait Operator {
    fn evaluate<T: PartialEq + Eq + PartialOrd + Ord + Debug>(
        &self,
        actual_value: &T,
        assertion_value: &T,
        output: bool,
    ) -> Box<EvaluationResult>;
    fn format(&self) -> String;
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
pub enum ComparableOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
pub enum EquatableOperator {
    Equal,
    NotEqual,
}

pub struct EvaluationResult {
    pub passed: bool,
    pub output: String,
}

impl Operator for ComparableOperator {
    fn evaluate<T: PartialEq + Eq + PartialOrd + Ord + Debug>(
        &self,
        actual_value: &T,
        assertion_value: &T,
        output: bool,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                ComparableOperator::Equal => T::eq(actual_value, assertion_value),
                ComparableOperator::NotEqual => T::ne(actual_value, assertion_value),
                ComparableOperator::GreaterThan => T::gt(actual_value, assertion_value),
                ComparableOperator::LessThan => T::lt(actual_value, assertion_value),
                ComparableOperator::GreaterThanOrEqual => T::ge(actual_value, assertion_value),
                ComparableOperator::LessThanOrEqual => T::le(actual_value, assertion_value),
            },
            output: if output {
                format!("{:?} {} {:?}", actual_value, self.format(), assertion_value)
            } else {
                "".to_string()
            },
        })
    }

    fn format(&self) -> String {
        match self {
            ComparableOperator::Equal => "==",
            ComparableOperator::NotEqual => "!=",
            ComparableOperator::GreaterThan => ">",
            ComparableOperator::LessThan => "<",
            ComparableOperator::GreaterThanOrEqual => ">=",
            ComparableOperator::LessThanOrEqual => "<=",
        }
        .to_string()
    }
}

impl Operator for EquatableOperator {
    fn evaluate<T: PartialEq + Eq + Debug>(
        &self,
        actual_value: &T,
        assertion_value: &T,
        output: bool,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                EquatableOperator::Equal => T::eq(actual_value, assertion_value),
                EquatableOperator::NotEqual => T::ne(actual_value, assertion_value),
            },
            output: if output {
                format!("{:?} {} {:?}", actual_value, self.format(), assertion_value)
            } else {
                "".to_string()
            },
        })
    }

    fn format(&self) -> String {
        match self {
            EquatableOperator::Equal => "==",
            EquatableOperator::NotEqual => "!=",
        }
        .to_string()
    }
}
