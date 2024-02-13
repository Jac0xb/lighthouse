use anchor_lang::{
    prelude::borsh::{self, BorshDeserialize, BorshSerialize},
    Owners, Result,
};
use anchor_spl::token_interface::{self};
use solana_program::{account_info::AccountInfo, program_option::COption, pubkey::Pubkey};

use crate::{
    error::LighthouseError, utils::unpack_coption_key, Assert, EvaluationResult, Operator,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum MintAccountField {
    MintAuthority(Option<Pubkey>),
    Supply(u64),
    Decimals(u8),
    IsInitialized(bool),
    FreezeAuthority(Option<Pubkey>),
}

impl Assert<AccountInfo<'_>> for MintAccountField {
    fn evaluate(
        &self,
        account: &AccountInfo,
        operator: &Operator,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        if account.data_is_empty() {
            return Err(LighthouseError::AccountNotInitialized.into());
        }

        if !token_interface::Mint::owners().contains(account.owner) {
            return Err(LighthouseError::OwnerMismatch.into());
        }

        // TODO: Logic to assert on if account is a mint account

        let data = account.try_borrow_mut_data().unwrap();

        // let (mint, owner, amount, delegate, state, is_native, delegated_amount, close_authority) =
        //     array_refs![src, 32, 32, 8, 36, 1, 12, 8, 36];
        // Ok(Account {
        //     mint: Pubkey::new_from_array(*mint),
        //     owner: Pubkey::new_from_array(*owner),
        //     amount: u64::from_le_bytes(*amount),
        //     delegate: unpack_coption_key(delegate)?,
        //     state: AccountState::try_from_primitive(state[0])
        //         .or(Err(ProgramError::InvalidAccountData))?,
        //     is_native: unpack_coption_u64(is_native)?,
        //     delegated_amount: u64::from_le_bytes(*delegated_amount),
        //     close_authority: unpack_coption_key(close_authority)?,
        // });

        let result = match self {
            MintAccountField::MintAuthority(pubkey) => {
                let mint_authority_slice = &data[0..36];
                let mint_authority = unpack_coption_key(mint_authority_slice)?;

                match (mint_authority, pubkey) {
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
            MintAccountField::Supply(supply) => {
                let supply_slice = &data[36..44];
                let actual_supply = u64::from_le_bytes(supply_slice.try_into().unwrap());

                operator.evaluate(&actual_supply, supply, include_output)
            }
            MintAccountField::Decimals(decimals) => {
                let decimals_slice = &data[44..45];
                let actual_decimals = u8::from_le_bytes(decimals_slice.try_into().unwrap());

                operator.evaluate(&actual_decimals, decimals, include_output)
            }
            MintAccountField::IsInitialized(is_initialized) => {
                let actual_is_initialized = (data[45]) != 0;

                operator.evaluate(&actual_is_initialized, is_initialized, include_output)
            }
            MintAccountField::FreezeAuthority(pubkey) => {
                let freeze_authority_slice = &data[46..82];

                let freeze_authority = unpack_coption_key(freeze_authority_slice)?;

                match (freeze_authority, pubkey) {
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

// pub fn unpack_coption_key(src: &[u8]) -> Result<COption<Pubkey>> {
//     let tag = &src[0..4];
//     let body = &src[4..36];

//     match *tag {
//         [0, 0, 0, 0] => Ok(COption::None),
//         [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(
//             body.try_into().unwrap(),
//         ))),
//         _ => Err(LighthouseError::AccountNotInitialized.into()),
//     }
// }

// pub fn unpack_coption_u64(src: &[u8]) -> Result<COption<u64>> {
//     let tag = &src[0..4];
//     let body = &src[4..12];

//     match *tag {
//         [0, 0, 0, 0] => Ok(COption::None),
//         [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(body.try_into().unwrap()))),
//         _ => Err(LighthouseError::AccountNotInitialized.into()),
//     }
// }

#[cfg(test)]
mod tests {
    mod evaluate {
        use anchor_spl::token_interface::spl_token_2022::{self};
        use solana_program::{
            account_info::AccountInfo, program_option::COption, program_pack::Pack,
        };
        use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
        use spl_token::state::Mint;
        use std::{cell::RefCell, rc::Rc};

        use crate::{Assert, MintAccountField, Operator};

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

            let result = MintAccountField::MintAuthority(None).evaluate(
                &account_info,
                &Operator::Equal,
                true,
            );

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountField::MintAuthority(Some(Keypair::new().encodable_pubkey()))
                .evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on supply
            //

            let result =
                MintAccountField::Supply(69).evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result =
                MintAccountField::Supply(1600).evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on decimals
            //

            let result =
                MintAccountField::Decimals(2).evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result =
                MintAccountField::Decimals(3).evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on is_initialized
            //

            let result = MintAccountField::IsInitialized(true).evaluate(
                &account_info,
                &Operator::Equal,
                true,
            );

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountField::IsInitialized(false).evaluate(
                &account_info,
                &Operator::Equal,
                true,
            );

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on freeze_authority
            //

            let result = MintAccountField::FreezeAuthority(None).evaluate(
                &account_info,
                &Operator::Equal,
                true,
            );

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountField::FreezeAuthority(Some(Keypair::new().encodable_pubkey()))
                .evaluate(&account_info, &Operator::Equal, true);

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

            let result = MintAccountField::MintAuthority(None).evaluate(
                &account_info,
                &Operator::Equal,
                true,
            );

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountField::MintAuthority(Some(freeze_authority.encodable_pubkey()))
                .evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on freeze_authority
            //

            let result = MintAccountField::FreezeAuthority(None).evaluate(
                &account_info,
                &Operator::Equal,
                true,
            );

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = MintAccountField::FreezeAuthority(Some(mint_authority.encodable_pubkey()))
                .evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }
    }
}
