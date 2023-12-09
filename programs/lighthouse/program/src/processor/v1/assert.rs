use std::collections::BTreeSet;

use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::error::ProgramError;
use crate::structs::{Assertion, Expression};

#[derive(Accounts)]
pub struct AssertV1<'info> {
    // TODO:
    pub cache: Option<UncheckedAccount<'info>>,
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

    for (i, assertion) in assertions.into_iter().enumerate() {
        if (i + 1) > remaining_accounts.len() {
            msg!("The next assertion requires more accounts than were provided");
            return Err(ProgramError::NotEnoughAccounts.into());
        }

        let mut assertion_result = false;

        match assertion {
            Assertion::AccountOwnedBy(pubkey) => {
                let account = &remaining_accounts[i];
                assertion_result = account.owner.key().eq(&pubkey);
            }
            Assertion::Memory(cache_offset, operator, memory_value) => {
                let cache = ctx.accounts.cache.as_ref().unwrap(); // TODO: Graceful error handling
                let cache_data = cache.try_borrow_data()?; // TODO: Graceful error handling

                let (value_str, expected_value_str, assertion_result) = memory_value
                    .deserialize_and_compare(*cache_data, (cache_offset + 8) as usize, &operator)?;

                msg!(
                    "{} {} AssertionParameter::Memory ({}) -> {} {} {}",
                    format!("[{:?}]", i),
                    if assertion_result {
                        "[✅] SUCCESS"
                    } else {
                        "[❌] FAIL   "
                    },
                    cache.key().to_string(),
                    value_str,
                    operator.format(),
                    expected_value_str,
                );
            }
            Assertion::AccountData(account_offset, operator, memory_value) => {
                let account = &remaining_accounts[i];
                let account_data = account.try_borrow_data()?;

                let (value_str, expected_value_str, result) = memory_value
                    .deserialize_and_compare(*account_data, account_offset as usize, &operator)?;

                assertion_result = result;

                msg!(
                    "{} {} Assertion::AccountData ({}) -> {} {} {}",
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
            Assertion::AccountInfo(optional_account_info_data) => {
                let account = &remaining_accounts[i];

                let account_info_data = optional_account_info_data;
                // {
                //     OptionalAccountInfoData::None => return Err(ProgramError::Unimplemented.into()),
                //     OptionalAccountInfoData::Some(account_info_data) => account_info_data,
                // };

                let mut assertion_result = true;

                if let Some(owner) = &account_info_data.owner {
                    if !account.owner.key().eq(owner) {
                        assertion_result = false;
                    }
                }

                if let Some(lamports) = &account_info_data.lamports {
                    if !account.get_lamports().eq(lamports) {
                        assertion_result = false;
                    }
                }

                // if let Some(data_length) = &account_info_data.data_length {
                //     if !account.data_len().eq(&(data_length as usize)) {
                //         assertion_result = false;
                //     }
                // }

                // if let Some(data) = &account_info_data.data {
                //     let account_data = account.try_borrow_data()?;

                //     if !account_data.eq(data) {
                //         assertion_result = false;
                //     }
                // }

                if let Some(rent_epoch) = &account_info_data.rent_epoch {
                    if !account.rent_epoch.eq(rent_epoch) {
                        assertion_result = false;
                    }
                }

                if verbose {
                    msg!(
                        "{} Assertion::AccountInfo ({}) -> {:?}",
                        if assertion_result {
                            "[✅] SUCCESS"
                        } else {
                            "[❌] FAIL   "
                        },
                        account.key().to_string(),
                        account_info_data,
                    );
                }
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
