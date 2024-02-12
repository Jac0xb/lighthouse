use anchor_lang::{
    prelude::borsh::{self, BorshDeserialize, BorshSerialize},
    Owners, Result,
};
use anchor_spl::token_interface::{
    self,
    spl_token_2022::state::{Account, AccountState},
};
use solana_program::{
    account_info::AccountInfo, program_option::COption, program_pack::Pack, pubkey::Pubkey,
};

use crate::{error::LighthouseError, Assert, EvaluationResult, Operator};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum TokenAccountDataField {
    Mint(Pubkey),
    Owner(Pubkey),
    Amount(u64),
    Delegate(Option<Pubkey>),
    State(u8),
    IsNative(Option<u64>),
    DelegatedAmount(u64),
    CloseAuthority(Option<Pubkey>),
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

impl Assert for TokenAccountDataField {
    fn evaluate(
        &self,
        account: &AccountInfo,
        operator: &Operator,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        if account.data_is_empty() {
            return Err(LighthouseError::AccountNotInitialized.into());
        }

        if !token_interface::TokenAccount::owners().contains(account.owner) {
            return Err(LighthouseError::AccountNotTokenAccount.into());
        }

        if account.data_len() < Account::LEN {
            return Err(LighthouseError::AccountNotTokenAccount.into());
        }

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
            TokenAccountDataField::Mint(pubkey) => {
                let mint_slice = &data[0..32];
                let mint = Pubkey::try_from(mint_slice).unwrap();

                operator.evaluate(&mint, pubkey, include_output)
            }
            TokenAccountDataField::Owner(pubkey) => {
                let owner_slice = &data[32..64];
                let owner = Pubkey::try_from(owner_slice).unwrap();

                operator.evaluate(&owner, pubkey, include_output)
            }
            TokenAccountDataField::Amount(amount) => {
                let amount_slice = &data[64..72];
                let actual_amount = u64::from_le_bytes(amount_slice.try_into().unwrap());

                operator.evaluate(&actual_amount, amount, include_output)
            }
            TokenAccountDataField::Delegate(assertion_pubkey) => {
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
            TokenAccountDataField::State(state) => {
                let actual_state = data[108];
                operator.evaluate(&actual_state, state, include_output)
            }
            TokenAccountDataField::IsNative(is_native) => {
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
            TokenAccountDataField::DelegatedAmount(delegated_amount) => {
                let delegated_amount_slice = &data[121..129];
                let actual_delegated_amount =
                    u64::from_le_bytes(delegated_amount_slice.try_into().unwrap());

                operator.evaluate(&actual_delegated_amount, delegated_amount, include_output)
            }
            TokenAccountDataField::CloseAuthority(pubkey) => {
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

pub fn unpack_coption_key(src: &[u8]) -> Result<COption<Pubkey>> {
    let tag = &src[0..4];
    let body = &src[4..36];

    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(
            body.try_into().unwrap(),
        ))),
        _ => Err(LighthouseError::AccountNotInitialized.into()),
    }
}

pub fn unpack_coption_u64(src: &[u8]) -> Result<COption<u64>> {
    let tag = &src[0..4];
    let body = &src[4..12];

    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(body.try_into().unwrap()))),
        _ => Err(LighthouseError::AccountNotInitialized.into()),
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

        use crate::{Assert, Operator, TokenAccountDataField};

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
            let result =
                TokenAccountDataField::Amount(69).evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result =
                TokenAccountDataField::Amount(1600).evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on mint
            //
            let result = TokenAccountDataField::Mint(mint.encodable_pubkey()).evaluate(
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

            //
            // Assert on owner
            //
            let result = TokenAccountDataField::Owner(owner.encodable_pubkey()).evaluate(
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

            //
            // Assert on delegate
            //

            let result = TokenAccountDataField::Delegate(None).evaluate(
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

            let result = TokenAccountDataField::Delegate(Some(owner.encodable_pubkey())).evaluate(
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
            // Assert on state
            //

            let result = TokenAccountDataField::State(AccountState::Initialized as u8).evaluate(
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

            let result = TokenAccountDataField::State(AccountState::Frozen as u8).evaluate(
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

            let result = TokenAccountDataField::State(AccountState::Uninitialized as u8).evaluate(
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
            // Assert on is_native
            //

            let result = TokenAccountDataField::IsNative(Some(1)).evaluate(
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

            //
            // Assert on delegated_amount
            //
            let result = TokenAccountDataField::DelegatedAmount(42).evaluate(
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

            //
            // Assert on close_authority
            //

            let result = TokenAccountDataField::CloseAuthority(None).evaluate(
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

            let result = TokenAccountDataField::CloseAuthority(Some(owner.encodable_pubkey()))
                .evaluate(&account_info, &Operator::Equal, true);

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

            let result = TokenAccountDataField::Delegate(None).evaluate(
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

            let result = TokenAccountDataField::Delegate(Some(delegate.encodable_pubkey()))
                .evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            //
            // Assert on close_authority
            //
            let result = TokenAccountDataField::CloseAuthority(None).evaluate(
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

            let result =
                TokenAccountDataField::CloseAuthority(Some(close_authority.encodable_pubkey()))
                    .evaluate(&account_info, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }
    }
}
