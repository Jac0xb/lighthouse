use super::{EquatableOperator, LogLevel};
use crate::{error::LighthouseError, types::assert::Operator, validation::SPL_NOOP_ID, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::Instruction, log::sol_log_data, msg, program::invoke, pubkey::Pubkey,
};
use std::fmt::Debug;

macro_rules! log_case {
    ($variant:ident, $actual:ident, $assertion:ident, $operator:ident, $passed:ident) => {
        msg!(
            "Result ({}): {:?} {} {:?}",
            if *$passed { "Passed" } else { "Failed" },
            $actual,
            $operator,
            $assertion,
        );
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
        match log_level {
            LogLevel::Silent => {}
            LogLevel::PlaintextMessage => {
                self.log_msg();
            }
            LogLevel::EncodedMessage => {
                self.log_data()?;
            }
            LogLevel::EncodedNoop => {
                self.log_noop()?;
            }
        }

        Ok(())
    }

    pub fn log_data(&self) -> Result<()> {
        let data = self.try_to_vec().map_err(LighthouseError::serialize_err)?;

        sol_log_data(&[&data]);

        Ok(())
    }

    pub fn log_noop(&self) -> Result<()> {
        let data = self.try_to_vec().map_err(LighthouseError::serialize_err)?;

        invoke(
            &Instruction {
                program_id: SPL_NOOP_ID,
                accounts: vec![],
                data,
            },
            &[],
        )
    }

    pub fn log_msg(&self) {
        match self {
            AssertionResult::U8(actual, assertion, operator, passed) => {
                log_case!(U8, actual, assertion, operator, passed);
            }
            AssertionResult::U16(actual, assertion, operator, passed) => {
                log_case!(U16, actual, assertion, operator, passed);
            }
            AssertionResult::U32(actual, assertion, operator, passed) => {
                log_case!(U32, actual, assertion, operator, passed);
            }
            AssertionResult::U64(actual, assertion, operator, passed) => {
                log_case!(U64, actual, assertion, operator, passed);
            }
            AssertionResult::U128(actual, assertion, operator, passed) => {
                log_case!(U128, actual, assertion, operator, passed);
            }
            AssertionResult::I8(actual, assertion, operator, passed) => {
                log_case!(I8, actual, assertion, operator, passed);
            }
            AssertionResult::I16(actual, assertion, operator, passed) => {
                log_case!(I16, actual, assertion, operator, passed);
            }
            AssertionResult::I32(actual, assertion, operator, passed) => {
                log_case!(I32, actual, assertion, operator, passed);
            }
            AssertionResult::I64(actual, assertion, operator, passed) => {
                log_case!(I64, actual, assertion, operator, passed);
            }
            AssertionResult::I128(actual, assertion, operator, passed) => {
                log_case!(I128, actual, assertion, operator, passed);
            }
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
            }
            AssertionResult::Bytes(actual, assertion, operator, passed) => {
                log_case!(Bytes, actual, assertion, operator, passed);
            }
            AssertionResult::Bool(actual, assertion, operator, passed) => {
                log_case!(Bool, actual, assertion, operator, passed);
            }
        }
    }
}
