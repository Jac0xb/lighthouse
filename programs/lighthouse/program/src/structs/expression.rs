use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum Expression {
    And(Vec<u8>),
    Or(Vec<u8>),
}

impl Expression {
    pub fn contains_assertion_index(&self, index: &u8) -> bool {
        match self {
            Expression::And(assertion_indexes) => assertion_indexes.contains(index),
            Expression::Or(assertion_indexes) => assertion_indexes.contains(index),
        }
    }
}
