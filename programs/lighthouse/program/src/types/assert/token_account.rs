use crate::{
    error::LighthouseError,
    types::{Assert, ComparableOperator, EquatableOperator, EvaluationResult, Operator},
    utils::{unpack_coption_key, unpack_coption_u64, Result}, // Assert, EvaluationResult, Operator,
};
use anchor_spl::token_interface::spl_token_2022::{self, state::AccountState};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, msg, program_option::COption, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum TokenAccountFieldAssertion {
    Mint(Pubkey, EquatableOperator),
    Owner(Pubkey, EquatableOperator),
    Amount(u64, ComparableOperator),
    Delegate(Option<Pubkey>, EquatableOperator),
    State(u8, ComparableOperator),
    IsNative(Option<u64>, ComparableOperator),
    DelegatedAmount(u64, ComparableOperator),
    CloseAuthority(Option<Pubkey>, EquatableOperator),
}

pub fn account_state_from_u8(value: u8) -> AccountState {
    match value {
        0 => AccountState::Uninitialized,
        1 => AccountState::Initialized,
        2 => AccountState::Frozen,
        _ => panic!("Invalid account state"),
    }
}

pub fn u8_from_account_state(state: AccountState) -> u8 {
    match state {
        AccountState::Uninitialized => 0,
        AccountState::Initialized => 1,
        AccountState::Frozen => 2,
    }
}

impl Assert<AccountInfo<'_>> for TokenAccountFieldAssertion {
    fn format(&self) -> String {
        format!("TokenAccountFieldAssertion[{:?}]", self)
    }

    fn evaluate(
        &self,
        account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        if account.data_is_empty() {
            return Err(LighthouseError::AccountNotInitialized.into());
        }

        if ![spl_token::ID, spl_token_2022::ID].contains(account.owner) {
            return Err(LighthouseError::OwnerMismatch.into());
        }

        let data = account.try_borrow_mut_data().unwrap();

        let result = match self {
            TokenAccountFieldAssertion::Mint(pubkey, operator) => {
                let mint_slice = &data[0..32];
                let mint = Pubkey::try_from(mint_slice).unwrap();

                operator.evaluate(&mint, pubkey, include_output)
            }
            TokenAccountFieldAssertion::Owner(pubkey, operator) => {
                let owner_slice = &data[32..64];
                let owner = Pubkey::try_from(owner_slice).unwrap();

                operator.evaluate(&owner, pubkey, include_output)
            }
            TokenAccountFieldAssertion::Amount(amount, operator) => {
                let amount_slice = &data[64..72];
                let actual_amount = u64::from_le_bytes(amount_slice.try_into().unwrap());

                operator.evaluate(&actual_amount, amount, include_output)
            }
            TokenAccountFieldAssertion::Delegate(assertion_pubkey, operator) => {
                let delegate_slice = &data[72..108];
                let delegate = unpack_coption_key(delegate_slice)?;

                match (delegate, assertion_pubkey) {
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
            TokenAccountFieldAssertion::State(state, operator) => {
                let actual_state = data[108];

                msg!("actual_state: {:?}", actual_state);
                msg!("state: {:?}", state);

                operator.evaluate(&actual_state, state, include_output)
            }
            TokenAccountFieldAssertion::IsNative(is_native, operator) => {
                let is_native_slice = &data[109..121];
                let actual_is_native = unpack_coption_u64(is_native_slice)?;

                match (actual_is_native, is_native) {
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
            TokenAccountFieldAssertion::DelegatedAmount(delegated_amount, operator) => {
                let delegated_amount_slice = &data[121..129];
                let actual_delegated_amount =
                    u64::from_le_bytes(delegated_amount_slice.try_into().unwrap());

                operator.evaluate(&actual_delegated_amount, delegated_amount, include_output)
            }
            TokenAccountFieldAssertion::CloseAuthority(pubkey, operator) => {
                let close_authority_slice = &data[129..165];
                let close_authority = unpack_coption_key(close_authority_slice)?;

                match (close_authority, pubkey) {
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

#[cfg(test)]
mod tests {
    mod evaluate {
        use anchor_spl::token_interface::spl_token_2022::{
            self,
            state::{Account, AccountState},
        };
        use solana_program::{
            account_info::AccountInfo, program_option::COption, program_pack::Pack, pubkey::Pubkey,
        };
        use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
        use std::{cell::RefCell, rc::Rc};

        use crate::types::{
            Assert, ComparableOperator, EquatableOperator, TokenAccountFieldAssertion,
        };

        #[test]
        fn evaluate_token_account_no_delegate_no_close_authority() {
            let mint = Keypair::new();
            let owner = Keypair::new();

            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    mint: mint.encodable_pubkey(),
                    owner: owner.encodable_pubkey(),
                    amount: 69,
                    delegate: COption::None,
                    state: AccountState::Initialized,
                    is_native: COption::Some(1),
                    delegated_amount: 42,
                    close_authority: COption::None,
                },
                serialized_token_account,
            )
            .unwrap();

            println!("{:?}", serialized_token_account);

            let lamports_data: &mut u64 = &mut 0;
            let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

            let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(serialized_token_account));

            let account_info = AccountInfo {
                key: &Pubkey::default(),
                is_signer: false,
                is_writable: false,
                owner: &spl_token_2022::ID,
                lamports: Rc::new(lamports),
                rent_epoch: 0,
                data,
                executable: false,
            };

            //
            // Assert on amount
            //
            let result = TokenAccountFieldAssertion::Amount(69, ComparableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountFieldAssertion::Amount(1600, ComparableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on mint
            //
            let result =
                TokenAccountFieldAssertion::Mint(mint.encodable_pubkey(), EquatableOperator::Equal)
                    .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on owner
            //
            let result = TokenAccountFieldAssertion::Owner(
                owner.encodable_pubkey(),
                EquatableOperator::Equal,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on delegate
            //

            let result = TokenAccountFieldAssertion::Delegate(None, EquatableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountFieldAssertion::Delegate(
                Some(owner.encodable_pubkey()),
                EquatableOperator::NotEqual,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on state
            //

            let result = TokenAccountFieldAssertion::State(
                AccountState::Initialized as u8,
                ComparableOperator::Equal,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountFieldAssertion::State(
                AccountState::Frozen as u8,
                ComparableOperator::Equal,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountFieldAssertion::State(
                AccountState::Uninitialized as u8,
                ComparableOperator::Equal,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on is_native
            //

            let result = TokenAccountFieldAssertion::IsNative(Some(1), ComparableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on delegated_amount
            //
            let result = TokenAccountFieldAssertion::DelegatedAmount(42, ComparableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on close_authority
            //

            let result = TokenAccountFieldAssertion::CloseAuthority(None, EquatableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountFieldAssertion::CloseAuthority(
                Some(owner.encodable_pubkey()),
                EquatableOperator::NotEqual,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }

        #[test]
        fn evaluate_token_account_some_delegate_some_close_authority() {
            let mint = Keypair::new();
            let owner = Keypair::new();
            let delegate = Keypair::new();
            let close_authority = Keypair::new();

            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    mint: mint.encodable_pubkey(),
                    owner: owner.encodable_pubkey(),
                    amount: 69,
                    delegate: COption::Some(delegate.encodable_pubkey()),
                    state: AccountState::Initialized,
                    is_native: COption::Some(1),
                    delegated_amount: 42,
                    close_authority: COption::Some(close_authority.encodable_pubkey()),
                },
                serialized_token_account,
            )
            .unwrap();

            println!("{:?}", serialized_token_account);

            let lamports_data: &mut u64 = &mut 0;
            let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

            let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(serialized_token_account));

            let account_info = AccountInfo {
                key: &Pubkey::default(),
                is_signer: false,
                is_writable: false,
                owner: &spl_token_2022::ID,
                lamports: Rc::new(lamports),
                rent_epoch: 0,
                data,
                executable: false,
            };

            //
            // Assert on delegate
            //

            let result = TokenAccountFieldAssertion::Delegate(None, EquatableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountFieldAssertion::Delegate(
                Some(delegate.encodable_pubkey()),
                EquatableOperator::Equal,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on close_authority
            //
            let result = TokenAccountFieldAssertion::CloseAuthority(None, EquatableOperator::Equal)
                .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountFieldAssertion::CloseAuthority(
                Some(close_authority.encodable_pubkey()),
                EquatableOperator::Equal,
            )
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }
    }
}
