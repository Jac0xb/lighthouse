use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt::Debug;

use crate::error::LighthouseError;

const EQUAL_SYMBOL: &str = "==";
const NOT_EQUAL_SYMBOL: &str = "!=";
const GREATER_THAN_SYMBOL: &str = ">";
const LESS_THAN_SYMBOL: &str = "<";
const GREATER_THAN_OR_EQUAL_SYMBOL: &str = ">=";
const LESS_THAN_OR_EQUAL_SYMBOL: &str = "<=";
const CONTAINS_SYMBOL: &str = "&";
const DOES_NOT_CONTAIN_SYMBOL: &str = "!&";

pub trait Operator {
    fn format(&self) -> &str;
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Copy, Clone)]
#[repr(u8)]
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

impl TryFrom<u8> for IntegerOperator {
    type Error = LighthouseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IntegerOperator::Equal),
            1 => Ok(IntegerOperator::NotEqual),
            2 => Ok(IntegerOperator::GreaterThan),
            3 => Ok(IntegerOperator::LessThan),
            4 => Ok(IntegerOperator::GreaterThanOrEqual),
            5 => Ok(IntegerOperator::LessThanOrEqual),
            6 => Ok(IntegerOperator::Contains),
            7 => Ok(IntegerOperator::DoesNotContain),
            _ => Err(LighthouseError::InvalidInstructionData),
        }
    }
}

impl Operator for IntegerOperator {
    fn format(&self) -> &str {
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
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Copy, Clone)]
#[repr(u8)]
pub enum EquatableOperator {
    Equal,
    NotEqual,
}

impl TryFrom<u8> for EquatableOperator {
    type Error = LighthouseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EquatableOperator::Equal),
            1 => Ok(EquatableOperator::NotEqual),
            _ => Err(LighthouseError::InvalidInstructionData),
        }
    }
}

impl Operator for EquatableOperator {
    fn format(&self) -> &str {
        match self {
            EquatableOperator::Equal => EQUAL_SYMBOL,
            EquatableOperator::NotEqual => NOT_EQUAL_SYMBOL,
        }
    }
}
