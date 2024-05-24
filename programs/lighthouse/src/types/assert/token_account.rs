use super::{Assert, EquatableOperator, Evaluate, IntegerOperator, LogLevel};
use crate::{
    error::LighthouseError,
    generate_asserts_c,
    utils::{checked_get_slice, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
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

        generate_asserts_c!(
            self,
            TokenAccountAssertion,
            data,
            log_level,
            (Mint, (Pubkey), 0),
            (Owner, (Pubkey), 32),
            (Amount, u64, 64),
            (Delegate, (Option<Pubkey>), 72),
            (State, u8, 108),
            (IsNative, (Option<u64>), 109),
            (DelegatedAmount, u64, 121),
            (CloseAuthority, (Option<Pubkey>), 129)
            (custom, TokenAccountOwnerIsDerived,
                {
                    let mint = bytemuck::from_bytes::<Pubkey>(checked_get_slice(&data, 0, 32)?);
                    let owner = bytemuck::from_bytes::<Pubkey>(checked_get_slice(&data, 32, 32)?);
                    let expected_ata =
                        get_associated_token_address_with_program_id(owner, mint, account.owner);

                    Pubkey::evaluate(
                        account.key,
                        &expected_ata,
                        &EquatableOperator::Equal,
                        log_level,
                    )
                }
            )
        )
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

        use crate::{
            test_utils::{assert_failed, assert_passed},
            types::assert::{
                Assert, EquatableOperator, IntegerOperator, LogLevel, TokenAccountAssertion,
            },
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

        #[test]
        fn negative_testing() {
            let pubkey = Pubkey::new_from_array([255; 32]);

            // Test mint
            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    mint: Pubkey::new_from_array([255; 32]),
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::Mint {
                    value: Pubkey::new_from_array([255; 32]),
                    operator: EquatableOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );

            // Test owner
            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    owner: Pubkey::new_from_array([255; 32]),
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::Owner {
                    value: Pubkey::new_from_array([255; 32]),
                    operator: EquatableOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );

            // Test amount
            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    amount: u64::MAX,
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::Amount {
                    value: u64::MAX,
                    operator: IntegerOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );

            // Test delegate
            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    delegate: COption::Some(Pubkey::new_from_array([255; 32])),
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::Delegate {
                    value: Some(Pubkey::new_from_array([255; 32])),
                    operator: EquatableOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );

            // Test state

            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    state: AccountState::Frozen,
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::State {
                    value: AccountState::Frozen as u8,
                    operator: IntegerOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );

            // Test is_native
            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];

            Account::pack(
                Account {
                    is_native: COption::Some(u64::MAX),
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::IsNative {
                    value: Some(u64::MAX),
                    operator: EquatableOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );

            // Test delegated_amount
            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    delegated_amount: u64::MAX,
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::DelegatedAmount {
                    value: u64::MAX,
                    operator: IntegerOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );

            // Test close_authority
            let serialized_token_account: &mut [u8; Account::LEN] = &mut [0u8; Account::LEN];
            Account::pack(
                Account {
                    close_authority: COption::Some(Pubkey::new_from_array([255; 32])),
                    ..Account::default()
                },
                serialized_token_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let account_info = AccountInfo::new(
                &pubkey,
                false,
                false,
                lamports_data,
                serialized_token_account,
                &spl_token_2022::ID,
                false,
                0,
            );

            assert_failed(
                TokenAccountAssertion::CloseAuthority {
                    value: Some(Pubkey::new_from_array([255; 32])),
                    operator: EquatableOperator::NotEqual,
                }
                .evaluate(&account_info, LogLevel::PlaintextMessage),
            );
        }
    }
}
