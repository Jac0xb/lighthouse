use std::fmt::Display;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use crate::{
    types::{Assert, ComparableOperator, EquatableOperator, EvaluationResult, Operator},
    utils::Result,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct AccountInfoData {
    pub key: Pubkey,
    pub lamports: u64,
    pub data_length: u64,
    pub owner: Pubkey,
    pub rent_epoch: u64,
    pub is_signer: bool,
    pub is_writable: bool,
    pub executable: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AccountInfoFieldAssertion {
    Key(Pubkey, EquatableOperator),
    Lamports(u64, ComparableOperator),
    DataLength(u64, ComparableOperator),
    Owner(Pubkey, EquatableOperator),
    RentEpoch(u64, ComparableOperator),
    IsSigner(bool, EquatableOperator),
    IsWritable(bool, EquatableOperator),
    Executable(bool, EquatableOperator),
}

impl Assert<AccountInfo<'_>> for AccountInfoFieldAssertion {
    fn format(&self) -> String {
        format!("AccountInfoFieldAssertion[{:?}]", self)
    }

    fn evaluate(
        &self,
        account: &AccountInfo,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        let result = match self {
            AccountInfoFieldAssertion::Key(pubkey, operator) => {
                operator.evaluate(account.unsigned_key(), pubkey, include_output)
            }
            AccountInfoFieldAssertion::Owner(pubkey, operator) => {
                operator.evaluate(account.owner, pubkey, include_output)
            }
            AccountInfoFieldAssertion::Lamports(lamports, operator) => {
                operator.evaluate(&account.try_lamports()?, lamports, include_output)
            }
            AccountInfoFieldAssertion::DataLength(data_length, operator) => {
                operator.evaluate(&(account.data_len() as u64), data_length, include_output)
            }
            AccountInfoFieldAssertion::Executable(executable, operator) => {
                operator.evaluate(&account.executable, executable, include_output)
            }
            AccountInfoFieldAssertion::IsSigner(is_signer, operator) => {
                operator.evaluate(&account.is_signer, is_signer, include_output)
            }
            AccountInfoFieldAssertion::IsWritable(is_writable, operator) => {
                operator.evaluate(&account.is_writable, is_writable, include_output)
            }
            AccountInfoFieldAssertion::RentEpoch(rent_epoch, operator) => {
                operator.evaluate(&account.rent_epoch as &u64, rent_epoch, include_output)
            }
        };

        Ok(result)
    }
}
