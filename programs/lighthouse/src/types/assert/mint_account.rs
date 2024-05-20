use super::{Assert, EquatableOperator, Evaluate, IntegerOperator, LogLevel};
use crate::error::LighthouseError;
use crate::generate_asserts_c;
use crate::utils::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum MintAccountAssertion {
    MintAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
    Supply {
        value: u64,
        operator: IntegerOperator,
    },
    Decimals {
        value: u8,
        operator: IntegerOperator,
    },
    IsInitialized {
        value: bool,
        operator: EquatableOperator,
    },
    FreezeAuthority {
        value: Option<Pubkey>,
        operator: EquatableOperator,
    },
}

impl Assert<&AccountInfo<'_>> for MintAccountAssertion {
    fn evaluate(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        let data = account
            .try_borrow_mut_data()
            .map_err(LighthouseError::failed_borrow_err)?;

        generate_asserts_c!(
            self,
            MintAccountAssertion,
            data,
            log_level,
            (MintAuthority, (Option<Pubkey>), 0),
            (Supply, u64, 36),
            (Decimals, u8, 44),
            (IsInitialized, bool, 45),
            (FreezeAuthority, (Option<Pubkey>), 46)
        )
    }
}

#[cfg(test)]
mod tests {
    mod evaluate {
        use solana_program::{
            account_info::AccountInfo, program_option::COption, program_pack::Pack,
        };
        use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
        use spl_token::state::Mint;
        use std::{cell::RefCell, rc::Rc};

        use crate::{
            test_utils::{assert_failed, assert_passed},
            types::assert::{
                Assert, EquatableOperator, IntegerOperator, LogLevel, MintAccountAssertion,
            },
        };

        #[test]
        fn evaluate_mint_account_no_mint_authority_no_freeze_authority() {
            let mint = Keypair::new();

            let serialized_mint_account: &mut [u8; Mint::LEN] = &mut [0u8; Mint::LEN];
            Mint::pack(
                Mint {
                    mint_authority: COption::None,
                    supply: 69,
                    decimals: 2,
                    is_initialized: true,
                    freeze_authority: COption::None,
                },
                serialized_mint_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

            let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(serialized_mint_account));

            let account_info = AccountInfo {
                key: &mint.encodable_pubkey(),
                is_signer: false,
                is_writable: false,
                owner: &spl_token_2022::ID,
                lamports: Rc::new(lamports),
                rent_epoch: 0,
                data,
                executable: false,
            };

            //
            // Assert on mint_authority
            //

            let result = MintAccountAssertion::MintAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = MintAccountAssertion::MintAuthority {
                value: Some(Keypair::new().encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
            //
            // Assert on supply
            //

            let result = MintAccountAssertion::Supply {
                value: 69,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = MintAccountAssertion::Supply {
                value: 1600,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
            //
            // Assert on decimals
            //

            let result = MintAccountAssertion::Decimals {
                value: 2,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = MintAccountAssertion::Decimals {
                value: 3,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
            //
            // Assert on is_initialized
            //

            let result = MintAccountAssertion::IsInitialized {
                value: true,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = MintAccountAssertion::IsInitialized {
                value: false,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
            //
            // Assert on freeze_authority
            //

            let result = MintAccountAssertion::FreezeAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_passed(result);

            let result = MintAccountAssertion::FreezeAuthority {
                value: Some(Keypair::new().encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
        }

        #[test]
        fn evaluate_mint_account_some_mint_authority_some_freeze_authority() {
            let mint = Keypair::new();
            let mint_authority = Keypair::new();
            let freeze_authority = Keypair::new();

            let serialized_mint_account: &mut [u8; Mint::LEN] = &mut [0u8; Mint::LEN];
            Mint::pack(
                Mint {
                    mint_authority: COption::Some(mint_authority.encodable_pubkey()),
                    supply: 69,
                    decimals: 2,
                    is_initialized: true,
                    freeze_authority: COption::Some(freeze_authority.encodable_pubkey()),
                },
                serialized_mint_account,
            )
            .unwrap();

            let lamports_data: &mut u64 = &mut 0;
            let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

            let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(serialized_mint_account));

            let account_info = AccountInfo {
                key: &mint.encodable_pubkey(),
                is_signer: false,
                is_writable: false,
                owner: &spl_token_2022::ID,
                lamports: Rc::new(lamports),
                rent_epoch: 0,
                data,
                executable: false,
            };

            //
            // Assert on mint_authority
            //

            let result = MintAccountAssertion::MintAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
            let result = MintAccountAssertion::MintAuthority {
                value: Some(freeze_authority.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
            //
            // Assert on freeze_authority
            //

            let result = MintAccountAssertion::FreezeAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
            let result = MintAccountAssertion::FreezeAuthority {
                value: Some(mint_authority.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, LogLevel::PlaintextMessage);

            assert_failed(result);
        }
    }
}
