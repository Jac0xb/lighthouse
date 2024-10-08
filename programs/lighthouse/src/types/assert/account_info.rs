use super::{Assert, EquatableOperator, IntegerOperator, KnownProgram, LogLevel};
use crate::{
    types::assert::evaluate::Evaluate,
    utils::{checked_get_slice, Result},
};
use borsh::{BorshDeserialize, BorshSerialize};
use lighthouse_common::CompactU64;
use solana_program::{account_info::AccountInfo, keccak, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum AccountInfoAssertion {
    Lamports {
        value: u64,
        operator: IntegerOperator,
    },
    DataLength {
        value: u64,
        operator: IntegerOperator,
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
        operator: IntegerOperator,
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
        start: CompactU64,
        length: CompactU64,
    },
}

impl Assert<&AccountInfo<'_>> for AccountInfoAssertion {
    fn evaluate(&self, account: &AccountInfo<'_>, log_level: LogLevel) -> Result<()> {
        match self {
            AccountInfoAssertion::Owner { value, operator } => {
                Pubkey::evaluate(account.owner, value, operator, log_level)
            }
            AccountInfoAssertion::KnownOwner { value, operator } => {
                Pubkey::evaluate(account.owner, &value.to_pubkey(), operator, log_level)
            }
            AccountInfoAssertion::Lamports { value, operator } => {
                u64::evaluate(&account.try_lamports()?, value, operator, log_level)
            }
            AccountInfoAssertion::DataLength { value, operator } => {
                u64::evaluate(&(account.data_len() as u64), value, operator, log_level)
            }
            AccountInfoAssertion::Executable { value, operator } => {
                bool::evaluate(&account.executable, value, operator, log_level)
            }
            AccountInfoAssertion::IsSigner { value, operator } => {
                bool::evaluate(&account.is_signer, value, operator, log_level)
            }
            AccountInfoAssertion::IsWritable { value, operator } => {
                bool::evaluate(&account.is_writable, value, operator, log_level)
            }
            AccountInfoAssertion::RentEpoch { value, operator } => {
                u64::evaluate(&account.rent_epoch as &u64, value, operator, log_level)
            }
            AccountInfoAssertion::VerifyDatahash {
                expected_hash,
                start,
                length,
            } => {
                let account_data = account.try_borrow_data()?;
                let account_data =
                    checked_get_slice(&account_data, **start as usize, **length as usize)?;
                let actual_hash = keccak::hashv(&[&account_data]).0;

                <[u8]>::evaluate(
                    &actual_hash,
                    expected_hash,
                    &EquatableOperator::Equal,
                    log_level,
                )
            }
        }
    }
}
