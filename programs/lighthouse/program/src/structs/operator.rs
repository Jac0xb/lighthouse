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

impl Operator {
    pub fn is_true<T: PartialEq + Eq + PartialOrd + Ord>(
        self,
        value: &T,
        expected_value: &T,
    ) -> bool {
        match self {
            Operator::Equal => T::eq(value, expected_value),
            Operator::NotEqual => T::ne(value, expected_value),
            Operator::GreaterThan => T::gt(value, expected_value),
            Operator::LessThan => T::lt(value, expected_value),
            Operator::GreaterThanOrEqual => T::ge(value, expected_value),
            Operator::LessThanOrEqual => T::le(value, expected_value),
        }
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
