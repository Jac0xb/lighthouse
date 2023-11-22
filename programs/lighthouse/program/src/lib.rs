#![allow(clippy::result_large_err)]
#![allow(clippy::too_many_arguments)]

use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

pub mod error;
pub mod processor;
pub mod state;
pub mod structs;
pub mod utils;

// pub use processor::*;

use crate::{
    // error::*,
    processor::v1::*,
    // state::*,
    structs::*,
    // utils::*,
    // processor::v1
};

declare_id!("4zzn1WUXHS9wDKhgWHwkFrHdq6VUpBH55oEL4iqVNLUo");

#[program]
pub mod lighthouse {

    use super::*;

    // pub fn default<'info>(
    //     _program_id: &Pubkey,
    //     _accounts: &[AccountInfo<'info>],
    //     _data: &[u8],
    // ) -> Result<()> {
    //     Err(ProgramError::Custom(1234).into())
    // }

    pub fn create_cache_account_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateCacheAccountV1<'info>>,
        cache_index: u8,
        cache_account_size: u64,
    ) -> Result<()> {
        processor::v1::create_cache_account(ctx)
    }

    pub fn create_test_account_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateTestAccountV1<'info>>,
    ) -> Result<()> {
        processor::v1::create_test_account(ctx)
    }

    // pub fn cache_set_v1<'info>(
    //     ctx: Context<'_, '_, '_, 'info, CacheSetV1<'info>>,
    //     slice: Vec<u8>,
    // ) -> Result<()> {
    //     processor::v1::cache_set(ctx, slice)
    // }

    pub fn cache_load_account_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, CacheLoadAccountV1<'info>>,
        cache_index: u8,
        cache_start: u16,
        dest_start: u16,
        slice_length: u16,
    ) -> Result<()> {
        processor::v1::cache_load_account(ctx, cache_index, cache_start, dest_start, slice_length)
    }

    pub fn assert_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, AssertV1<'info>>,
        assertions: Vec<Assertion>,
        logical_expression: Option<Vec<Expression>>,
        // options: Option<Config>,
    ) -> Result<()> {
        processor::assert(ctx, assertions, logical_expression, None)
    }
}
