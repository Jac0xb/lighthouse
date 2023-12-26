use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_option::COption;
use solana_program::program_pack::Pack;

use crate::error::ProgramError;
use crate::structs::{
    u8_from_account_state, AccountInfoDataField, Assertion, AssertionState, Expression,
    LegacyTokenAccountDataField,
};
use crate::utils::print_assertion_result;
use anchor_spl::token::spl_token::state::{Account, AccountState};

#[derive(Accounts)]
pub struct AssertV1<'info> {
    system_program: Program<'info, System>,
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
    let remaining_accounts = ctx.remaining_accounts;
    let mut assertion_state = AssertionState::new(assertions.clone(), logical_expression)?;

    for (i, assertion) in assertions.into_iter().enumerate() {
        let assertion_result: Result<bool> = match &assertion {
            Assertion::AccountOwnedBy(pubkey, operator) => {
                let account = &remaining_accounts[i % remaining_accounts.len()];
                let result = account.owner.key().eq(pubkey);

                let value_str = account.owner.key().to_string();
                let expected_value_str = pubkey.to_string();

                print_assertion_result(
                    &config,
                    &assertion,
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::AccountData(account_offset, operator, memory_value) => {
                let account = &remaining_accounts[i % remaining_accounts.len()];
                let account_data = account.try_borrow_data()?;

                let (value_str, expected_value_str, result) = memory_value
                    .deserialize_and_compare(account_data, (*account_offset) as usize, operator)?;

                print_assertion_result(
                    &config,
                    &assertion,
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::AccountBalance(balance_value, operator) => {
                let account = &remaining_accounts[i % remaining_accounts.len()];
                let result = operator.evaluate(&**account.try_borrow_lamports()?, balance_value);

                let value_str = account.get_lamports().to_string();
                let expected_value_str = balance_value.to_string();

                print_assertion_result(
                    &config,
                    &assertion,
                    result,
                    i,
                    operator,
                    value_str,
                    expected_value_str,
                );

                Ok(result)
            }
            Assertion::LegacyTokenAccountField(token_account_field, operator) => {
                let account = &remaining_accounts[i % remaining_accounts.len()];

                if account.owner.eq(&spl_associated_token_account::id()) {
                    return Err(ProgramError::InvalidAccount.into());
                }

                let token_account = Account::unpack_from_slice(&account.try_borrow_data()?)?;

                let result = match token_account_field {
                    LegacyTokenAccountDataField::Mint(pubkey) => {
                        operator.evaluate(&token_account.mint, pubkey)
                    }
                    LegacyTokenAccountDataField::Owner(pubkey) => {
                        operator.evaluate(&token_account.owner, pubkey)
                    }
                    LegacyTokenAccountDataField::Amount(amount) => {
                        operator.evaluate(&token_account.amount, amount)
                    }
                    LegacyTokenAccountDataField::Delegate(pubkey) => {
                        match (token_account.delegate, pubkey) {
                            (COption::None, None) => true,
                            (COption::None, Some(_)) => false,
                            (COption::Some(_), None) => false,
                            (COption::Some(token_account_delegate), Some(pubkey)) => {
                                operator.evaluate(&token_account_delegate, &pubkey)
                            }
                        }
                    }
                    LegacyTokenAccountDataField::State(state) => {
                        operator.evaluate(&u8_from_account_state(token_account.state), state)
                    }
                    LegacyTokenAccountDataField::IsNative(is_native) => {
                        match (token_account.is_native, is_native) {
                            (COption::None, None) => true,
                            (COption::None, Some(_)) => false,
                            (COption::Some(_), None) => false,
                            (COption::Some(token_account_is_native), Some(is_native)) => {
                                operator.evaluate(&token_account_is_native, is_native)
                            }
                        }
                    }
                    LegacyTokenAccountDataField::DelegatedAmount(delegated_amount) => {
                        operator.evaluate(&token_account.delegated_amount, delegated_amount)
                    }
                    LegacyTokenAccountDataField::CloseAuthority(pubkey) => {
                        match (token_account.close_authority, pubkey) {
                            (COption::None, None) => true,
                            (COption::None, Some(_)) => false,
                            (COption::Some(_), None) => false,
                            (COption::Some(token_account_close_authority), Some(pubkey)) => {
                                operator.evaluate(&token_account_close_authority, pubkey)
                            }
                        }
                    }
                };

                // let result = operator.evaluate(&token_account.amount, balance_value);

                // let value_str = token_account.amount.to_string();
                // let expected_value_str = balance_value.to_string();

                // print_assertion_result(
                //     &config,
                //     &assertion,
                //     result,
                //     i,
                //     operator,
                //     value_str,
                //     expected_value_str,
                // );

                Ok(result)
            }
            Assertion::AccountInfoField(account_info_field, operator) => {
                let account = &remaining_accounts[i % remaining_accounts.len()];
                let operator_result = match account_info_field {
                    AccountInfoDataField::Key(pubkey) => operator.evaluate(&account.key(), pubkey),
                    AccountInfoDataField::Owner(pubkey) => operator.evaluate(account.owner, pubkey),
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
