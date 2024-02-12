use anchor_lang::{
    error,
    prelude::borsh::{self, BorshDeserialize, BorshSerialize},
    AccountDeserialize, Owners,
};
use anchor_spl::token_interface::{self, spl_token_2022::state::AccountState};
use solana_program::{account_info::AccountInfo, program_option::COption, pubkey::Pubkey, stake};

use crate::{error::LighthouseError, Assert, EvaluationResult, Operator};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum StakeAccountDataField {}

// stake

impl Assert for StakeAccountDataField {
    fn evaluate(
        &self,
        account: &AccountInfo,
        operator: &Operator,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>, error::Error> {
        if account.data_is_empty() {
            return Err(LighthouseError::AccountNotInitialized.into());
        }

        if !token_interface::TokenAccount::owners().contains(account.owner) {
            return Err(LighthouseError::AccountNotTokenAccount.into());
        }

        let data = account.try_borrow_mut_data()?;
        let token_account = token_interface::TokenAccount::try_deserialize(&mut data.as_ref())?;
        let result = match self {
            TokenAccountDataField::Mint(pubkey) => {
                operator.evaluate(&token_account.mint, pubkey, include_output)
            }
            TokenAccountDataField::Owner(pubkey) => {
                operator.evaluate(&token_account.owner, pubkey, include_output)
            }
            TokenAccountDataField::Amount(amount) => {
                operator.evaluate(&token_account.amount, amount, include_output)
            }
            TokenAccountDataField::Delegate(assertion_pubkey) => {
                match (token_account.delegate, assertion_pubkey) {
                    (COption::None, None) => Box::new(EvaluationResult {
                        passed: true,
                        output: "None == None".to_string(),
                    }),
                    (COption::Some(token_account_delegate), None) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("{:?} != None", token_account_delegate),
                    }),
                    (COption::None, Some(assertion_pubkey)) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("None != {:?}", assertion_pubkey),
                    }),
                    (COption::Some(token_account_delegate), Some(assertion_pubkey)) => {
                        operator.evaluate(&token_account_delegate, assertion_pubkey, include_output)
                    }
                }
            }
            TokenAccountDataField::State(state) => operator.evaluate(
                &u8_from_account_state(token_account.state),
                state,
                include_output,
            ),
            TokenAccountDataField::IsNative(is_native) => {
                match (token_account.is_native, is_native) {
                    (COption::None, None) => Box::new(EvaluationResult {
                        passed: true,
                        output: "None == None".to_string(),
                    }),
                    (COption::Some(token_account_is_native), None) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("{:?} != None", token_account_is_native),
                    }),
                    (COption::None, Some(is_native)) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("None != {:?}", is_native),
                    }),
                    (COption::Some(token_account_is_native), Some(is_native)) => {
                        operator.evaluate(&token_account_is_native, is_native, include_output)
                    }
                }
            }
            TokenAccountDataField::DelegatedAmount(delegated_amount) => operator.evaluate(
                &token_account.delegated_amount,
                delegated_amount,
                include_output,
            ),
            TokenAccountDataField::CloseAuthority(pubkey) => {
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
                        operator.evaluate(&token_account_close_authority, pubkey, include_output)
                    }
                }
            }
        };

        Ok(result)
    }
}
