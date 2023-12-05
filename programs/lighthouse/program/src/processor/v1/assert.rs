use std::collections::BTreeSet;

use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::error::ProgramError;
use crate::structs::{Assertion, BorshField, BorshValue, Expression, Operator};
use crate::utils::process_value;

#[derive(Accounts)]
pub struct AssertV1<'info> {
    pub system_program: Program<'info, System>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Config {
    pub verbose: bool,
}

pub fn assert<'info>(
    ctx: Context<'_, '_, '_, 'info, AssertV1<'info>>,
    assertions: Vec<Assertion>,
    logical_expression: Option<Vec<Expression>>,
    options: Option<Config>,
) -> Result<()> {
    let remaining_accounts = &ctx.remaining_accounts.to_vec();

    let verbose = options.map(|options| options.verbose).unwrap_or(false);
    let mut assertion_results: Vec<bool> = vec![];
    let mut logically_dependent_assertions: Option<BTreeSet<u8>> = None;

    if let Some(logical_expression) = &logical_expression {
        if verbose {
            msg!("Logical expression: {:?}", logical_expression);
        }

        logically_dependent_assertions = Some(BTreeSet::new());
        let tree = logically_dependent_assertions.as_mut().unwrap();

        for (_, logical_expression) in logical_expression.iter().enumerate() {
            match logical_expression {
                Expression::And(assertion_indexes) => {
                    for assertion_index in assertion_indexes {
                        tree.insert(*assertion_index);
                    }
                }
                Expression::Or(assertion_indexes) => {
                    for assertion_index in assertion_indexes {
                        tree.insert(*assertion_index);
                    }
                }
            }
        }
    }

    for (i, assertion_type) in assertions.into_iter().enumerate() {
        if (i + 1) > remaining_accounts.len() {
            msg!("The next assertion requires more accounts than were provided");
            return Err(ProgramError::NotEnoughAccounts.into());
        }

        let mut assertion_result = true;
        if verbose {
            msg!("Testing assertion {:?}", assertion_type);
        }

        match assertion_type {
            Assertion::AccountExists => {
                let account = &remaining_accounts[i];

                if account.data_is_empty() && account.lamports() == 0 {
                    assertion_result = false;
                }
            }
            Assertion::AccountOwnedBy(pubkey) => {
                let account = &remaining_accounts[i];

                if !account.owner.key().eq(&pubkey) {
                    assertion_result = false;
                }
            }
            Assertion::RawAccountData(offset, operator, expected_slice) => {
                let account = &remaining_accounts[i];
                let data = account.try_borrow_data()?;

                let slice = &data[offset as usize..(offset + expected_slice.len() as u64) as usize];

                match operator {
                    Operator::Equal => {
                        if !slice.eq(&expected_slice) {
                            assertion_result = false;
                        }
                    }
                    Operator::NotEqual => {
                        if slice.eq(&expected_slice) {
                            assertion_result = false;
                        }
                    }
                    _ => return Err(ProgramError::UnsupportedOperator.into()),
                }

                if verbose {
                    msg!(
                        "{} Assertion::RawAccountData ({}) -> {:?} {} {:?}",
                        if assertion_result {
                            "[✅] SUCCESS"
                        } else {
                            "[❌] FAIL   "
                        },
                        account.key().to_string(),
                        slice,
                        operator.format(),
                        expected_slice,
                    );
                }
            }
            Assertion::BorshAccountData(offset, borsh_field, operator, expected_value) => {
                let account = &remaining_accounts[i];
                let data = account.try_borrow_data()?;

                let value_str: String;
                let expected_value_str: String;

                match borsh_field {
                    BorshField::U8 => {
                        (value_str, expected_value_str, assertion_result) = process_value::<u8>(
                            &data,
                            offset as u32,
                            1,
                            &match expected_value {
                                BorshValue::U8(value) => value,
                                _ => return Err(ProgramError::BorshValueMismatch.into()),
                            },
                            &borsh_field,
                            &operator,
                        )?;
                    }
                    BorshField::I8 => {
                        let slice = &data[offset as usize..(offset + 1) as usize];
                        let value = i8::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::I8(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::U16 => {
                        let expected_value = match expected_value {
                            BorshValue::U16(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        let slice = &data[offset as usize..(offset + 2) as usize];
                        let value = u16::try_from_slice(slice)?;

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::I16 => {
                        let expected_value = match expected_value {
                            BorshValue::I16(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        let slice = &data[offset as usize..(offset + 2) as usize];
                        let value = i16::try_from_slice(slice)?;

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::U32 => {
                        let slice = &data[offset as usize..(offset + 4) as usize];
                        let value = u32::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::U32(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::I32 => {
                        let slice = &data[offset as usize..(offset + 4) as usize];
                        let value = i32::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::I32(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::U64 => {
                        let slice: &[u8] = &data[offset as usize..(offset + 8) as usize];
                        let value = u64::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::U64(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::I64 => {
                        let slice: &[u8] = &data[offset as usize..(offset + 8) as usize];
                        let value = i64::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::I64(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::U128 => {
                        let slice: &[u8] = &data[offset as usize..(offset + 16) as usize];
                        let value = u128::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::U128(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                    BorshField::I128 => {
                        let slice: &[u8] = &data[offset as usize..(offset + 16) as usize];
                        let value = i128::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::I128(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();

                        // let value = i128::from_le_bytes(*array_ref![data, offset as usize, 16]);
                    }
                    BorshField::Bytes(bytes) => {
                        let slice: &[u8] =
                            &data[offset as usize..(offset + bytes.len() as u64) as usize];
                        let value = u128::try_from_slice(slice)?;

                        let expected_value = match expected_value {
                            BorshValue::U128(value) => value,
                            _ => return Err(ProgramError::BorshValueMismatch.into()),
                        };

                        match operator {
                            Operator::Equal => {}
                            Operator::NotEqual => {}
                            _ => return Err(ProgramError::UnsupportedOperator.into()),
                        }

                        assertion_result = operator.is_true(&value, &expected_value);

                        value_str = value.to_string();
                        expected_value_str = expected_value.to_string();
                    }
                }

                msg!(
                    "{} {} Assertion::BorshAccountData ({}) -> {} {} {}",
                    format!("[{:?}]", i),
                    if assertion_result {
                        "[✅] SUCCESS"
                    } else {
                        "[❌] FAIL   "
                    },
                    account.key().to_string(),
                    value_str,
                    operator.format(),
                    expected_value_str,
                );
            }
            Assertion::AccountBalance(expected_balance, operator) => {
                let account = &remaining_accounts[i];

                assertion_result =
                    operator.is_true(&**account.try_borrow_lamports()?, &expected_balance);

                if verbose {
                    msg!(
                        "{} Assertion::AccountBalance ({}) -> {} {} {}",
                        if assertion_result {
                            "[✅] SUCCESS"
                        } else {
                            "[❌] FAIL   "
                        },
                        account.key().to_string(),
                        account.get_lamports(),
                        operator.format(),
                        expected_balance,
                    );
                }
            }
            Assertion::TokenAccountBalance(expected_balance, operator) => {
                return Err(ProgramError::Unimplemented.into());
            }
            (_) => {} // REMOVE
        }

        assertion_results.push(assertion_result);

        if (logical_expression.is_none()
            || !logically_dependent_assertions
                .as_ref()
                .unwrap()
                .contains(&(i as u8)))
            && !assertion_result
        {
            return Err(ProgramError::AssertionFailed.into());
        }
    }

    if let Some(logical_expressions) = &logical_expression {
        for logical_expression in logical_expressions {
            match logical_expression {
                Expression::And(assertion_indexes) => {
                    let mut result = true;

                    for assertion_index in assertion_indexes {
                        result = result && assertion_results[*assertion_index as usize];
                    }

                    if verbose {
                        msg!(
                            "{} Expression::And -> {:?} {}",
                            if result {
                                "[✅] SUCCESS"
                            } else {
                                "[❌] FAIL   "
                            },
                            result,
                            assertion_indexes
                                .iter()
                                .map(|i| format!("[{}]", i))
                                .collect::<Vec<String>>()
                                .join(" AND ")
                        );
                    }

                    if !result {
                        return Err(ProgramError::AssertionFailed.into());
                    }
                }
                Expression::Or(assertion_indexes) => {
                    let mut result = false;

                    for assertion_index in assertion_indexes {
                        result = result || assertion_results[*assertion_index as usize];
                    }

                    if verbose {
                        msg!(
                            "{} Expression::Or -> {:?} {}",
                            if result {
                                "[✅] SUCCESS"
                            } else {
                                "[❌] FAIL   "
                            },
                            result,
                            assertion_indexes
                                .iter()
                                .map(|i| format!("[{}]", i))
                                .collect::<Vec<String>>()
                                .join(" OR ")
                        );
                    }

                    if !result {
                        return Err(ProgramError::AssertionFailed.into());
                    }
                }
            }
        }
    }

    Ok(())
}
