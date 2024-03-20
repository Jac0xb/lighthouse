use super::LogLevel;
use crate::{error::LighthouseError, validation::SPL_NOOP_ID, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::Instruction, log::sol_log_data, msg, program::invoke, program_memory::sol_memcmp,
    pubkey::Pubkey,
};
use std::fmt::Debug;

const EQUAL_SYMBOL: &str = "==";
const NOT_EQUAL_SYMBOL: &str = "!=";
const GREATER_THAN_SYMBOL: &str = ">";
const LESS_THAN_SYMBOL: &str = "<";
const GREATER_THAN_OR_EQUAL_SYMBOL: &str = ">=";
const LESS_THAN_OR_EQUAL_SYMBOL: &str = "<=";
const CONTAINS_SYMBOL: &str = "&";
const DOES_NOT_CONTAIN_SYMBOL: &str = "!&";

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
}

pub trait Operator {
    fn format(&self) -> &str;
}

pub trait Evaluate<T: Operator> {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        operator: &T,
        log_level: LogLevel,
    ) -> Result<()>;
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Copy, Clone)]
#[repr(u8)]
pub enum IntegerOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    DoesNotContain,
}

impl Operator for IntegerOperator {
    fn format(&self) -> &str {
        match self {
            IntegerOperator::Equal => EQUAL_SYMBOL,
            IntegerOperator::NotEqual => NOT_EQUAL_SYMBOL,
            IntegerOperator::GreaterThan => GREATER_THAN_SYMBOL,
            IntegerOperator::LessThan => LESS_THAN_SYMBOL,
            IntegerOperator::GreaterThanOrEqual => GREATER_THAN_OR_EQUAL_SYMBOL,
            IntegerOperator::LessThanOrEqual => LESS_THAN_OR_EQUAL_SYMBOL,
            IntegerOperator::Contains => CONTAINS_SYMBOL,
            IntegerOperator::DoesNotContain => DOES_NOT_CONTAIN_SYMBOL,
        }
    }
}

#[macro_export]
macro_rules! impl_integer_evaluate {
    ($(($type:ty, $payload_variant:ident)),*) => {
        $(
            impl Evaluate<IntegerOperator> for $type {
                fn evaluate(
                    actual_value: &Self,
                    assertion_value: &Self,
                    operator: &IntegerOperator,
                    log_level: LogLevel,
                ) -> Result<()> {
                    let passed = match operator {
                        IntegerOperator::Equal => actual_value == assertion_value,
                        IntegerOperator::NotEqual => actual_value != assertion_value,
                        IntegerOperator::GreaterThan => actual_value > assertion_value,
                        IntegerOperator::LessThan => actual_value < assertion_value,
                        IntegerOperator::GreaterThanOrEqual => actual_value >= assertion_value,
                        IntegerOperator::LessThanOrEqual => actual_value <= assertion_value,
                        IntegerOperator::Contains => actual_value & assertion_value == *assertion_value,
                        IntegerOperator::DoesNotContain => actual_value & assertion_value == 0,
                    };

                    match log_level {
                        LogLevel::PlaintextMessage => {
                            msg!(
                                "Result: {} {} {}",
                                actual_value,
                                operator.format(),
                                assertion_value
                            );
                        }
                        LogLevel::Silent => {}
                        LogLevel::EncodedMessage => {
                            AssertionResult::$payload_variant(
                                Some(*actual_value),
                                Some(*assertion_value),
                                *operator as u8,
                                passed,
                            ).log_data()?;
                        }
                        LogLevel::EncodedNoop => {
                            AssertionResult::$payload_variant(
                                Some(*actual_value),
                                Some(*assertion_value),
                                *operator as u8,
                                passed,
                            ).log_noop()?;
                        }
                    }

                    if passed {
                        Ok(())
                    } else {
                        Err(LighthouseError::AssertionFailed.into())
                    }
                }
            }
        )*
    };
}

impl_integer_evaluate!(
    (u8, U8),
    (u16, U16),
    (u32, U32),
    (u64, U64),
    (u128, U128),
    (i8, I8),
    (i16, I16),
    (i32, I32),
    (i64, I64),
    (i128, I128)
);

#[derive(BorshDeserialize, BorshSerialize, Debug, Copy, Clone)]
#[repr(u8)]
pub enum EquatableOperator {
    Equal,
    NotEqual,
}

impl Operator for EquatableOperator {
    fn format(&self) -> &str {
        match self {
            EquatableOperator::Equal => EQUAL_SYMBOL,
            EquatableOperator::NotEqual => NOT_EQUAL_SYMBOL,
        }
    }
}

#[macro_export]
macro_rules! impl_evaluate {
    ($(($type:ty, $payload_variant:ident)),*) => {
        $(
            impl Evaluate<EquatableOperator> for $type {
                fn evaluate(
                    actual_value: &Self,
                    assertion_value: &Self,
                    operator: &EquatableOperator,
                    log_level: LogLevel,
                ) -> Result<()> {
                    let passed = match operator {
                        EquatableOperator::Equal => actual_value == assertion_value,
                        EquatableOperator::NotEqual => actual_value != assertion_value,
                    };

                    match log_level {
                        LogLevel::PlaintextMessage => {
                            msg!(
                                "Result: {} {} {}",
                                actual_value,
                                operator.format(),
                                assertion_value
                            );
                        }
                        LogLevel::EncodedMessage => {
                            AssertionResult::$payload_variant(
                                Some(*actual_value),
                                Some(*assertion_value),
                                *operator as u8,
                                passed,
                            ).log_data()?;
                        }
                        LogLevel::EncodedNoop => {
                            AssertionResult::$payload_variant(
                                Some(*actual_value),
                                Some(*assertion_value),
                                *operator as u8,
                                passed,
                            ).log_noop()?;
                        }
                        LogLevel::Silent => {}
                    }

                    if passed {
                        Ok(())
                    } else {
                        Err(LighthouseError::AssertionFailed.into())
                    }
                }
            }
        )*
    };
}
impl_evaluate!(
    (u8, U8),
    (u16, U16),
    (u32, U32),
    (u64, U64),
    (u128, U128),
    (i8, I8),
    (i16, I16),
    (i32, I32),
    (i64, I64),
    (i128, I128),
    (bool, Bool)
);

#[macro_export]
macro_rules! impl_evaluate_for_option_type {
    ($(($type:ty, $payload_variant:ident)),*) => {
        $(
            impl Evaluate<EquatableOperator> for Option<$type> {
                fn evaluate(
                    actual_value: &Self,
                    assertion_value: &Self,
                    operator: &EquatableOperator,
                    log_level: LogLevel,
                ) -> Result<()> {
                    let passed = match operator {
                        EquatableOperator::Equal => actual_value == assertion_value,
                        EquatableOperator::NotEqual => actual_value != assertion_value,
                    };

                    match log_level {
                        LogLevel::PlaintextMessage => {
                            msg!(
                                "Result: {:?} {} {:?}",
                                actual_value,
                                operator.format(),
                                assertion_value
                            );
                        }
                        LogLevel::EncodedMessage => {
                            AssertionResult::$payload_variant(
                                *actual_value,
                                *assertion_value,
                                *operator as u8,
                                passed,
                            ).log_data()?;
                        }
                        LogLevel::EncodedNoop => {
                            AssertionResult::$payload_variant(
                                *actual_value,
                                *assertion_value,
                                *operator as u8,
                                passed,
                            ).log_noop()?;
                        }
                        LogLevel::Silent => {}
                    }

                    if passed {
                        Ok(())
                    } else {
                        Err(LighthouseError::AssertionFailed.into())
                    }
                }
            }
        )*
    };
}

impl_evaluate_for_option_type!(
    (i8, I8),
    (u8, U8),
    (i16, I16),
    (u16, U16),
    (i32, I32),
    (u32, U32),
    (i64, I64),
    (u64, U64),
    (i128, I128),
    (u128, U128),
    (bool, Bool)
);

impl Evaluate<EquatableOperator> for Pubkey {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        operator: &EquatableOperator,
        log_level: LogLevel,
    ) -> Result<()> {
        let passed = match operator {
            EquatableOperator::Equal => actual_value == assertion_value,
            EquatableOperator::NotEqual => actual_value != assertion_value,
        };

        match log_level {
            LogLevel::PlaintextMessage => {
                msg!("Result: ");
                actual_value.log();
                msg!(operator.format());
                assertion_value.log();
            }
            LogLevel::EncodedMessage => {
                let payload = AssertionResult::Pubkey(
                    Some(*actual_value),
                    Some(*assertion_value),
                    *operator as u8,
                    passed,
                )
                .try_to_vec()
                .map_err(LighthouseError::serialize_err)?;

                sol_log_data(&[payload.as_slice()]);
            }
            LogLevel::EncodedNoop => {
                let payload = AssertionResult::Pubkey(
                    Some(*actual_value),
                    Some(*assertion_value),
                    *operator as u8,
                    passed,
                )
                .try_to_vec()
                .map_err(LighthouseError::serialize_err)?;

                sol_log_data(&[payload.as_slice()]);
            }
            LogLevel::Silent => {}
        }

        if passed {
            Ok(())
        } else {
            Err(LighthouseError::AssertionFailed.into())
        }
    }
}

impl Evaluate<EquatableOperator> for Option<Pubkey> {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        operator: &EquatableOperator,
        log_level: LogLevel,
    ) -> Result<()> {
        let passed = match operator {
            EquatableOperator::Equal => actual_value == assertion_value,
            EquatableOperator::NotEqual => actual_value != assertion_value,
        };

        match log_level {
            LogLevel::PlaintextMessage => match (actual_value, assertion_value) {
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
                    msg!("Result: ");
                    actual_value.log();
                    msg!(operator.format());
                    msg!("None");
                }
                (None, None) => {
                    msg!("Result: None {} None", operator.format());
                }
            },
            LogLevel::EncodedMessage => {
                let payload = AssertionResult::Pubkey(
                    *actual_value,
                    *assertion_value,
                    *operator as u8,
                    passed,
                )
                .try_to_vec()
                .map_err(LighthouseError::serialize_err)?;

                sol_log_data(&[payload.as_slice()]);
            }
            LogLevel::EncodedNoop => {
                let payload = AssertionResult::Pubkey(
                    *actual_value,
                    *assertion_value,
                    *operator as u8,
                    passed,
                )
                .try_to_vec()
                .map_err(LighthouseError::serialize_err)?;

                sol_log_data(&[payload.as_slice()]);
            }
            LogLevel::Silent => {}
        }

        if passed {
            Ok(())
        } else {
            Err(LighthouseError::AssertionFailed.into())
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Copy, Clone)]
#[repr(u8)]
pub enum ByteSliceOperator {
    Equal,
    NotEqual,
}

impl Operator for ByteSliceOperator {
    fn format(&self) -> &str {
        match self {
            ByteSliceOperator::Equal => EQUAL_SYMBOL,
            ByteSliceOperator::NotEqual => NOT_EQUAL_SYMBOL,
        }
    }
}

impl<T> Evaluate<ByteSliceOperator> for T
where
    T: AsRef<[u8]> + ?Sized,
{
    fn evaluate(
        actual_value: &T,
        assertion_value: &T,
        operator: &ByteSliceOperator,
        log_level: LogLevel,
    ) -> Result<()> {
        let passed = match operator {
            ByteSliceOperator::Equal => {
                let actual_value = actual_value.as_ref();
                let assertion_value = assertion_value.as_ref();

                if actual_value.len() == assertion_value.len() {
                    sol_memcmp(actual_value, assertion_value, assertion_value.len()) == 0
                } else {
                    false
                }
            }
            ByteSliceOperator::NotEqual => {
                let actual_value = actual_value.as_ref();
                let assertion_value = assertion_value.as_ref();

                if actual_value.len() == assertion_value.len() {
                    sol_memcmp(actual_value, assertion_value, assertion_value.len()) != 0
                } else {
                    true
                }
            }
        };

        match log_level {
            LogLevel::PlaintextMessage => {
                msg!(
                    "Result: {:?} {} {:?}",
                    actual_value.as_ref(),
                    operator.format(),
                    assertion_value.as_ref()
                );
            }
            LogLevel::EncodedMessage => {
                AssertionResult::Bytes(
                    actual_value.as_ref().to_vec(),
                    assertion_value.as_ref().to_vec(),
                    *operator as u8,
                    passed,
                )
                .log_data()?;
            }
            LogLevel::EncodedNoop => {
                AssertionResult::Bytes(
                    actual_value.as_ref().to_vec(),
                    assertion_value.as_ref().to_vec(),
                    *operator as u8,
                    passed,
                )
                .log_noop()?;
            }
            LogLevel::Silent => {}
        }

        if passed {
            Ok(())
        } else {
            Err(LighthouseError::AssertionFailed.into())
        }
    }
}
