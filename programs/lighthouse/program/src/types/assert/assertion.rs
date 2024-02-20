#![allow(non_snake_case)]

use crate::types::{DataValueAssertion, EvaluationResult};
use crate::types::{EquatableOperator, Operator};
use crate::utils::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::keccak;

pub trait Assert<T> {
    fn evaluate(&self, parameters: &T, include_output: bool) -> Result<Box<EvaluationResult>>;
    fn format(&self) -> String;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AssertionConfigV1 {
    pub verbose: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountDataHashAssertion {
    pub hash: [u8; 32],
    pub operator: EquatableOperator,
    pub start: Option<u16>,
    pub end: Option<u16>,
}

impl Assert<AccountInfo<'_>> for AccountDataHashAssertion {
    fn format(&self) -> String {
        format!(
            "AccountDataHashAssertion[{:?}|{:?}|({:?},{:?})]",
            self.hash, self.operator, self.start, self.end
        )
    }

    fn evaluate(
        &self,
        account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        let AccountDataHashAssertion {
            hash: account_hash_value,
            operator,
            start,
            end,
        } = self;

        let account_data = account.try_borrow_data()?;

        let start = start.unwrap_or(0);
        let end = end.unwrap_or(account_data.len() as u16);

        let account_data = &account_data[start as usize..end as usize];
        let account_hash = keccak::hashv(&[&account_data]).0;

        Ok(operator.evaluate(&account_hash, account_hash_value, include_output))
    }
}

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
//             AccountInfoAssertion, Assertion, ComparableOperator, DataValueAssertion,
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Key(
//             key,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Key(
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Owner(
//             key,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Owner(
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Lamports(
//             69,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Lamports(
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::DataLength(
//             128,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::DataLength(
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Executable(
//             true,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::Executable(
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::IsSigner(
//             true,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::IsSigner(
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::IsWritable(
//             true,
//             EquatableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::IsWritable(
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
//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::RentEpoch(
//             69,
//             ComparableOperator::Equal,
//         ));
//         let result = assertion.evaluate(&account_info, true).unwrap();
//         assert!(result.passed);

//         let assertion = Assertion::AccountInfo(AccountInfoAssertion::RentEpoch(
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
// // TokenAccount
// // AccountInfoAssertion
// // AccountInfoAssertion::Key
// // AccountInfoAssertion::Owner
// // AccountInfoAssertion::Lamports
// // AccountInfoAssertion::DataLength
// // AccountInfoAssertion::Executable
// // AccountInfoAssertion::IsSigner
// // AccountInfoAssertion::IsWritable
// // AccountInfoAssertion::RentEpoch

// // Test edge cases
// // AccountDataOption does not exist
// // AccountDataOption does exist

// // Test exceptions
