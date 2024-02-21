use std::{fmt::Debug, ops::BitAnd};

use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::PrimInt;

pub trait Operator<T> {
    fn evaluate(
        &self,
        actual_value: &T,
        assertion_value: &T,
        output: bool,
    ) -> Box<EvaluationResult>;
}

pub trait Format {
    fn format(&self) -> String;
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
pub enum IntegerOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    DoesNotContain,
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

impl Format for ComparableOperator {
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

impl<T: PartialEq + Eq + PartialOrd + Ord + Debug> Operator<T> for ComparableOperator {
    fn evaluate(
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
}

impl<T: PrimInt + BitAnd + Debug + Eq> Operator<T> for IntegerOperator {
    fn evaluate(
        &self,
        actual_value: &T,
        assertion_value: &T,
        output: bool,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                IntegerOperator::Equal => T::eq(actual_value, assertion_value),
                IntegerOperator::NotEqual => T::ne(actual_value, assertion_value),
                IntegerOperator::GreaterThan => T::gt(actual_value, assertion_value),
                IntegerOperator::LessThan => T::lt(actual_value, assertion_value),
                IntegerOperator::GreaterThanOrEqual => T::ge(actual_value, assertion_value),
                IntegerOperator::LessThanOrEqual => T::le(actual_value, assertion_value),
                IntegerOperator::Contains => {
                    let actual_value = *actual_value;
                    let assertion_value = *assertion_value;

                    actual_value & assertion_value == assertion_value
                }
                IntegerOperator::DoesNotContain => {
                    let actual_value = *actual_value;
                    let assertion_value = *assertion_value;

                    actual_value & assertion_value == T::zero()
                }
            },
            output: if output {
                format!(
                    "{:?} (actual) {} {:?} (expected)",
                    actual_value,
                    self.format(),
                    assertion_value
                )
            } else {
                "".to_string()
            },
        })
    }
}

impl Format for IntegerOperator {
    fn format(&self) -> String {
        match self {
            IntegerOperator::Equal => "==",
            IntegerOperator::NotEqual => "!=",
            IntegerOperator::GreaterThan => ">",
            IntegerOperator::LessThan => "<",
            IntegerOperator::GreaterThanOrEqual => ">=",
            IntegerOperator::LessThanOrEqual => "<=",
            IntegerOperator::Contains => "&",
            IntegerOperator::DoesNotContain => "!&",
        }
        .to_string()
    }
}

impl<T: PartialEq + Eq + Debug> Operator<T> for EquatableOperator {
    fn evaluate(
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
}

impl Format for EquatableOperator {
    fn format(&self) -> String {
        match self {
            EquatableOperator::Equal => "==",
            EquatableOperator::NotEqual => "!=",
        }
        .to_string()
    }
}
