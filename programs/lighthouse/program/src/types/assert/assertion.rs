#![allow(non_snake_case)]

use crate::types::EquatableOperator;
use crate::types::{DataValueAssertion, EvaluationResult};
use crate::utils::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;

pub trait Assert<T> {
    fn evaluate(&self, parameters: &T, include_output: bool) -> Result<Box<EvaluationResult>>;
    fn format(&self) -> String;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AssertionConfigV1 {
    pub verbose: bool,
}

pub type AccountDataHashAssertionTuple = ([u8; 32], EquatableOperator, Option<u16>, Option<u16>);

// #[derive(Debug)]
// pub enum Assertion {
//     AccountInfoField(AccountInfoFieldAssertion),

//     // Account data offset, Borsh type, ComparableOperator
//     AccountData(AccountDataAssertionParameters),
//     AccountDataHash([u8; 32], EquatableOperator, Option<u16>, Option<u16>),
//     TokenAccountField(TokenAccountFieldAssertion),
//     MintAccountField(MintAccountFieldAssertion),
//     SysvarClockField(SysvarClockFieldAssertion),
// }

// impl Assertion {
//     pub fn format(&self) -> String {
//         match self {
//             Assertion::AccountData(offset, value) => {
//                 format!("AccountData[{}|{:?}]", offset, value)
//             }
//             Assertion::AccountDataHash(hash, ComparableOperator, start, end) => {
//                 format!(
//                     "AccountDataHash[{:?}|{:?}|({:?},{:?})]",
//                     hash, ComparableOperator, start, end
//                 )
//             }
//             Assertion::TokenAccountField(field) => {
//                 format!("TokenAccountField[{:?}]", field)
//             }
//             Assertion::MintAccountField(field) => {
//                 format!("MintAccountField[{:?}]", field)
//             }
//             Assertion::SysvarClockField(field) => {
//                 format!("SysvarClockField[{:?}]", field)
//             }
//             Assertion::AccountInfoField(fields) => {
//                 format!("AccountInfoField[{:?}]", fields)
//             }
//         }
//     }

//     pub fn evaluate(
//         &self,
//         target_account: &AccountInfo,
//         include_output: bool,
//     ) -> Result<Vec<EvaluationResult>> {
//         let mut results = Vec::new();

//         match &self {
//             Assertion::AccountData({
//                 offset,
//                 assertion
//             }) => {
//                 let account_data = target_account.try_borrow_data()?;

//                 Ok(vec![memory_value.evaluate_from_data_slice(
//                     account_data,
//                     (*account_offset) as usize,
//                     include_output,
//                 )?])
//             }
//             Assertion::AccountDataHash(account_hash_value, operator, start, end) => {
//                 let account_data = target_account.try_borrow_data()?;

//                 let start = start.unwrap_or(0);
//                 let end = end.unwrap_or(account_data.len() as u16);

//                 let account_data = &account_data[start as usize..end as usize];
//                 let account_hash = keccak::hashv(&[&account_data]).0;

//                 Ok(vec![operator.evaluate(
//                     &account_hash,
//                     account_hash_value,
//                     include_output,
//                 )])
//             }
//             Assertion::TokenAccountField(token_account_field) => Ok(vec![
//                 token_account_field.evaluate(target_account, include_output)?
//             ]),
//             Assertion::MintAccountField(mint_account_field) => Ok(vec![
//                 mint_account_field.evaluate(target_account, include_output)?
//             ]),
//             Assertion::SysvarClockField(clock_field) => {
//                 Ok(vec![clock_field.evaluate(&Clock::get()?, include_output)?])
//             }
//             Assertion::AccountInfoField(account_info_field) => Ok(vec![
//                 account_info_field.evaluate(target_account, include_output)?
//             ]),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use solana_program::{
//         account_info::AccountInfo, keccak, program_error::ProgramError, pubkey::Pubkey,
//     };
//     use solana_sdk::{signature::Keypair, signer::Signer};
//     use std::{cell::RefCell, rc::Rc};

//     use crate::{
//         error::{assert_is_program_error, LighthouseError},
//         types::{
//             AccountInfoFieldAssertion, Assertion, ComparableOperator, DataValueAssertion,
//             EquatableOperator,
//         },
//     };

//     #[test]
//     fn evaluate__data_hash() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let account_data: &mut [u8] = &mut [69u8; 10];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(account_data));

//         let data_hash = keccak::hashv(&[&[69u8; 10]]).0;

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountDataHash(data_hash, EquatableOperator::Equal, None, None);
//         let result = assertion.evaluate(&account_info, false).unwrap();
//         assert!(result.passed);
//     }

//     #[test]
//     fn evaluate__out_of_range() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion =
//             Assertion::AccountData(0, DataValueAssertion::U64(0, ComparableOperator::Equal));
//         let result = assertion.evaluate(&account_info, false);

//         assert_is_program_error(result.err().unwrap(), LighthouseError::OutOfRange.into());
//     }

//     #[test]
//     fn evaluate__fail_borrow_account_info() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 10];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };

//         let borrowed = account_info.try_borrow_mut_data().unwrap();

//         let assertion =
//             Assertion::AccountData(0, DataValueAssertion::U64(0, ComparableOperator::Equal));
//         let result = assertion.evaluate(&account_info, false);

//         drop(borrowed);

//         assert_is_program_error(result.err().unwrap(), ProgramError::AccountBorrowFailed)
//     }

//     #[test]
//     fn evaluate__account_data() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [69u8; 1];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion =
//             Assertion::AccountData(0, DataValueAssertion::U8(69, ComparableOperator::Equal));
//         let result = assertion.evaluate(&account_info, false).unwrap();
//         assert!(result.passed);

//         let assertion =
//             Assertion::AccountData(0, DataValueAssertion::U8(70, ComparableOperator::Equal));
//         let result = assertion.evaluate(&account_info, false).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_key() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let key = Keypair::new().pubkey();

//         let account_info = AccountInfo {
//             key: &key,
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Key(
//             key,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Key(
//             Pubkey::default(),
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_owner() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let key = Keypair::new().pubkey();

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &key,
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Owner(
//             key,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Owner(
//             Pubkey::default(),
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_lamports() {
//         let lamports_data: &mut u64 = &mut 69;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Lamports(
//             69,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Lamports(
//             1,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_data_length() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 128];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::DataLength(
//             128,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::DataLength(
//             129,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_executable() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: true,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Executable(
//             true,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::Executable(
//             false,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_is_signer() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: true,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::IsSigner(
//             true,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::IsSigner(
//             false,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_is_writable() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: true,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 0,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::IsWritable(
//             true,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::IsWritable(
//             false,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }

//     #[test]
//     fn evaluate__account_info_rent_epoch() {
//         let lamports_data: &mut u64 = &mut 0;
//         let lamports: RefCell<&mut u64> = RefCell::new(lamports_data);

//         let data_data: &mut [u8] = &mut [0u8; 0];
//         let data: Rc<RefCell<&mut [u8]>> = Rc::new(RefCell::new(data_data));

//         let account_info = AccountInfo {
//             key: &Pubkey::default(),
//             is_signer: false,
//             is_writable: false,
//             owner: &Pubkey::default(),
//             lamports: Rc::new(lamports),
//             rent_epoch: 69,
//             data,
//             executable: false,
//         };
//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::RentEpoch(
//             69,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfoField(AccountInfoFieldAssertion::RentEpoch(
//             1,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(!result.passed);
//     }
// }

// // Evaluate tests for each assertion type

// // AccountData

// // AccountDataOption
// // TokenAccountField
// // AccountInfoFieldAssertion
// // AccountInfoFieldAssertion::Key
// // AccountInfoFieldAssertion::Owner
// // AccountInfoFieldAssertion::Lamports
// // AccountInfoFieldAssertion::DataLength
// // AccountInfoFieldAssertion::Executable
// // AccountInfoFieldAssertion::IsSigner
// // AccountInfoFieldAssertion::IsWritable
// // AccountInfoFieldAssertion::RentEpoch

// // Test edge cases
// // AccountDataOption does not exist
// // AccountDataOption does exist

// // Test exceptions
