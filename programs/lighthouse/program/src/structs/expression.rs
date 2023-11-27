use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum Expression {
    And(Vec<u8>),
    Or(Vec<u8>),
}
