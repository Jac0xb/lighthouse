use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

use super::borsh_field::BorshField;

// TODO: probably worth creating a macro that permeates all these size variants so
// sdk can optimize space. Need to make sure its smaller than 256 variants though
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum WriteType {
    AccountBalanceU8(u8),
    AccountBalanceU16(u16),
    AccountBalanceU32(u32),

    // CacheOffset, AccountOffset, Length
    AccountDataU8(u8, u8, u8),
    AccountDataU16(u16, u16, u16),
    AccountDataU32(u32, u32, u32),

    // CacheOffset
    AccountInfoU8(u8),
    AccountInfoU16(u16),
    AccountInfoU32(u32),

    // TODO:

    // CacheOffset, BorshField
    BorshFieldU8(u8, BorshField),
    BorshFieldU16(u16, BorshField),

    //
    MintAccount,
    TokenAccount(u16),
    TokenAccountOwner(u16),
    TokenAccountBalance(u16),
    // Program Account Assertions

    // Always add variants to the end of this enum to avoid messing with indexers.
}
