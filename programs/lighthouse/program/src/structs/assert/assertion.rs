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
    error::ProgramError,
    structs::{
        operator::{EvaluationResult, Operator},
        u8_from_account_state, AccountInfoDataField, DataValue, LegacyTokenAccountDataField,
    },
    utils::print_assertion_result,
};
use anchor_spl::token::{self, spl_token::state::Account};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AssertionConfig {
    pub verbose: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
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

    // balance, operator
    AccountBalance(u64, Operator),

    AccountOwnedBy(Pubkey, Operator),

    // token balance, operator
    LegacyTokenAccountField(LegacyTokenAccountDataField, Operator),
}

impl Assertion {
    pub fn format(&self) -> String {
        match self {
            Assertion::AccountData(offset, operator, value) => {
                format!("AccountData[{}|{:?}|{:?}]", offset, operator, value)
            }
            Assertion::AccountBalance(balance, operator) => {
                format!("AccountBalance[{}|{:?}]", balance, operator)
            }
            Assertion::AccountOwnedBy(pubkey, operator) => {
                format!("AccountOwnedBy[{}|{:?}]", pubkey, operator)
            }
            Assertion::LegacyTokenAccountField(field, operator) => {
                format!("LegacyTokenAccountField[{:?}|{:?}]", field, operator)
            }
            Assertion::AccountInfoField(fields, operator) => {
                format!("AccountInfoField[{:?}|{:?}]", fields, operator)
            }
        }
    }

    ///
    ///  Remaining account modding for assertion association with remaining accounts
    ///  Reasoning: there are a few ways to associate assertions with accounts
    ///  1. You could store the remaining account index in the assertion instruction data (1 byte)
    ///  2. You could group assertions by account through a vector<vector<assertion> (4 * (4 bytes * unique accounts)), struct (1 byte), or enough
    ///  3. Sort assertions such that their remainder is equal to the index of the remaining account associated with the assertion (<1 byte of transaction data for best case)
    ///     - This sorting will be handled by the client and can be as inefficient or as efficient as the client wants
    ///     - IE a client could pass in a remaining account of [A, A, A, A] or [A] and assertions will be properly associated with the remaining accounts
    ///
    ///  Account A, B, C
    ///
    ///  remaining accounts = [A]
    ///  Assertion 1 (A)
    ///  Assertion 2 (A)
    ///  Assertion 3 (A)
    ///  (A [0%1=0], A [1%1=0], A [2%1=0])
    ///  (1, 2, 3)
    ///
    ///  remaining accounts = [A, B]
    ///  Assertion 1 (A)
    ///  Assertion 2 (A) <- Reorder to third position
    ///  Assertion 3 (B)
    ///  (A [0%2=0], B [1%2=1], A [2%2=0])
    ///  (1, 3, 2)
    ///
    ///  remaining accounts = [A, B, C]
    ///  Assertion 1 (A)
    ///  Assertion 2 (B)
    ///  Assertion 3 (C)
    ///  (A [0%3=0], B [1%3=1], C [2%3=2])
    ///  (1, 2, 3)
    ///
    ///  remaining accounts = [A, B, A]
    ///  Assertion 1 (A)
    ///  Assertion 2 (B)
    ///  Assertion 4 (A)
    ///  Assertion 3 (A)
    ///  (A [0%4=0], B [1%4=1], A [2%3=2], A [3%3=0])
    ///  (1, 2, 4, 3)
    pub fn assert_multi(
        remaining_accounts: &[AccountInfo<'_>],
        assertions: &[Assertion],
        config: Option<AssertionConfig>,
    ) -> Result<()> {
        let include_output = match &config {
            Some(config) => config.verbose,
            None => false,
        };

        if remaining_accounts.is_empty() {
            return Err(ProgramError::NotEnoughAccounts.into());
        }

        for (assertion_index, assertion) in assertions.iter().enumerate() {
            let evaluation_result = assertion.evaluate(
                &remaining_accounts[assertion_index % remaining_accounts.len()],
                include_output,
            )?;

            if include_output {
                print_assertion_result(assertion, assertion_index, &evaluation_result);
            }

            if !evaluation_result.passed {
                return Err(ProgramError::AssertionFailed.into());
            }
        }

        Ok(())
    }

    pub fn evaluate(
        &self,
        target_account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        match &self {
            Assertion::AccountOwnedBy(pubkey, operator) => {
                Ok(operator.evaluate(&target_account.owner, &pubkey, include_output))
            }
            Assertion::AccountData(account_offset, operator, memory_value) => {
                let account_data = target_account.try_borrow_data()?;

                Ok(memory_value.evaluate_from_data_slice(
                    account_data,
                    (*account_offset) as usize,
                    operator,
                    include_output,
                )?)
            }
            Assertion::AccountBalance(balance_value, operator) => Ok(operator.evaluate(
                &**target_account.try_borrow_lamports()?,
                balance_value,
                include_output,
            )),
            Assertion::LegacyTokenAccountField(token_account_field, operator) => {
                if target_account.data_is_empty() {
                    return Err(ProgramError::AccountNotInitialized.into());
                }

                if !target_account.owner.eq(&token::ID) {
                    return Err(ProgramError::AccountNotTokenAccount.into());
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
                    LegacyTokenAccountDataField::Delegate(expected_pubkey) => {
                        match (token_account.delegate, expected_pubkey) {
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
                            (COption::None, Some(expected_pubkey)) => Box::new(EvaluationResult {
                                passed: false,
                                output: format!("None != {:?}", expected_pubkey),
                            }),
                            (COption::Some(token_account_delegate), Some(expected_pubkey)) => {
                                operator.evaluate(
                                    &token_account_delegate,
                                    expected_pubkey,
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
