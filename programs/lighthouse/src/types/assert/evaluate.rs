use super::LogLevel;
use crate::{error::LighthouseError, validation::SPL_NOOP_ID, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use lighthouse_common::assertion_settings::{AssertionSettings, DataValue};
use lighthouse_common::integer_operator::IntegerOperator;
use lighthouse_common::operator::{Operator, EQUAL_SYMBOL, NOT_EQUAL_SYMBOL};
use num_traits::ToBytes;
use solana_program::{
    instruction::Instruction, log::sol_log_data, msg, program::invoke, program_memory::sol_memcmp,
    pubkey::Pubkey,
};
use std::{cmp::Ordering, fmt::Debug};

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
    Pubkey(Pubkey, Pubkey, u8, bool),
    Bytes(Vec<u8>, Vec<u8>, u8, bool),
    Bool(Option<bool>, Option<bool>, u8, bool),
}

impl AssertionResult {
    pub fn data_value_to_result(
        actual_slice: &[u8],
        assertion_slice: &[u8],
        assertion_settings: &AssertionSettings,
        passed: bool,
    ) -> Result<Self> {
        match assertion_settings.data_value {
            DataValue::Bool => {
                let actual_value = actual_slice[0] != 0;
                let assertion_value = assertion_slice[0] != 0;

                Ok(AssertionResult::Bool(
                    Some(actual_value),
                    Some(assertion_value),
                    assertion_settings.operator as u8,
                    passed,
                ))
            }
            DataValue::Number => match actual_slice.len() {
                1 => {
                    let actual_value = u8::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = u8::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::U8(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                2 => {
                    let actual_value = u16::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = u16::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::U16(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                4 => {
                    let actual_value = u32::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = u32::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::U32(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                8 => {
                    let actual_value = u64::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = u64::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::U64(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                16 => {
                    let actual_value = u128::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = u128::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::U128(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                _ => Ok(AssertionResult::Bytes(
                    actual_slice.to_vec(),
                    assertion_slice.to_vec(),
                    assertion_settings.operator as u8,
                    passed,
                )),
            },
            DataValue::SignedNumber => match actual_slice.len() {
                1 => {
                    let actual_value = i8::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = i8::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::I8(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                2 => {
                    let actual_value = i16::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = i16::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::I16(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                4 => {
                    let actual_value = i32::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = i32::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::I32(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                8 => {
                    let actual_value = i64::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = i64::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::I64(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                16 => {
                    let actual_value = i128::from_le_bytes(actual_slice.try_into().unwrap());
                    let assertion_value = i128::from_le_bytes(assertion_slice.try_into().unwrap());

                    Ok(AssertionResult::I128(
                        Some(actual_value),
                        Some(assertion_value),
                        assertion_settings.operator as u8,
                        passed,
                    ))
                }
                _ => Ok(AssertionResult::Bytes(
                    actual_slice.to_vec(),
                    assertion_slice.to_vec(),
                    assertion_settings.operator as u8,
                    passed,
                )),
            },
            DataValue::Pubkey => {
                let actual_value = Pubkey::try_from(actual_slice).unwrap();
                let assertion_value = Pubkey::try_from(assertion_slice).unwrap();

                Ok(AssertionResult::Pubkey(
                    actual_value,
                    assertion_value,
                    assertion_settings.operator as u8,
                    passed,
                ))
            }
            DataValue::Bytes => Ok(AssertionResult::Bytes(
                actual_slice.to_vec(),
                assertion_slice.to_vec(),
                assertion_settings.operator as u8,
                passed,
            )),
        }
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
        msg!("Result: {:?}", self);
    }
}

pub trait Evaluate<T: Operator> {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        operator: &T,
        log_level: LogLevel,
    ) -> Result<()>;
}

pub trait EvaluateV2<T: Operator> {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        assertion_settings: &AssertionSettings,
        operator: &T,
        log_level: LogLevel,
    ) -> Result<()>;
}

#[macro_export]
macro_rules! impl_uint_evaluate {
    ($(($type:ty, $payload_variant:ident)),*) => {
        $(
            impl Evaluate<IntegerOperator> for $type {
                fn evaluate(
                    actual_value: &Self,
                    assertion_value: &Self,
                    operator: &IntegerOperator,
                    log_level: LogLevel,
                ) -> Result<()> {
                    evaluate_bytes(
                        &actual_value.to_le_bytes(),
                        &assertion_value.to_le_bytes(),
                        &AssertionSettings {
                            is_big_endian: false,
                            operator: *operator,
                            data_value: DataValue::Number,
                        },
                        log_level,
                    )
                }
            }
        )*
    };
}

impl_uint_evaluate!((u8, U8), (u16, U16), (u32, U32), (u64, U64), (u128, U128));

#[macro_export]
macro_rules! impl_int_evaluate {
    ($(($type:ty, $payload_variant:ident)),*) => {
        $(
            impl Evaluate<IntegerOperator> for $type {
                fn evaluate(
                    actual_value: &Self,
                    assertion_value: &Self,
                    operator: &IntegerOperator,
                    log_level: LogLevel,
                ) -> Result<()> {
                    evaluate_bytes(
                        &actual_value.to_le_bytes(),
                        &assertion_value.to_le_bytes(),
                        &AssertionSettings {
                            is_big_endian: false,
                            operator: *operator,
                            data_value: DataValue::SignedNumber,
                        },
                        log_level,
                    )
                }
            }
        )*
    };
}

impl_int_evaluate!((i8, I8), (i16, I16), (i32, I32), (i64, I64), (i128, I128));

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

                    // evaluate_bytes(
                    //     &actual_value.unwrap_or_default().to_le_bytes(),
                    //     &assertion_value.unwrap_or_default().to_le_bytes(),
                    //     &AssertionSettings {
                    //         is_big_endian: false,
                    //         operator: IntegerOperator::try_from(*operator as u8).unwrap(),
                    //         data_value: DataValue::Number,
                    //     },
                    //     log_level,
                    // )

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
        // let passed = match operator {
        //     EquatableOperator::Equal => actual_value == assertion_value,
        //     EquatableOperator::NotEqual => actual_value != assertion_value,
        // };

        evaluate_bytes(
            &actual_value.to_bytes(),
            &assertion_value.to_bytes(),
            &AssertionSettings {
                is_big_endian: false,
                operator: IntegerOperator::try_from(*operator as u8).unwrap(),
                data_value: DataValue::Pubkey,
            },
            log_level,
        )

        // match log_level {
        //     LogLevel::PlaintextMessage => {
        //         msg!("Result: ");
        //         actual_value.log();
        //         msg!(operator.format());
        //         assertion_value.log();
        //     }
        //     LogLevel::EncodedMessage => {
        //         let payload = AssertionResult::Pubkey(
        //             Some(*actual_value),
        //             Some(*assertion_value),
        //             *operator as u8,
        //             passed,
        //         )
        //         .try_to_vec()
        //         .map_err(LighthouseError::serialize_err)?;

        //         sol_log_data(&[payload.as_slice()]);
        //     }
        //     LogLevel::EncodedNoop => {
        //         let payload = AssertionResult::Pubkey(
        //             Some(*actual_value),
        //             Some(*assertion_value),
        //             *operator as u8,
        //             passed,
        //         )
        //         .try_to_vec()
        //         .map_err(LighthouseError::serialize_err)?;

        //         sol_log_data(&[payload.as_slice()]);
        //     }
        //     LogLevel::Silent => {}
        // }

        // if passed {
        //     Ok(())
        // } else {
        //     Err(LighthouseError::AssertionFailed.into())
        // }
    }
}

// impl Evaluate<EquatableOperator> for Option<&Pubkey> {
//     fn evaluate(
//         actual_value: &Self,
//         assertion_value: &Self,
//         operator: &EquatableOperator,
//         log_level: LogLevel,
//     ) -> Result<()> {

//         evaluate_bytes(
//             &actual_value.unwrap_or(&Pubkey::default()).to_bytes(),
//             &assertion_value.unwrap_or(&Pubkey::default()).to_bytes(),
//             &AssertionSettings {
//                 is_big_endian: false,
//                 operator: IntegerOperator::try_from(*operator as u8).unwrap(),
//                 data_value: DataValue::Pubkey,
//             },
//             log_level,
//         )
//     }
// }

impl Evaluate<EquatableOperator> for [u8] {
    fn evaluate(
        actual_value: &Self,
        assertion_value: &Self,
        operator: &EquatableOperator,
        log_level: LogLevel,
    ) -> Result<()> {
        // let passed = match operator {
        //     EquatableOperator::Equal => {
        //         if actual_value.len() == assertion_value.len() {
        //             sol_memcmp(actual_value, assertion_value, assertion_value.len()) == 0
        //         } else {
        //             false
        //         }
        //     }
        //     EquatableOperator::NotEqual => {
        //         if actual_value.len() == assertion_value.len() {
        //             sol_memcmp(actual_value, assertion_value, assertion_value.len()) != 0
        //         } else {
        //             true
        //         }
        //     }
        // };

        evaluate_bytes(
            actual_value,
            assertion_value,
            &AssertionSettings {
                is_big_endian: false,
                operator: IntegerOperator::try_from(*operator as u8).unwrap(),
                data_value: DataValue::Bytes,
            },
            log_level,
        )

        // match log_level {
        //     LogLevel::PlaintextMessage => {
        //         msg!(
        //             "Result: {:?} {} {:?}",
        //             actual_value,
        //             operator.format(),
        //             assertion_value
        //         );
        //     }
        //     LogLevel::EncodedMessage => {
        //         AssertionResult::Bytes(
        //             actual_value.to_vec(),
        //             assertion_value.to_vec(),
        //             *operator as u8,
        //             passed,
        //         )
        //         .log_data()?;
        //     }
        //     LogLevel::EncodedNoop => {
        //         AssertionResult::Bytes(
        //             actual_value.to_vec(),
        //             assertion_value.to_vec(),
        //             *operator as u8,
        //             passed,
        //         )
        //         .log_noop()?;
        //     }
        //     LogLevel::Silent => {}
        // }

        // if passed {
        //     Ok(())
        // } else {
        //     Err(LighthouseError::AssertionFailed.into())
        // }
    }
}

pub fn evaluate_bytes(
    actual_value: &[u8],
    assertion_value: &[u8],
    assertion_settings: &AssertionSettings,
    log_level: LogLevel,
) -> Result<()> {
    // msg!("actual_value: {:?}", actual_value);
    // msg!("assertion_value: {:?}", assertion_value);
    // msg!("assertion settings: {:?}", assertion_settings);

    if actual_value.len() != assertion_value.len() {
        panic!("Evaluation bytes are not equal")
    }

    let passed = match assertion_settings.operator {
        IntegerOperator::Equal => {
            sol_memcmp(actual_value, assertion_value, assertion_value.len()) == 0
        }
        IntegerOperator::NotEqual => {
            sol_memcmp(actual_value, assertion_value, assertion_value.len()) != 0
        }
        IntegerOperator::GreaterThan => {
            if assertion_settings.data_value == DataValue::SignedNumber {
                compare_signed_bytes(actual_value, assertion_value) == Ordering::Greater
            } else {
                compare_unsigned_bytes(actual_value, assertion_value) == Ordering::Greater
            }
        }
        IntegerOperator::LessThan => {
            if assertion_settings.data_value == DataValue::SignedNumber {
                compare_signed_bytes(actual_value, assertion_value) == Ordering::Less
            } else {
                compare_unsigned_bytes(actual_value, assertion_value) == Ordering::Less
            }
        }
        IntegerOperator::GreaterThanOrEqual => {
            if assertion_settings.data_value == DataValue::SignedNumber {
                compare_signed_bytes(actual_value, assertion_value) != Ordering::Less
            } else {
                compare_unsigned_bytes(actual_value, assertion_value) != Ordering::Less
            }
        }
        IntegerOperator::LessThanOrEqual => {
            if assertion_settings.data_value == DataValue::SignedNumber {
                compare_signed_bytes(actual_value, assertion_value) != Ordering::Greater
            } else {
                compare_unsigned_bytes(actual_value, assertion_value) != Ordering::Greater
            }
        }
        IntegerOperator::Contains => {
            let mut contains = true;
            for i in 0..actual_value.len() {
                if actual_value[i] & assertion_value[i] != assertion_value[i] {
                    contains = false;
                    break;
                }
            }

            contains
        }
        IntegerOperator::DoesNotContain => {
            let mut contains = true;
            for i in 0..actual_value.len() {
                if actual_value[i] & assertion_value[i] != 0 {
                    contains = false;
                    break;
                }
            }

            contains
        }
    };

    match log_level {
        LogLevel::EncodedMessage => AssertionResult::data_value_to_result(
            actual_value,
            assertion_value,
            assertion_settings,
            passed,
        )?
        .log_data()?,
        LogLevel::EncodedNoop => AssertionResult::data_value_to_result(
            actual_value,
            assertion_value,
            assertion_settings,
            passed,
        )?
        .log_noop()?,
        LogLevel::PlaintextMessage => AssertionResult::data_value_to_result(
            actual_value,
            assertion_value,
            assertion_settings,
            passed,
        )?
        .log_msg(),
        LogLevel::Silent => {}
    }

    if passed {
        Ok(())
    } else {
        Err(LighthouseError::AssertionFailed.into())
    }
}

fn compare_signed_bytes(v1: &[u8], v2: &[u8]) -> Ordering {
    let negative1 = v1.last().map_or(false, |&b| b & 0x80 != 0);
    let negative2 = v2.last().map_or(false, |&b| b & 0x80 != 0);

    match (negative1, negative2) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => {
            // Both numbers have the same sign
            if v1.len() == v2.len() {
                v1.iter().rev().cmp(v2.iter().rev())
            } else if negative1 {
                // Both are negative, longer means more magnitude (more negative)
                v2.len().cmp(&v1.len())
            } else {
                // Both are positive, longer means more magnitude (more positive)
                v1.len().cmp(&v2.len())
            }
        }
    }
}

fn compare_unsigned_bytes(v1: &[u8], v2: &[u8]) -> Ordering {
    if v1.len() == v2.len() {
        v1.iter().rev().cmp(v2.iter().rev())
    } else {
        v1.len().cmp(&v2.len())
    }
}
