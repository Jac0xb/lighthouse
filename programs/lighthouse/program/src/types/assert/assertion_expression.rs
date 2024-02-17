use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AssertionExpression {
    And(Vec<u8>),
    Or(Vec<u8>),
}

impl AssertionExpression {
    pub fn contains_assertion_index(&self, index: &u8) -> bool {
        match self {
            AssertionExpression::And(assertion_indexes) => assertion_indexes.contains(index),
            AssertionExpression::Or(assertion_indexes) => assertion_indexes.contains(index),
        }
    }
}
