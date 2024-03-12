use super::LogLevel;
use crate::{error::LighthouseError, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::PrimInt;
use solana_program::{msg, program_memory::sol_memcmp};
use std::{fmt::Debug, ops::BitAnd};

const EQUAL_SYMBOL: &str = "==";
const NOT_EQUAL_SYMBOL: &str = "!=";
const GREATER_THAN_SYMBOL: &str = ">";
const LESS_THAN_SYMBOL: &str = "<";
const GREATER_THAN_OR_EQUAL_SYMBOL: &str = ">=";
const LESS_THAN_OR_EQUAL_SYMBOL: &str = "<=";
const CONTAINS_SYMBOL: &str = "&";
const DOES_NOT_CONTAIN_SYMBOL: &str = "!&";

pub trait Operator<T: ?Sized + Debug> {
    fn evaluate(&self, actual_value: &T, assertion_value: &T, log_level: LogLevel) -> Result<()>;
    fn log(&self, actual_value: &T, assertion_value: &T, log_level: LogLevel) {
        if log_level == LogLevel::PlaintextMessage {
            msg!(
                "{:?} {} {:?}",
                actual_value,
                self.format_operator(),
                assertion_value
            );
        }
    }
    fn format_operator(&self) -> &str;
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

impl<T: PartialEq + Eq + PartialOrd + Ord + Debug + Sized> Operator<T> for ComparableOperator {
    fn format_operator(&self) -> &str {
        match self {
            ComparableOperator::Equal => EQUAL_SYMBOL,
            ComparableOperator::NotEqual => NOT_EQUAL_SYMBOL,
            ComparableOperator::GreaterThan => GREATER_THAN_SYMBOL,
            ComparableOperator::LessThan => LESS_THAN_SYMBOL,
            ComparableOperator::GreaterThanOrEqual => GREATER_THAN_OR_EQUAL_SYMBOL,
            ComparableOperator::LessThanOrEqual => LESS_THAN_OR_EQUAL_SYMBOL,
        }
    }

    fn evaluate(&self, actual_value: &T, assertion_value: &T, log_level: LogLevel) -> Result<()> {
        let passed = match self {
            ComparableOperator::Equal => T::eq(actual_value, assertion_value),
            ComparableOperator::NotEqual => T::ne(actual_value, assertion_value),
            ComparableOperator::GreaterThan => T::gt(actual_value, assertion_value),
            ComparableOperator::LessThan => T::lt(actual_value, assertion_value),
            ComparableOperator::GreaterThanOrEqual => T::ge(actual_value, assertion_value),
            ComparableOperator::LessThanOrEqual => T::le(actual_value, assertion_value),
        };

        self.log(actual_value, assertion_value, log_level);

        if passed {
            Ok(())
        } else {
            Err(LighthouseError::AssertionFailed.into())
        }
    }
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

impl<T: PrimInt + BitAnd + Debug + Eq + Sized> Operator<T> for IntegerOperator {
    fn format_operator(&self) -> &str {
        match self {
            IntegerOperator::Equal => EQUAL_SYMBOL,
            IntegerOperator::NotEqual => NOT_EQUAL_SYMBOL,
            IntegerOperator::GreaterThan => GREATER_THAN_SYMBOL,
            IntegerOperator::LessThan => LESS_THAN_SYMBOL,
            IntegerOperator::GreaterThanOrEqual => GREATER_THAN_OR_EQUAL_SYMBOL,
            IntegerOperator::LessThanOrEqual => LESS_THAN_OR_EQUAL_SYMBOL,
            IntegerOperator::Contains => CONTAINS_SYMBOL,
            IntegerOperator::DoesNotContain => DOES_NOT_CONTAIN_SYMBOL,
        }
    }

    fn evaluate(&self, actual_value: &T, assertion_value: &T, log_level: LogLevel) -> Result<()> {
        let passed = match self {
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
        };

        self.log(actual_value, assertion_value, log_level);

        if passed {
            Ok(())
        } else {
            Err(LighthouseError::AssertionFailed.into())
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum EquatableOperator {
    Equal,
    NotEqual,
}

impl<T: PartialEq + Eq + Debug + Sized> Operator<T> for EquatableOperator {
    fn format_operator(&self) -> &str {
        match self {
            EquatableOperator::Equal => EQUAL_SYMBOL,
            EquatableOperator::NotEqual => NOT_EQUAL_SYMBOL,
        }
    }

    fn evaluate(&self, actual_value: &T, assertion_value: &T, log_level: LogLevel) -> Result<()> {
        let passed = match self {
            EquatableOperator::Equal => T::eq(actual_value, assertion_value),
            EquatableOperator::NotEqual => T::ne(actual_value, assertion_value),
        };

        self.log(actual_value, assertion_value, log_level);

        if passed {
            Ok(())
        } else {
            Err(LighthouseError::AssertionFailed.into())
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum ByteSliceOperator {
    Equal,
    NotEqual,
}

impl<T> Operator<T> for ByteSliceOperator
where
    T: AsRef<[u8]> + Debug + ?Sized,
{
    fn format_operator(&self) -> &str {
        match self {
            ByteSliceOperator::Equal => EQUAL_SYMBOL,
            ByteSliceOperator::NotEqual => NOT_EQUAL_SYMBOL,
        }
    }

    fn evaluate(&self, actual_value: &T, assertion_value: &T, log_level: LogLevel) -> Result<()> {
        let passed = match self {
            ByteSliceOperator::Equal => {
                let actual_value = actual_value.as_ref();
                let assertion_value = assertion_value.as_ref();

                if actual_value.len() == assertion_value.len() {
                    sol_memcmp(actual_value, assertion_value, assertion_value.len()) == 0
                } else {
                    false
                }
            }
            ByteSliceOperator::NotEqual => {
                let actual_value = actual_value.as_ref();
                let assertion_value = assertion_value.as_ref();

                if actual_value.len() == assertion_value.len() {
                    sol_memcmp(actual_value, assertion_value, assertion_value.len()) != 0
                } else {
                    true
                }
            }
        };

        self.log(actual_value, assertion_value, log_level);

        if passed {
            Ok(())
        } else {
            Err(LighthouseError::AssertionFailed.into())
        }
    }
}
