use super::{evaluate_bytes, Assert, Evaluate, LogLevel};
use crate::{
    err, err_msg,
    error::LighthouseError,
    types::assert::evaluate::EquatableOperator,
    utils::{unpack_coption_key, unpack_coption_u64, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
use lighthouse_common::{
    assertion_settings::{AssertionSettings, DataValue},
    integer_operator::IntegerOperator,
};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
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
        operator: IntegerOperator,
    },
    Delegate {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    State {
        value: u8,
        operator: IntegerOperator,
    },
    // This is an EquatableOperator because current iteration of IntegerOperator doesn't make sense for a Option<u64>
    IsNative {
        value: Option<u64>,
        operator: EquatableOperator,
    },
    DelegatedAmount {
        value: u64,
        operator: IntegerOperator,
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
    fn evaluate(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        let data = account
            .try_borrow_data()
            .map_err(LighthouseError::failed_borrow_err)?;

        match self {
            TokenAccountAssertion::Mint {
                value: assertion_value,
                operator,
            } => {
                let data_slice = data
                    .get(0..32)
                    .ok_or_else(|| LighthouseError::oob_err(0..32))?;
                let actual_mint = bytemuck::from_bytes::<Pubkey>(data_slice);

                Pubkey::evaluate(actual_mint, assertion_value, operator, log_level)
            }
            TokenAccountAssertion::Owner {
                value: assertion_value,
                operator,
            } => {
                let data_slice = data
                    .get(32..64)
                    .ok_or_else(|| LighthouseError::oob_err(32..64))?;
                let actual_owner = bytemuck::from_bytes::<Pubkey>(data_slice);

                Pubkey::evaluate(actual_owner, assertion_value, operator, log_level)
            }
            TokenAccountAssertion::Amount {
                value: assertion_value,
                operator,
            } => {
                let data_slice = data
                    .get(64..72)
                    .ok_or_else(|| LighthouseError::oob_err(64..72))?;
                let actual_amount = u64::from_le_bytes(data_slice.try_into().map_err(|e| {
                    err_msg!("Failed to deserialize amount from account data", e);
                    err!(LighthouseError::FailedToDeserialize)
                })?);

                u64::evaluate(&actual_amount, assertion_value, operator, log_level)
            }
            TokenAccountAssertion::Delegate {
                value: assertion_value,
                operator,
            } => {
                let data_slice = data
                    .get(72..108)
                    .ok_or_else(|| LighthouseError::oob_err(72..108))?;
                let delegate = unpack_coption_key(data_slice)?;

                let some_bytes = [1u8, 0, 0, 0];
                let none_bytes = [0u8, 0, 0, 0];

                evaluate_bytes(
                    &data_slice[0..4],
                    if delegate.is_some() {
                        &some_bytes
                    } else {
                        &none_bytes
                    },
                    &AssertionSettings {
                        is_big_endian: true,
                        operator: IntegerOperator::try_from(*operator as u8).unwrap(),
                        data_value: DataValue::Bytes,
                    },
                    log_level,
                )

                evaluate_bytes(&data_slice[5..37], &assertion_value.as_ref().to_bytes(), &AssertionSettings {
                    is_big_endian: false,
                    operator: IntegerOperator::Equal,
                    data_value: DataValue::Bytes,
                }, log_level)

                // <Option<&Pubkey>>::evaluate(
                //     &delegate,
                //     &assertion_value.as_ref(),
                //     operator,
                //     log_level,
                // )
            }
            TokenAccountAssertion::State {
                value: assertion_value,
                operator,
            } => {
                let actual_state = data
                    .get(108)
                    .ok_or_else(|| LighthouseError::oob_err(108..109))?;

                u8::evaluate(actual_state, assertion_value, operator, log_level)
            }
            TokenAccountAssertion::IsNative { value, operator } => {
                let data_slice = data
                    .get(109..121)
                    .ok_or_else(|| LighthouseError::oob_err(109..121))?;

                let actual_is_native = unpack_coption_u64(data_slice)?;

                <Option<u64>>::evaluate(&actual_is_native, value, operator, log_level)
            }
            TokenAccountAssertion::DelegatedAmount {
                value: assertion_value,
                operator,
            } => {
                let data_slice = data
                    .get(121..129)
                    .ok_or_else(|| LighthouseError::oob_err(121..129))?;

                let actual_delegated_amount =
                    u64::from_le_bytes(data_slice.try_into().map_err(|e| {
                        err_msg!("Failed to deserialize delegatedamount from account data", e);
                        err!(LighthouseError::FailedToDeserialize)
                    })?);

                u64::evaluate(
                    &actual_delegated_amount,
                    assertion_value,
                    operator,
                    log_level,
                )
            }
            TokenAccountAssertion::CloseAuthority { value, operator } => {
                let data_slice = data
                    .get(129..165)
                    .ok_or_else(|| LighthouseError::oob_err(129..165))?;
                let close_authority = unpack_coption_key(data_slice)?;

                let some_bytes = [1u8, 0, 0, 0];
                let none_bytes = [0u8, 0, 0, 0];

                evaluate_bytes(
                    &data_slice[0..4],
                    if close_authority.is_some() {
                        &some_bytes
                    } else {
                        &none_bytes
                    },
                    &AssertionSettings {
                        is_big_endian: true,
                        operator: IntegerOperator::try_from(*operator as u8).unwrap(),
                        data_value: DataValue::Bytes,
                    },
                    log_level,
                )

                // <Option<&Pubkey>>::evaluate(&close_authority, &value.as_ref(), operator, log_level)
            }
            TokenAccountAssertion::TokenAccountOwnerIsDerived => {
                let mint_data = data
                    .get(0..32)
                    .ok_or_else(|| LighthouseError::oob_err(0..32))?;
                let mint = bytemuck::from_bytes::<Pubkey>(mint_data);

                let owner_data = data
                    .get(32..64)
                    .ok_or_else(|| LighthouseError::oob_err(32..64))?;
                let owner = bytemuck::from_bytes::<Pubkey>(owner_data);

                let expected_ata =
                    get_associated_token_address_with_program_id(owner, mint, account.owner);

                Pubkey::evaluate(
                    account.key,
                    &expected_ata,
                    &EquatableOperator::Equal,
                    log_level,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod evaluate {
        use lighthouse_common::integer_operator::IntegerOperator;
        use solana_program::{
            account_info::AccountInfo, program_option::COption, program_pack::Pack, pubkey::Pubkey,
        };
        use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
        use spl_associated_token_account::get_associated_token_address_with_program_id;
        use spl_token_2022::state::{Account, AccountState};
        use std::{cell::RefCell, rc::Rc};

        use crate::{
            test_utils::{assert_failed, assert_passed},
            types::assert::{evaluate::EquatableOperator, Assert, LogLevel, TokenAccountAssertion},
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
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = TokenAccountAssertion::Amount {
                value: 1600,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);

            //
            // Assert on mint
            //
            let result = TokenAccountAssertion::Mint {
                value: mint.encodable_pubkey(),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            //
            // Assert on owner
            //
            let result = TokenAccountAssertion::Owner {
                value: owner.encodable_pubkey(),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            //
            // Assert on delegate
            //

            let result = TokenAccountAssertion::Delegate {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = TokenAccountAssertion::Delegate {
                value: Some(owner.encodable_pubkey()),
                operator: EquatableOperator::NotEqual,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            //
            // Assert on state
            //

            let result = TokenAccountAssertion::State {
                value: AccountState::Initialized as u8,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = TokenAccountAssertion::State {
                value: AccountState::Frozen as u8,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);

            let result = TokenAccountAssertion::State {
                value: AccountState::Uninitialized as u8,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);

            //
            // Assert on is_native
            //

            let result = TokenAccountAssertion::IsNative {
                value: Some(1),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            //
            // Assert on delegated_amount
            //
            let result = TokenAccountAssertion::DelegatedAmount {
                value: 42,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            //
            // Assert on close_authority
            //

            let result = TokenAccountAssertion::CloseAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = TokenAccountAssertion::CloseAuthority {
                value: Some(owner.encodable_pubkey()),
                operator: EquatableOperator::NotEqual,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
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
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);

            let result = TokenAccountAssertion::Delegate {
                value: Some(delegate.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            //
            // Assert on close_authority
            //
            let result = TokenAccountAssertion::CloseAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);

            let result = TokenAccountAssertion::CloseAuthority {
                value: Some(close_authority.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);
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
                    .evaluate(&account_info, LogLevel::PlaintextMessage);

                assert_passed(result);
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
                    .evaluate(&account_info, LogLevel::PlaintextMessage);

                assert_failed(result);
            }
        }

        #[test]
        fn evaluate_option() {
            let mint = Keypair::new();
            let owner = Keypair::new();
            let delegate = Keypair::new();
            let close_authority = Keypair::new();

            //
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
                let key = Keypair::new().encodable_pubkey();
                let account_info = AccountInfo::new(
                    &key,
                    false,
                    false,
                    lamports_data,
                    serialized_token_account,
                    &spl_token_2022::ID,
                    false,
                    0,
                );

                let result = TokenAccountAssertion::IsNative {
                    value: Some(1),
                    operator: EquatableOperator::Equal,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage);

                assert_passed(result);

                let result = TokenAccountAssertion::IsNative {
                    value: None,
                    operator: EquatableOperator::Equal,
                };

                assert_failed(result.evaluate(&account_info, LogLevel::PlaintextMessage));
            }
        }
    }
}
