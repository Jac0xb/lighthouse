use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, keccak, pubkey::Pubkey};

use crate::{
    error::LighthouseError,
    types::{
        Assert, ComparableOperator, EquatableOperator, EvaluationResult, KnownProgram, Operator,
    },
    utils::Result,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountInfoData {
    pub key: Pubkey,
    pub lamports: u64,
    pub data_length: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub executable: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AccountInfoAssertion {
    Key {
        value: Pubkey,
        operator: EquatableOperator,
    },
    Lamports {
        value: u64,
        operator: ComparableOperator,
    },
    DataLength {
        value: u64,
        operator: ComparableOperator,
    },
    Owner {
        value: Pubkey,
        operator: EquatableOperator,
    },
    KnownOwner {
        value: KnownProgram,
        operator: EquatableOperator,
    },
    RentEpoch {
        value: u64,
        operator: ComparableOperator,
    },
    IsSigner {
        value: bool,
        operator: EquatableOperator,
    },
    IsWritable {
        value: bool,
        operator: EquatableOperator,
    },
    Executable {
        value: bool,
        operator: EquatableOperator,
    },
    VerifyDatahash {
        expected_hash: [u8; 32],
        start: Option<u16>,
        length: Option<u16>,
    },
}

impl Assert<AccountInfo<'_>> for AccountInfoAssertion {
    fn format(&self) -> String {
        format!("AccountInfoAssertion[{:?}]", self)
    }

    fn evaluate(
        &self,
        account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        let result = match self {
            AccountInfoAssertion::Key { value, operator } => {
                operator.evaluate(account.unsigned_key(), value, include_output)
            }
            AccountInfoAssertion::Owner { value, operator } => {
                operator.evaluate(account.owner, value, include_output)
            }
            AccountInfoAssertion::Lamports { value, operator } => {
                operator.evaluate(&account.try_lamports()?, value, include_output)
            }
            AccountInfoAssertion::DataLength { value, operator } => {
                operator.evaluate(&(account.data_len() as u64), value, include_output)
            }
            AccountInfoAssertion::Executable { value, operator } => {
                operator.evaluate(&account.executable, value, include_output)
            }
            AccountInfoAssertion::IsSigner { value, operator } => {
                operator.evaluate(&account.is_signer, value, include_output)
            }
            AccountInfoAssertion::IsWritable { value, operator } => {
                operator.evaluate(&account.is_writable, value, include_output)
            }
            AccountInfoAssertion::RentEpoch { value, operator } => {
                operator.evaluate(&account.rent_epoch as &u64, value, include_output)
            }
            AccountInfoAssertion::VerifyDatahash {
                expected_hash,
                start,
                length,
            } => {
                let account_data = account.try_borrow_data()?;

                let start = start.unwrap_or(0);
                let length = length.unwrap_or(
                    account_data
                        .len()
                        .checked_sub(start as usize)
                        .ok_or(LighthouseError::OutOfRange)? as u16,
                );

                let account_data = &account_data[start as usize..(start + length) as usize];
                let actual_hash = keccak::hashv(&[&account_data]).0;

                EquatableOperator::Equal.evaluate(&actual_hash, expected_hash, include_output)
            }
            AccountInfoAssertion::KnownOwner { value, operator } => {
                operator.evaluate(account.owner, &value.to_pubkey(), include_output)
            }
        };

        Ok(result)
    }
}
