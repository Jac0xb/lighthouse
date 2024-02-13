#![allow(clippy::result_large_err)]
#![allow(clippy::too_many_arguments)]

use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

pub mod error;
pub mod processor;
pub mod state;
pub mod types;
pub mod utils;

use crate::{processor::*, types::*};

declare_id!("L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK");

#[program]
pub mod lighthouse {
    use super::*;

    pub fn create_memory_account_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateMemoryAccountV1<'info>>,
        memory_index: u8,
        memory_account_size: u64,
    ) -> Result<()> {
        processor::v1::create_memory_account(ctx, memory_index, memory_account_size)
    }

    pub fn close_memory_account_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, CloseMemoryAccountV1<'info>>,
        memory_index: u8,
        memory_bump: u8,
    ) -> Result<()> {
        processor::v1::close_memory_account(ctx)
    }

    pub fn write_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, WriteV1<'info>>,
        memory_index: u8,
        memory_account_bump: u8,
        write_type: WriteTypeParameter,
    ) -> Result<()> {
        processor::v1::write(ctx, memory_index, memory_account_bump, write_type)
    }

    pub fn assert_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, AssertV1<'info>>,
        assertion: Assertion,
        config: Option<AssertionConfigV1>,
    ) -> Result<()> {
        processor::v1::assert(&ctx.accounts.target_account, &assertion, config)
    }

    pub fn assert_compact_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, AssertCompactV1<'info>>,
        assertion: Assertion,
    ) -> Result<()> {
        processor::v1::assert(&ctx.accounts.target_account, &assertion, None)
    }

    pub fn assert_multi_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, AssertMultiV1<'info>>,
        assertions: Vec<Assertion>,
        config: Option<AssertionConfigV1>,
    ) -> Result<()> {
        processor::v1::assert_multi(ctx.remaining_accounts, assertions.as_slice(), config)
    }

    pub fn assert_multi_compact_v1<'info>(
        ctx: Context<'_, '_, '_, 'info, AssertMultiCompactV1<'info>>,
        assertions: CompactAssertionArray,
    ) -> Result<()> {
        let assertions: &[Assertion] = match &assertions {
            CompactAssertionArray::Size1(a) => a,
            CompactAssertionArray::Size2(a) => a,
            CompactAssertionArray::Size3(a) => a,
            CompactAssertionArray::Size4(a) => a,
            CompactAssertionArray::Size5(a) => a,
            CompactAssertionArray::Size6(a) => a,
            CompactAssertionArray::Size7(a) => a,
            CompactAssertionArray::Size8(a) => a,
            CompactAssertionArray::Size9(a) => a,
            CompactAssertionArray::Size10(a) => a,
            CompactAssertionArray::Size11(a) => a,
            CompactAssertionArray::Size12(a) => a,
            CompactAssertionArray::Size13(a) => a,
            CompactAssertionArray::Size14(a) => a,
            CompactAssertionArray::Size15(a) => a,
            CompactAssertionArray::Size16(a) => a,
        };

        processor::v1::assert_multi(ctx.remaining_accounts, assertions, None)
    }
}
