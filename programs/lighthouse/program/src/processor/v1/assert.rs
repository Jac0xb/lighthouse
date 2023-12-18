use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::Pack;

use crate::error::ProgramError;
use crate::structs::{AccountInfoDataField, Assertion, AssertionState, Expression};
use crate::utils::print_assertion_result;

#[derive(Accounts)]
pub struct AssertV1<'info> {
    // TODO:
    pub cache: Option<UncheckedAccount<'info>>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AssertionConfig {
    pub verbose: bool,
}

pub fn assert<'info>(
    ctx: Context<'_, '_, '_, 'info, AssertV1<'info>>,
    assertions: Vec<Assertion>,
    logical_expression: Option<Vec<Expression>>,
    config: Option<AssertionConfig>,
) -> Result<()> {
    let remaining_accounts = &mut ctx.remaining_accounts.iter();
    let mut assertion_state = AssertionState::new(assertions.clone(), logical_expression)?;

    for (i, assertion) in assertions.into_iter().enumerate() {
        let assertion_result: Result<bool> = match &assertion {
            Assertion::AccountOwnedBy(pubkey, operator) => {
                let account = remaining_accounts.next().unwrap();
                let result = account.owner.key().eq(pubkey);

                let value_str = account.owner.key().to_string();
                let expected_value_str = pubkey.to_string();

                print_assertion_result(
                    &config,
                    assertion.format(),
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::Memory(cache_offset, operator, memory_value) => {
                let cache = ctx.accounts.cache.as_ref().unwrap(); // TODO: Graceful error handling
                let cache_data = cache.try_borrow_data()?; // TODO: Graceful error handling

                let (value_str, expected_value_str, result) = memory_value
                    .deserialize_and_compare(cache_data, (cache_offset + 8) as usize, operator)?;

                print_assertion_result(
                    &config,
                    assertion.format(),
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::AccountData(account_offset, operator, memory_value) => {
                let account = remaining_accounts.next().unwrap();
                let account_data = account.try_borrow_data()?;

                let (value_str, expected_value_str, result) = memory_value
                    .deserialize_and_compare(account_data, (*account_offset) as usize, operator)?;

                print_assertion_result(
                    &config,
                    assertion.format(),
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::AccountBalance(balance_value, operator) => {
                let account = remaining_accounts.next().unwrap();
                let result = operator.evaluate(&**account.try_borrow_lamports()?, balance_value);

                let value_str = account.get_lamports().to_string();
                let expected_value_str = balance_value.to_string();

                print_assertion_result(
                    &config,
                    assertion.format(),
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::TokenAccountBalance(balance_value, operator) => {
                let account = remaining_accounts.next().unwrap();

                if account.owner.eq(&spl_associated_token_account::id()) {
                    return Err(ProgramError::InvalidAccount.into());
                }

                let token_account =
                    spl_token::state::Account::unpack_from_slice(&account.try_borrow_data()?)?;

                let result = operator.evaluate(&token_account.amount, balance_value);

                let value_str = token_account.amount.to_string();
                let expected_value_str = balance_value.to_string();

                print_assertion_result(
                    &config,
                    assertion.format(),
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::AccountInfo(account_info_fields, operator) => {
                let account = remaining_accounts.next().unwrap();
                let operator_result = true;

                for account_info_field in account_info_fields {
                    let operator_result = match account_info_field {
                        AccountInfoDataField::Key(pubkey) => {
                            operator.evaluate(&account.key(), pubkey)
                        }
                        AccountInfoDataField::Owner(pubkey) => {
                            operator.evaluate(account.owner, pubkey)
                        }
                        AccountInfoDataField::Lamports(lamports) => {
                            operator.evaluate(&account.get_lamports(), lamports)
                        }
                        AccountInfoDataField::DataLength(data_length) => {
                            operator.evaluate(&(account.data_len() as u64), data_length)
                        }
                        AccountInfoDataField::Executable(executable) => {
                            operator.evaluate(&account.executable, executable)
                        }
                        AccountInfoDataField::IsSigner(is_signer) => {
                            operator.evaluate(&account.is_signer, is_signer)
                        }
                        AccountInfoDataField::IsWritable(is_writable) => {
                            operator.evaluate(&account.is_writable, is_writable)
                        }
                        AccountInfoDataField::RentEpoch(rent_epoch) => {
                            operator.evaluate(&account.rent_epoch as &u64, rent_epoch)
                        }
                    };

                    if !operator_result {
                        break;
                    }
                }

                Ok(operator_result)
            }
        };

        assertion_state.record_result(i, assertion_result?)?;
    }

    msg!("assertion_state: {:?}", assertion_state);
    assertion_state.evaluate()?;

    Ok(())
}

pub fn truncate_pubkey(pubkey: &Pubkey) -> String {
    let mut pubkey_str = pubkey.to_string();
    pubkey_str.truncate(5);
    pubkey_str.push_str("...");

    pubkey_str
}
