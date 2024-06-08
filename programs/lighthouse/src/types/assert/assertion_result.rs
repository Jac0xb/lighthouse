use super::{EquatableOperator, IntegerOperator, LogLevel};
use crate::{error::LighthouseError, types::assert::Operator, validation::SPL_NOOP_ID, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::Instruction, log::sol_log_data, msg, program::invoke, pubkey::Pubkey,
};
use std::fmt::Debug;

macro_rules! log_cases {
    ($self:ident, $($type:ident),+) => {
        match $self {
            $(
                AssertionResult::$type(actual, assertion, operator, passed) => {
                    let operator = IntegerOperator::try_from(*operator).unwrap();

                    msg!(
                        "Result ({}): {:?} {} {:?}",
                        if *passed { "Passed" } else { "Failed" },
                        actual,
                        operator.format(),
                        assertion,
                    );
                }
            )+
            // You can include special cases directly within the macro or outside as needed
            AssertionResult::Pubkey(actual, assertion, operator, passed) => {
                let operator = EquatableOperator::try_from(*operator).unwrap();

                match (actual, assertion) {
                    (Some(actual_value), Some(assertion_value)) => {
                        msg!("Result: ");
                        actual_value.log();
                        msg!(operator.format());
                        assertion_value.log();
                    }
                    (None, Some(assertion_value)) => {
                        msg!("Result: ");
                        msg!("None");
                        msg!(operator.format());
                        assertion_value.log();
                    }
                    (Some(actual_value), None) => {
                        msg!("Result ({}): ", if *passed { "Passed" } else { "Failed" });
                        actual_value.log();
                        msg!(operator.format());
                        msg!("None");
                    }
                    (None, None) => {
                        msg!("Result: None {} None", operator.format());
                    }
                }
            },
        }
    };
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AssertionResult {
    U8(Option<u8>, Option<u8>, u8, bool),
    U16(Option<u16>, Option<u16>, u8, bool),
    U32(Option<u32>, Option<u32>, u8, bool),
    U64(Option<u64>, Option<u64>, u8, bool),
    U128(Option<u128>, Option<u128>, u8, bool),
    I8(Option<i8>, Option<i8>, u8, bool),
    I16(Option<i16>, Option<i16>, u8, bool),
    I32(Option<i32>, Option<i32>, u8, bool),
    I64(Option<i64>, Option<i64>, u8, bool),
    I128(Option<i128>, Option<i128>, u8, bool),
    Pubkey(Option<Pubkey>, Option<Pubkey>, u8, bool),
    Bytes(Vec<u8>, Vec<u8>, u8, bool),
    Bool(Option<bool>, Option<bool>, u8, bool),
}

impl AssertionResult {
    pub fn log(&self, log_level: LogLevel) -> Result<()> {
        if self.passed() && log_level.ignore_success() {
            return Ok(());
        }

        match log_level {
            LogLevel::Silent => {}
            LogLevel::PlaintextMessage | LogLevel::FailedPlaintextMessage => {
                log_cases!(self, U8, U16, U32, U64, U128, I8, I16, I32, I64, I128, Bool, Bytes);
            }
            LogLevel::EncodedMessage | LogLevel::FailedEncodedMessage => {
                let data = self.try_to_vec().map_err(LighthouseError::serialize_err)?;
                sol_log_data(&[&data]);
            }
            LogLevel::EncodedNoop | LogLevel::FailedEncodedNoop => {
                let data = self.try_to_vec().map_err(LighthouseError::serialize_err)?;

                invoke(
                    &Instruction {
                        program_id: SPL_NOOP_ID,
                        accounts: vec![],
                        data,
                    },
                    &[],
                )?;
            }
        }

        Ok(())
    }

    #[inline(always)]
    pub fn passed(&self) -> bool {
        match self {
            AssertionResult::U8(_, _, _, passed)
            | AssertionResult::U16(_, _, _, passed)
            | AssertionResult::U32(_, _, _, passed)
            | AssertionResult::U64(_, _, _, passed)
            | AssertionResult::U128(_, _, _, passed)
            | AssertionResult::I8(_, _, _, passed)
            | AssertionResult::I16(_, _, _, passed)
            | AssertionResult::I32(_, _, _, passed)
            | AssertionResult::I64(_, _, _, passed)
            | AssertionResult::I128(_, _, _, passed)
            | AssertionResult::Pubkey(_, _, _, passed)
            | AssertionResult::Bytes(_, _, _, passed)
            | AssertionResult::Bool(_, _, _, passed) => *passed,
        }
    }
}
