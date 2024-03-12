use super::{Assert, KnownProgram, LogLevel};
use crate::{
    error::LighthouseError,
    types::assert::operator::{ComparableOperator, EquatableOperator, Operator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, keccak, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
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

impl Assert<&AccountInfo<'_>> for AccountInfoAssertion {
    fn evaluate(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        match self {
            AccountInfoAssertion::Key { value, operator } => {
                operator.evaluate(account.unsigned_key(), value, log_level)
            }
            AccountInfoAssertion::Owner { value, operator } => {
                operator.evaluate(account.owner, value, log_level)
            }
            AccountInfoAssertion::Lamports { value, operator } => {
                operator.evaluate(&account.try_lamports()?, value, log_level)
            }
            AccountInfoAssertion::DataLength { value, operator } => {
                operator.evaluate(&(account.data_len() as u64), value, log_level)
            }
            AccountInfoAssertion::Executable { value, operator } => {
                operator.evaluate(&account.executable, value, log_level)
            }
            AccountInfoAssertion::IsSigner { value, operator } => {
                operator.evaluate(&account.is_signer, value, log_level)
            }
            AccountInfoAssertion::IsWritable { value, operator } => {
                operator.evaluate(&account.is_writable, value, log_level)
            }
            AccountInfoAssertion::RentEpoch { value, operator } => {
                operator.evaluate(&account.rent_epoch as &u64, value, log_level)
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
                        .ok_or(LighthouseError::RangeOutOfBounds)? as u16,
                );

                let account_data = &account_data[start as usize..(start + length) as usize];
                let actual_hash = keccak::hashv(&[&account_data]).0;

                EquatableOperator::Equal.evaluate(&actual_hash, expected_hash, log_level)
            }
            AccountInfoAssertion::KnownOwner { value, operator } => {
                operator.evaluate(account.owner, &value.to_pubkey(), log_level)
            }
        }
    }
}
