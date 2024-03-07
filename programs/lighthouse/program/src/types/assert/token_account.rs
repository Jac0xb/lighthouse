use super::{Assert, LogLevel};
use crate::{
    err, err_msg,
    error::LighthouseError,
    types::assert::operator::{ComparableOperator, EquatableOperator, EvaluationResult, Operator},
    utils::{out_of_bounds_err, unpack_coption_key, unpack_coption_u64, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_option::COption, pubkey::Pubkey};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token_2022::state::AccountState;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum TokenAccountAssertion {
    Mint {
        value: Pubkey,
        operator: EquatableOperator,
    },
    Owner {
        value: Pubkey,
        operator: EquatableOperator,
    },
    Amount {
        value: u64,
        operator: ComparableOperator,
    },
    Delegate {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    State {
        value: u8,
        operator: ComparableOperator,
    },
    IsNative {
        value: Option<u64>,
        operator: ComparableOperator,
    },
    DelegatedAmount {
        value: u64,
        operator: ComparableOperator,
    },
    CloseAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    TokenAccountOwnerIsDerived,
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

impl Assert<&AccountInfo<'_>> for TokenAccountAssertion {
    fn evaluate(
        &self,
        account: &AccountInfo<'_>,
        log_level: LogLevel,
    ) -> Result<Box<EvaluationResult>> {
        if account.data_is_empty() {
            return Err(LighthouseError::AccountNotInitialized.into());
        }

        if ![spl_token::ID, spl_token_2022::ID].contains(account.owner) {
            return Err(LighthouseError::AccountOwnerMismatch.into());
        }

        let data = account.try_borrow_data().map_err(|e| {
            err_msg!("Failed to borrow data for target account", e);
            err!(LighthouseError::AccountBorrowFailed)
        })?;

        let result = match self {
            TokenAccountAssertion::Mint {
                value: assertion_value,
                operator,
            } => {
                let data_range = 0..32;
                let data_slice = data
                    .get(data_range.clone())
                    .ok_or_else(|| out_of_bounds_err(data_range))?;
                let mint = Pubkey::try_from(data_slice).map_err(|e| {
                    err_msg!("Failed to deserialize mint from account data", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?;

                operator.evaluate(&mint, assertion_value, log_level)
            }
            TokenAccountAssertion::Owner {
                value: assertion_value,
                operator,
            } => {
                let data_range = 32..64;
                let data_slice = data
                    .get(data_range.clone())
                    .ok_or_else(|| out_of_bounds_err(data_range))?;
                let owner = Pubkey::try_from(data_slice).map_err(|e| {
                    err_msg!("Failed to deserialize owner from account data", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?;

                operator.evaluate(&owner, assertion_value, log_level)
            }
            TokenAccountAssertion::Amount {
                value: assertion_value,
                operator,
            } => {
                let data_range = 64..72;
                let data_slice = data
                    .get(data_range.clone())
                    .ok_or_else(|| out_of_bounds_err(data_range))?;
                let actual_amount = u64::from_le_bytes(data_slice.try_into().map_err(|e| {
                    err_msg!("Failed to deserialize amount from account data", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?);

                operator.evaluate(&actual_amount, assertion_value, log_level)
            }
            TokenAccountAssertion::Delegate {
                value: assertion_value,
                operator,
            } => {
                let data_range = 72..108;
                let data_slice = data
                    .get(data_range.clone())
                    .ok_or_else(|| out_of_bounds_err(data_range))?;
                let delegate = unpack_coption_key(data_slice)?;

                match (delegate, assertion_value) {
                    (COption::None, None) => Box::new(EvaluationResult {
                        passed: true,
                        output: "None == None".to_string(),
                    }),
                    (COption::Some(token_account_delegate), None) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("{} != None", token_account_delegate),
                    }),
                    (COption::None, Some(assertion_pubkey)) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("None != {}", assertion_pubkey),
                    }),
                    (COption::Some(token_account_delegate), Some(assertion_pubkey)) => {
                        operator.evaluate(&token_account_delegate, assertion_pubkey, log_level)
                    }
                }
            }
            TokenAccountAssertion::State {
                value: assertion_value,
                operator,
            } => {
                let actual_state = data.get(108).ok_or_else(|| out_of_bounds_err(108..109))?;

                operator.evaluate(actual_state, assertion_value, log_level)
            }
            TokenAccountAssertion::IsNative {
                value: assertion_value,
                operator,
            } => {
                let data_range = 109..121;
                let data_slice = data
                    .get(data_range.clone())
                    .ok_or_else(|| out_of_bounds_err(data_range))?;

                let actual_is_native = unpack_coption_u64(data_slice)?;

                match (actual_is_native, assertion_value) {
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
                        operator.evaluate(&token_account_is_native, is_native, log_level)
                    }
                }
            }
            TokenAccountAssertion::DelegatedAmount {
                value: assertion_value,
                operator,
            } => {
                let data_range = 121..129;
                let data_slice = data
                    .get(data_range.clone())
                    .ok_or_else(|| out_of_bounds_err(data_range))?;

                let actual_delegated_amount =
                    u64::from_le_bytes(data_slice.try_into().map_err(|e| {
                        err_msg!("Failed to deserialize delegatedamount from account data", e);
                        err!(LighthouseError::FailedToDeserialize)
                    })?);

                operator.evaluate(&actual_delegated_amount, assertion_value, log_level)
            }
            TokenAccountAssertion::CloseAuthority {
                value: assertion_value,
                operator,
            } => {
                let data_range = 129..165;
                let data_slice = data
                    .get(data_range.clone())
                    .ok_or_else(|| out_of_bounds_err(data_range))?;
                let close_authority = unpack_coption_key(data_slice)?;

                match (close_authority, assertion_value) {
                    (COption::None, None) => Box::new(EvaluationResult {
                        passed: true,
                        output: "None == None".to_string(),
                    }),
                    (COption::Some(token_account_close_authority), None) => {
                        Box::new(EvaluationResult {
                            passed: false,
                            output: format!("{} != None", token_account_close_authority),
                        })
                    }
                    (COption::None, Some(pubkey)) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("None != {}", pubkey),
                    }),
                    (COption::Some(token_account_close_authority), Some(pubkey)) => {
                        operator.evaluate(&token_account_close_authority, pubkey, log_level)
                    }
                }
            }
            TokenAccountAssertion::TokenAccountOwnerIsDerived => {
                let mint_range = 0..32;
                let mint_data = data
                    .get(mint_range.clone())
                    .ok_or_else(|| out_of_bounds_err(mint_range))?;
                let mint = Pubkey::try_from(mint_data).map_err(|e| {
                    err_msg!("Failed to deserialize mint from account data", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?;

                let owner_range = 32..64;
                let owner_data = data
                    .get(owner_range.clone())
                    .ok_or_else(|| out_of_bounds_err(owner_range))?;
                let owner = Pubkey::try_from(owner_data).map_err(|e| {
                    err_msg!("Failed to deserialize owner from account data", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?;

                let expected_ata =
                    get_associated_token_address_with_program_id(&owner, &mint, account.owner);

                EquatableOperator::Equal.evaluate(account.key, &expected_ata, log_level)
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    mod evaluate {
        use solana_program::{
            account_info::AccountInfo, program_option::COption, program_pack::Pack, pubkey::Pubkey,
        };
        use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
        use spl_associated_token_account::get_associated_token_address_with_program_id;
        use spl_token_2022::state::{Account, AccountState};
        use std::{cell::RefCell, rc::Rc};

        use crate::types::{
            assert::operator::{ComparableOperator, EquatableOperator},
            assert::{Assert, LogLevel, TokenAccountAssertion},
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
            let result = TokenAccountAssertion::Amount {
                value: 69,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountAssertion::Amount {
                value: 1600,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on mint
            //
            let result = TokenAccountAssertion::Mint {
                value: mint.encodable_pubkey(),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on owner
            //
            let result = TokenAccountAssertion::Owner {
                value: owner.encodable_pubkey(),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on delegate
            //

            let result = TokenAccountAssertion::Delegate {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountAssertion::Delegate {
                value: Some(owner.encodable_pubkey()),
                operator: EquatableOperator::NotEqual,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on state
            //

            let result = TokenAccountAssertion::State {
                value: AccountState::Initialized as u8,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountAssertion::State {
                value: AccountState::Frozen as u8,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountAssertion::State {
                value: AccountState::Uninitialized as u8,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on is_native
            //

            let result = TokenAccountAssertion::IsNative {
                value: Some(1),
                operator: ComparableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on delegated_amount
            //
            let result = TokenAccountAssertion::DelegatedAmount {
                value: 42,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on close_authority
            //

            let result = TokenAccountAssertion::CloseAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountAssertion::CloseAuthority {
                value: Some(owner.encodable_pubkey()),
                operator: EquatableOperator::NotEqual,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

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

            let result = TokenAccountAssertion::Delegate {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountAssertion::Delegate {
                value: Some(delegate.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on close_authority
            //
            let result = TokenAccountAssertion::CloseAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = TokenAccountAssertion::CloseAuthority {
                value: Some(close_authority.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }

        #[test]
        fn evaluate_token_account_address_is_derived() {
            let mint = Keypair::new();
            let owner = Keypair::new();
            let delegate = Keypair::new();
            let close_authority = Keypair::new();

            // Owner is derived
            {
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

                let lamports_data: &mut u64 = &mut 0;
                let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);
                let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(serialized_token_account));
                let account_info = AccountInfo {
                    key: &get_associated_token_address_with_program_id(
                        &owner.encodable_pubkey(),
                        &mint.encodable_pubkey(),
                        &spl_token_2022::ID,
                    ),
                    is_signer: false,
                    is_writable: false,
                    owner: &spl_token_2022::ID,
                    lamports: Rc::new(lamports),
                    rent_epoch: 0,
                    data,
                    executable: false,
                };

                // assert on TokenAccountOwnerIsDerived
                let result = TokenAccountAssertion::TokenAccountOwnerIsDerived
                    .evaluate(&account_info, LogLevel::PlaintextMsgLog);

                if let Ok(result) = result {
                    assert!(result.passed, "{:?}", result.output);
                } else {
                    let error = result.err().unwrap();
                    panic!("{:?}", error);
                }
            }

            // None derived owner
            {
                let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
                Account::pack(
                    Account {
                        mint: mint.encodable_pubkey(),
                        owner: Keypair::new().encodable_pubkey(),
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

                // assert on TokenAccountOwnerIsDerived
                let result = TokenAccountAssertion::TokenAccountOwnerIsDerived
                    .evaluate(&account_info, LogLevel::PlaintextMsgLog);

                if let Ok(result) = result {
                    assert!(!result.passed, "{:?}", result.output);
                } else {
                    let error = result.err().unwrap();
                    panic!("{:?}", error);
                }
            }
        }
    }
}
