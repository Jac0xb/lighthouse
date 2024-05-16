use super::{AssertionResult, EquatableOperator, IntegerOperator, LogLevel, Operator};
use crate::{error::LighthouseError, Result};
use solana_program::{program_memory::sol_memcmp, pubkey::Pubkey};

/*
*    Implement the `Evaluate` trait for any `Operator`, used in the evaluation of two values + operator.
*/
pub trait Evaluate<T: Operator> {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        operator: &T,
        log_level: LogLevel,
    ) -> Result<()>;
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

                    if log_level != LogLevel::Silent {
                        AssertionResult::$payload_variant(
                            Some(*actual_value),
                            Some(*assertion_value),
                            *operator as u8,
                            passed,
                        )
                        .log(log_level)?;
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

                    if log_level != LogLevel::Silent {
                        AssertionResult::$payload_variant(
                            Some(*actual_value),
                            Some(*assertion_value),
                            *operator as u8,
                            passed,
                        )
                        .log(log_level)?;
                    }

                    passed
                    .then_some(Ok(()))
                    .unwrap_or_else(|| Err(LighthouseError::AssertionFailed.into()))
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
    (bool, Bool),
    (Pubkey, Pubkey)
);

#[macro_export]
macro_rules! impl_evaluate_option {
    ($(($type:ty, $payload_variant:ident)),*) => {
        $(
            impl Evaluate<EquatableOperator> for Option<&$type> {
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

                    if log_level != LogLevel::Silent {
                        AssertionResult::$payload_variant(
                            actual_value.map(|v| *v),
                            assertion_value.map(|v| *v),
                            *operator as u8,
                            passed,
                        )
                        .log(log_level)?;
                    }

                    passed
                    .then_some(Ok(()))
                    .unwrap_or_else(|| Err(LighthouseError::AssertionFailed.into()))
                }
            }
        )*
    };
}

impl_evaluate_option!((u64, U64), (Pubkey, Pubkey));

impl Evaluate<EquatableOperator> for [u8] {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        operator: &EquatableOperator,
        log_level: LogLevel,
    ) -> Result<()> {
        let passed = match operator {
            EquatableOperator::Equal => {
                if actual_value.len() == assertion_value.len() {
                    sol_memcmp(actual_value, assertion_value, assertion_value.len()) == 0
                } else {
                    false
                }
            }
            EquatableOperator::NotEqual => {
                if actual_value.len() == assertion_value.len() {
                    sol_memcmp(actual_value, assertion_value, assertion_value.len()) != 0
                } else {
                    true
                }
            }
        };

        if log_level != LogLevel::Silent {
            AssertionResult::Bytes(
                actual_value.to_vec(),
                assertion_value.to_vec(),
                *operator as u8,
                passed,
            )
            .log(log_level)?;
        }

        passed
            .then_some(Ok(()))
            .unwrap_or_else(|| Err(LighthouseError::AssertionFailed.into()))
    }
}
