use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum Expression {
    And(Vec<u8>),
    Or(Vec<u8>),
}
