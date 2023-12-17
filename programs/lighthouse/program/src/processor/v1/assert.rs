use std::collections::BTreeSet;

use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::error::ProgramError;
use crate::structs::{AccountInfoDataField, Assertion, AssertionState, Expression};
use crate::utils::print_result;

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
    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    let verbose = options.map(|options| options.verbose).unwrap_or(false);
    let mut assertion_state = AssertionState::new(assertions.clone(), logical_expression)?;

    for (i, assertion) in assertions.into_iter().enumerate() {
        let mut assertion_result = false;

        match assertion {
            Assertion::AccountOwnedBy(pubkey, operator) => {
                let account = remaining_accounts.next().unwrap();
                assertion_result = account.owner.key().eq(&pubkey);

                let value_str = account.owner.key().to_string();
                let expected_value_str = pubkey.to_string();

                if verbose {
                    print_result(assertion_result, i, operator, value_str, expected_value_str);
                }
            }
            Assertion::Memory(cache_offset, operator, memory_value) => {
                let cache = ctx.accounts.cache.as_ref().unwrap(); // TODO: Graceful error handling
                let cache_data = cache.try_borrow_data()?; // TODO: Graceful error handling

                let (value_str, expected_value_str, result) = memory_value
                    .deserialize_and_compare(cache_data, (cache_offset + 8) as usize, &operator)?;

                assertion_result = result;

                if verbose {
                    print_result(assertion_result, i, operator, value_str, expected_value_str);
                }
            }
            Assertion::AccountData(account_offset, operator, memory_value) => {
                let account = remaining_accounts.next().unwrap();
                let account_data = account.try_borrow_data()?;

                let (value_str, expected_value_str, result) = memory_value
                    .deserialize_and_compare(account_data, account_offset as usize, &operator)?;

                assertion_result = result;

                if verbose {
                    print_result(assertion_result, i, operator, value_str, expected_value_str);
                }
            }
            Assertion::AccountBalance(expected_balance, operator) => {
                let account = remaining_accounts.next().unwrap();

                assertion_result =
                    operator.evaluate(&**account.try_borrow_lamports()?, &expected_balance);

                if verbose {
                    print_result(
                        assertion_result,
                        i,
                        operator,
                        account.get_lamports().to_string(),
                        expected_balance.to_string(),
                    );
                }
            }
            Assertion::TokenAccountBalance(_, _) => {
                return Err(ProgramError::Unimplemented.into());
            }
            Assertion::AccountInfo(account_info_fields, operator) => {
                let account = remaining_accounts.next().unwrap();

                for account_info_field in account_info_fields {
                    match account_info_field {
                        AccountInfoDataField::Key(pubkey) => {
                            assertion_result = operator.evaluate(&account.key(), &pubkey);
                        }
                        AccountInfoDataField::Owner(pubkey) => {
                            assertion_result = operator.evaluate(account.owner, &pubkey);
                        }
                        AccountInfoDataField::Lamports(lamports) => {
                            assertion_result =
                                operator.evaluate(&account.get_lamports(), &lamports);
                        }
                        AccountInfoDataField::DataLength(data_length) => {
                            assertion_result =
                                operator.evaluate(&(account.data_len() as u64), &data_length);
                        }
                        AccountInfoDataField::Executable(executable) => {
                            assertion_result = operator.evaluate(&account.executable, &executable);
                        }
                        AccountInfoDataField::IsSigner(is_signer) => {
                            assertion_result = operator.evaluate(&account.is_signer, &is_signer);
                        }
                        AccountInfoDataField::IsWritable(is_writable) => {
                            assertion_result =
                                operator.evaluate(&account.is_writable, &is_writable);
                        }
                        AccountInfoDataField::RentEpoch(rent_epoch) => {
                            assertion_result =
                                operator.evaluate(&account.rent_epoch as &u64, &rent_epoch);
                        }
                    }
                }
            }
        }

        assertion_state.record_result(i, assertion_result)?;

        // assertion_results.push(assertion_result);

        // if (logical_expression.is_none()
        //     || !logically_dependent_assertions
        //         .as_ref()
        //         .unwrap()
        //         .contains(&(i as u8)))
        //     && !assertion_result
        // {
        //     return Err(ProgramError::AssertionFailed.into());
        // }
    }

    msg!("assertion_state: {:?}", assertion_state);
    assertion_state.evaluate()?;

    // if let Some(logical_expressions) = &logical_expression {
    //     for logical_expression in logical_expressions {
    //         match logical_expression {
    //             Expression::And(assertion_indexes) => {
    //                 let mut result = true;

    //                 for assertion_index in assertion_indexes {
    //                     result = result && assertion_results[*assertion_index as usize];
    //                 }

    //                 if verbose {
    //                     msg!(
    //                         "{} Expression::And -> {:?} {}",
    //                         if result {
    //                             "[✅] SUCCESS"
    //                         } else {
    //                             "[❌] FAIL   "
    //                         },
    //                         result,
    //                         assertion_indexes
    //                             .iter()
    //                             .map(|i| format!("[{}]", i))
    //                             .collect::<Vec<String>>()
    //                             .join(" AND ")
    //                     );
    //                 }

    //                 if !result {
    //                     return Err(ProgramError::AssertionFailed.into());
    //                 }
    //             }
    //             Expression::Or(assertion_indexes) => {
    //                 let mut result = false;

    //                 for assertion_index in assertion_indexes {
    //                     result = result || assertion_results[*assertion_index as usize];
    //                 }

    //                 if verbose {
    //                     msg!(
    //                         "{} Expression::Or -> {:?} {}",
    //                         if result {
    //                             "[✅] SUCCESS"
    //                         } else {
    //                             "[❌] FAIL   "
    //                         },
    //                         result,
    //                         assertion_indexes
    //                             .iter()
    //                             .map(|i| format!("[{}]", i))
    //                             .collect::<Vec<String>>()
    //                             .join(" OR ")
    //                     );
    //                 }

    //                 if !result {
    //                     return Err(ProgramError::AssertionFailed.into());
    //                 }
    //             }
    //         }
    //     }
    // }

    Ok(())
}

pub fn truncate_pubkey(pubkey: &Pubkey) -> String {
    let mut pubkey_str = pubkey.to_string();
    pubkey_str.truncate(5);
    pubkey_str.push_str("...");

    pubkey_str
}
