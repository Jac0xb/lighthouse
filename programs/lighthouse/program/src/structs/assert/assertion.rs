use anchor_lang::{
    prelude::{
        borsh,
        borsh::{BorshDeserialize, BorshSerialize},
        Result,
    },
    Key, Lamports,
};
use solana_program::{
    account_info::AccountInfo, program_option::COption, program_pack::Pack, pubkey::Pubkey,
};

use crate::{
    error::LighthouseError,
    structs::{
        operator::{EvaluationResult, Operator},
        u8_from_account_state, AccountInfoDataField, DataValue, LegacyTokenAccountDataField,
    },
};
use anchor_spl::token::{self, spl_token::state::Account};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AssertionConfigV1 {
    pub verbose: bool,
}

///
///     Used to store assertions in a compact form and not require 3 additional vector bytes
///
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AssertionArray {
    Size1([Assertion; 1]),
    Size2([Assertion; 2]),
    Size3([Assertion; 3]),
    Size4([Assertion; 4]),
    Size5([Assertion; 5]),
    Size6([Assertion; 6]),
    Size7([Assertion; 7]),
    Size8([Assertion; 8]),
    Size9([Assertion; 9]),
    Size10([Assertion; 10]),
    Size11([Assertion; 11]),
    Size12([Assertion; 12]),
    Size13([Assertion; 13]),
    Size14([Assertion; 14]),
    Size15([Assertion; 15]),
    Size16([Assertion; 16]),
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum Assertion {
    AccountInfoField(AccountInfoDataField, Operator),

    // account data offset, borsh type, operator
    AccountData(u16, Operator, DataValue),

    // token balance, operator
    LegacyTokenAccountField(LegacyTokenAccountDataField, Operator),
}

impl Assertion {
    pub fn format(&self) -> String {
        match self {
            Assertion::AccountData(offset, operator, value) => {
                format!("AccountData[{}|{:?}|{:?}]", offset, operator, value)
            }
            Assertion::LegacyTokenAccountField(field, operator) => {
                format!("LegacyTokenAccountField[{:?}|{:?}]", field, operator)
            }
            Assertion::AccountInfoField(fields, operator) => {
                format!("AccountInfoField[{:?}|{:?}]", fields, operator)
            }
        }
    }

    pub fn evaluate(
        &self,
        target_account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        match &self {
            Assertion::AccountData(account_offset, operator, memory_value) => {
                let account_data = target_account.try_borrow_data()?;

                Ok(memory_value.evaluate_from_data_slice(
                    account_data,
                    (*account_offset) as usize,
                    operator,
                    include_output,
                )?)
            }
            Assertion::LegacyTokenAccountField(token_account_field, operator) => {
                if target_account.data_is_empty() {
                    return Err(LighthouseError::AccountNotInitialized.into());
                }

                if !target_account.owner.eq(&token::ID) {
                    return Err(LighthouseError::AccountNotTokenAccount.into());
                }

                let token_account = Account::unpack_from_slice(&target_account.try_borrow_data()?)?;

                let result = match token_account_field {
                    LegacyTokenAccountDataField::Mint(pubkey) => {
                        operator.evaluate(&token_account.mint, pubkey, include_output)
                    }
                    LegacyTokenAccountDataField::Owner(pubkey) => {
                        operator.evaluate(&token_account.owner, pubkey, include_output)
                    }
                    LegacyTokenAccountDataField::Amount(amount) => {
                        operator.evaluate(&token_account.amount, amount, include_output)
                    }
                    LegacyTokenAccountDataField::Delegate(assertion_pubkey) => {
                        match (token_account.delegate, assertion_pubkey) {
                            (COption::None, None) => Box::new(EvaluationResult {
                                passed: true,
                                output: "None == None".to_string(),
                            }),
                            (COption::Some(token_account_delegate), None) => {
                                Box::new(EvaluationResult {
                                    passed: false,
                                    output: format!("{:?} != None", token_account_delegate),
                                })
                            }
                            (COption::None, Some(assertion_pubkey)) => Box::new(EvaluationResult {
                                passed: false,
                                output: format!("None != {:?}", assertion_pubkey),
                            }),
                            (COption::Some(token_account_delegate), Some(assertion_pubkey)) => {
                                operator.evaluate(
                                    &token_account_delegate,
                                    assertion_pubkey,
                                    include_output,
                                )
                            }
                        }
                    }
                    LegacyTokenAccountDataField::State(state) => operator.evaluate(
                        &u8_from_account_state(token_account.state),
                        state,
                        include_output,
                    ),
                    LegacyTokenAccountDataField::IsNative(is_native) => {
                        match (token_account.is_native, is_native) {
                            (COption::None, None) => Box::new(EvaluationResult {
                                passed: true,
                                output: "None == None".to_string(),
                            }),
                            (COption::Some(token_account_is_native), None) => {
                                Box::new(EvaluationResult {
                                    passed: false,
                                    output: format!("{:?} != None", token_account_is_native),
                                })
                            }
                            (COption::None, Some(is_native)) => Box::new(EvaluationResult {
                                passed: false,
                                output: format!("None != {:?}", is_native),
                            }),
                            (COption::Some(token_account_is_native), Some(is_native)) => operator
                                .evaluate(&token_account_is_native, is_native, include_output),
                        }
                    }
                    LegacyTokenAccountDataField::DelegatedAmount(delegated_amount) => operator
                        .evaluate(
                            &token_account.delegated_amount,
                            delegated_amount,
                            include_output,
                        ),
                    LegacyTokenAccountDataField::CloseAuthority(pubkey) => {
                        match (token_account.close_authority, pubkey) {
                            (COption::None, None) => Box::new(EvaluationResult {
                                passed: true,
                                output: "None == None".to_string(),
                            }),
                            (COption::Some(token_account_close_authority), None) => {
                                Box::new(EvaluationResult {
                                    passed: false,
                                    output: format!("{:?} != None", token_account_close_authority),
                                })
                            }
                            (COption::None, Some(pubkey)) => Box::new(EvaluationResult {
                                passed: false,
                                output: format!("None != {:?}", pubkey),
                            }),
                            (COption::Some(token_account_close_authority), Some(pubkey)) => {
                                operator.evaluate(
                                    &token_account_close_authority,
                                    pubkey,
                                    include_output,
                                )
                            }
                        }
                    }
                };

                Ok(result)
            }
            Assertion::AccountInfoField(account_info_field, operator) => {
                let operator_result = match account_info_field {
                    AccountInfoDataField::Key(pubkey) => {
                        operator.evaluate(&target_account.key(), pubkey, include_output)
                    }
                    AccountInfoDataField::Owner(pubkey) => {
                        operator.evaluate(target_account.owner, pubkey, include_output)
                    }
                    AccountInfoDataField::Lamports(lamports) => {
                        operator.evaluate(&target_account.get_lamports(), lamports, include_output)
                    }
                    AccountInfoDataField::DataLength(data_length) => operator.evaluate(
                        &(target_account.data_len() as u64),
                        data_length,
                        include_output,
                    ),
                    AccountInfoDataField::Executable(executable) => {
                        operator.evaluate(&target_account.executable, executable, include_output)
                    }
                    AccountInfoDataField::IsSigner(is_signer) => {
                        operator.evaluate(&target_account.is_signer, is_signer, include_output)
                    }
                    AccountInfoDataField::IsWritable(is_writable) => {
                        operator.evaluate(&target_account.is_writable, is_writable, include_output)
                    }
                    AccountInfoDataField::RentEpoch(rent_epoch) => operator.evaluate(
                        &target_account.rent_epoch as &u64,
                        rent_epoch,
                        include_output,
                    ),
                };

                Ok(operator_result)
            }
        }
    }
}
