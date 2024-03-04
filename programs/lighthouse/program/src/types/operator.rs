use std::{fmt::Debug, ops::BitAnd};

use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::PrimInt;

use super::LogLevel;

const EQUAL_SYMBOL: &str = "==";
const NOT_EQUAL_SYMBOL: &str = "!=";
const GREATER_THAN_SYMBOL: &str = ">";
const LESS_THAN_SYMBOL: &str = "<";
const GREATER_THAN_OR_EQUAL_SYMBOL: &str = ">=";
const LESS_THAN_OR_EQUAL_SYMBOL: &str = "<=";
const CONTAINS_SYMBOL: &str = "&";
const DOES_NOT_CONTAIN_SYMBOL: &str = "!&";

pub trait Operator<T: ?Sized> {
    fn evaluate(
        &self,
        actual_value: &T,
        assertion_value: &T,
        log_level: &LogLevel,
    ) -> Box<EvaluationResult>;
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
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

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum ComparableOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum EquatableOperator {
    Equal,
    NotEqual,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum BytesOperator {
    Equal,
    NotEqual,
}

pub struct EvaluationResult {
    pub passed: bool,
    pub output: String,
}

impl<T: PartialEq + Eq + PartialOrd + Ord + Debug + Sized> Operator<T> for ComparableOperator {
    fn evaluate(
        &self,
        actual_value: &T,
        assertion_value: &T,
        log_level: &LogLevel,
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
            output: if log_level == &LogLevel::PlaintextLog {
                format!(
                    "{:?} {} {:?}",
                    actual_value,
                    match self {
                        ComparableOperator::Equal => EQUAL_SYMBOL.to_string(),
                        ComparableOperator::NotEqual => NOT_EQUAL_SYMBOL.to_string(),
                        ComparableOperator::GreaterThan => GREATER_THAN_SYMBOL.to_string(),
                        ComparableOperator::LessThan => LESS_THAN_SYMBOL.to_string(),
                        ComparableOperator::GreaterThanOrEqual =>
                            GREATER_THAN_OR_EQUAL_SYMBOL.to_string(),
                        ComparableOperator::LessThanOrEqual =>
                            LESS_THAN_OR_EQUAL_SYMBOL.to_string(),
                    },
                    assertion_value
                )
            } else {
                "".to_string()
            },
        })
    }
}

impl<T: PrimInt + BitAnd + Debug + Eq + Sized> Operator<T> for IntegerOperator {
    fn evaluate(
        &self,
        actual_value: &T,
        assertion_value: &T,
        log_level: &LogLevel,
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
            output: if log_level == &LogLevel::PlaintextLog {
                format!(
                    "{:?} (actual) {} {:?} (expected)",
                    actual_value,
                    match self {
                        IntegerOperator::Equal => EQUAL_SYMBOL.to_string(),
                        IntegerOperator::NotEqual => NOT_EQUAL_SYMBOL.to_string(),
                        IntegerOperator::GreaterThan => GREATER_THAN_SYMBOL.to_string(),
                        IntegerOperator::LessThan => LESS_THAN_SYMBOL.to_string(),
                        IntegerOperator::GreaterThanOrEqual =>
                            GREATER_THAN_OR_EQUAL_SYMBOL.to_string(),
                        IntegerOperator::LessThanOrEqual => LESS_THAN_OR_EQUAL_SYMBOL.to_string(),
                        IntegerOperator::Contains => CONTAINS_SYMBOL.to_string(),
                        IntegerOperator::DoesNotContain => DOES_NOT_CONTAIN_SYMBOL.to_string(),
                    },
                    assertion_value
                )
            } else {
                "".to_string()
            },
        })
    }
}

impl<T: PartialEq + Eq + Debug + Sized> Operator<T> for EquatableOperator {
    fn evaluate(
        &self,
        actual_value: &T,
        assertion_value: &T,
        log_level: &LogLevel,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                EquatableOperator::Equal => T::eq(actual_value, assertion_value),
                EquatableOperator::NotEqual => T::ne(actual_value, assertion_value),
            },
            output: if log_level == &LogLevel::PlaintextLog {
                format!(
                    "{:?} {} {:?}",
                    actual_value,
                    match self {
                        EquatableOperator::Equal => EQUAL_SYMBOL.to_string(),
                        EquatableOperator::NotEqual => NOT_EQUAL_SYMBOL.to_string(),
                    },
                    assertion_value
                )
            } else {
                "".to_string()
            },
        })
    }
}

impl<T> Operator<T> for BytesOperator
where
    T: AsRef<[u8]> + Debug + ?Sized,
{
    fn evaluate(
        &self,
        actual_value: &T,
        assertion_value: &T,
        log_level: &LogLevel,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                BytesOperator::Equal => {
                    let actual_value = actual_value.as_ref();
                    let assertion_value = assertion_value.as_ref();

                    actual_value == assertion_value
                }
                BytesOperator::NotEqual => {
                    let actual_value = actual_value.as_ref();
                    let assertion_value = assertion_value.as_ref();

                    actual_value != assertion_value
                }
            },
            output: if log_level == &LogLevel::PlaintextLog {
                format!(
                    "{:?} {} {:?}",
                    actual_value,
                    match self {
                        BytesOperator::Equal => EQUAL_SYMBOL.to_string(),
                        BytesOperator::NotEqual => NOT_EQUAL_SYMBOL.to_string(),
                    },
                    assertion_value
                )
            } else {
                "".to_string()
            },
        })
    }
}
