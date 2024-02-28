use crate::{
    types::{EquatableOperator, IntegerOperator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_option::COption, pubkey::Pubkey};

use crate::{
    error::LighthouseError,
    types::{Assert, EvaluationResult, Operator},
    utils::unpack_coption_key,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
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

impl Assert<AccountInfo<'_>> for MintAccountAssertion {
    fn format(&self) -> String {
        format!("MintAccountAssertion[{:?}]", self)
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
            return Err(LighthouseError::AccountOwnerMismatch.into());
        }

        // TODO: Logic to assert on if account is a mint account
        let data = account.try_borrow_mut_data().unwrap();

        let result = match self {
            MintAccountAssertion::MintAuthority {
                value: assertion_value,
                operator,
            } => {
                let mint_authority_slice = &data[0..36];
                let mint_authority = unpack_coption_key(mint_authority_slice)?;

                match (mint_authority, assertion_value) {
                    (COption::None, None) => Box::new(EvaluationResult {
                        passed: true,
                        output: "None == None".to_string(),
                    }),
                    (COption::Some(mint_authority), None) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("{:?} != None", mint_authority),
                    }),
                    (COption::None, Some(pubkey)) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("None != {:?}", pubkey),
                    }),
                    (COption::Some(mint_authority), Some(pubkey)) => {
                        operator.evaluate(&mint_authority, pubkey, include_output)
                    }
                }
            }
            MintAccountAssertion::Supply {
                value: assertion_value,
                operator,
            } => {
                let supply_slice = &data[36..44];
                let actual_supply = u64::from_le_bytes(supply_slice.try_into().unwrap());

                operator.evaluate(&actual_supply, assertion_value, include_output)
            }
            MintAccountAssertion::Decimals {
                value: assertion_value,
                operator,
            } => {
                let decimals_slice = &data[44..45];
                let actual_decimals = u8::from_le_bytes(decimals_slice.try_into().unwrap());

                operator.evaluate(&actual_decimals, assertion_value, include_output)
            }
            MintAccountAssertion::IsInitialized {
                value: assertion_value,
                operator,
            } => {
                let actual_is_initialized = (data[45]) != 0;

                operator.evaluate(&actual_is_initialized, assertion_value, include_output)
            }
            MintAccountAssertion::FreezeAuthority {
                value: assertion_value,
                operator,
            } => {
                let freeze_authority_slice = &data[46..82];

                let freeze_authority = unpack_coption_key(freeze_authority_slice)?;

                match (freeze_authority, assertion_value) {
                    (COption::None, None) => Box::new(EvaluationResult {
                        passed: true,
                        output: "None == None".to_string(),
                    }),
                    (COption::Some(freeze_authority), None) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("{:?} != None", freeze_authority),
                    }),
                    (COption::None, Some(pubkey)) => Box::new(EvaluationResult {
                        passed: false,
                        output: format!("None != {:?}", pubkey),
                    }),
                    (COption::Some(freeze_authority), Some(pubkey)) => {
                        operator.evaluate(&freeze_authority, pubkey, include_output)
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
        use solana_program::{
            account_info::AccountInfo, program_option::COption, program_pack::Pack,
        };
        use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
        use spl_token::state::Mint;
        use std::{cell::RefCell, rc::Rc};

        use crate::types::{Assert, EquatableOperator, IntegerOperator, MintAccountAssertion};

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
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountAssertion::MintAuthority {
                value: Some(Keypair::new().encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on supply
            //

            let result = MintAccountAssertion::Supply {
                value: 69,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountAssertion::Supply {
                value: 1600,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on decimals
            //

            let result = MintAccountAssertion::Decimals {
                value: 2,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountAssertion::Decimals {
                value: 3,
                operator: IntegerOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on is_initialized
            //

            let result = MintAccountAssertion::IsInitialized {
                value: true,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountAssertion::IsInitialized {
                value: false,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on freeze_authority
            //

            let result = MintAccountAssertion::FreezeAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountAssertion::FreezeAuthority {
                value: Some(Keypair::new().encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
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
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountAssertion::MintAuthority {
                value: Some(freeze_authority.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on freeze_authority
            //

            let result = MintAccountAssertion::FreezeAuthority {
                value: None,
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountAssertion::FreezeAuthority {
                value: Some(mint_authority.encodable_pubkey()),
                operator: EquatableOperator::Equal,
            }
            .evaluate(&account_info, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }
    }
}
