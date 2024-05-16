use std::ops::Deref;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::operator::{
    Operator, CONTAINS_SYMBOL, DOES_NOT_CONTAIN_SYMBOL, EQUAL_SYMBOL, GREATER_THAN_OR_EQUAL_SYMBOL,
    GREATER_THAN_SYMBOL, LESS_THAN_OR_EQUAL_SYMBOL, LESS_THAN_SYMBOL, NOT_EQUAL_SYMBOL,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    type Error = std::io::Error;

    fn try_from(value: u8) -> Result<Self, std::io::Error> {
        match value {
            0 => Ok(IntegerOperator::Equal),
            1 => Ok(IntegerOperator::NotEqual),
            2 => Ok(IntegerOperator::GreaterThan),
            3 => Ok(IntegerOperator::LessThan),
            4 => Ok(IntegerOperator::GreaterThanOrEqual),
            5 => Ok(IntegerOperator::LessThanOrEqual),
            6 => Ok(IntegerOperator::Contains),
            7 => Ok(IntegerOperator::DoesNotContain),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid IntegerOperator",
            )),
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

impl Deref for IntegerOperator {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            IntegerOperator::Equal => &0,
            IntegerOperator::NotEqual => &1,
            IntegerOperator::GreaterThan => &2,
            IntegerOperator::LessThan => &3,
            IntegerOperator::GreaterThanOrEqual => &4,
            IntegerOperator::LessThanOrEqual => &5,
            IntegerOperator::Contains => &6,
            IntegerOperator::DoesNotContain => &7,
        }
    }
}
