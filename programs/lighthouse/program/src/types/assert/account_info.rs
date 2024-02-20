use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, keccak, pubkey::Pubkey};

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
pub enum AccountInfoAssertion {
    Key(Pubkey, EquatableOperator),
    Lamports(u64, ComparableOperator),
    DataLength(u64, ComparableOperator),
    Owner(Pubkey, EquatableOperator),
    RentEpoch(u64, ComparableOperator),
    IsSigner(bool, EquatableOperator),
    IsWritable(bool, EquatableOperator),
    Executable(bool, EquatableOperator),
    VerifyDatahash([u8; 32], Option<u16>, Option<u16>),
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
            AccountInfoAssertion::Key(pubkey, operator) => {
                operator.evaluate(account.unsigned_key(), pubkey, include_output)
            }
            AccountInfoAssertion::Owner(pubkey, operator) => {
                operator.evaluate(account.owner, pubkey, include_output)
            }
            AccountInfoAssertion::Lamports(lamports, operator) => {
                operator.evaluate(&account.try_lamports()?, lamports, include_output)
            }
            AccountInfoAssertion::DataLength(data_length, operator) => {
                operator.evaluate(&(account.data_len() as u64), data_length, include_output)
            }
            AccountInfoAssertion::Executable(executable, operator) => {
                operator.evaluate(&account.executable, executable, include_output)
            }
            AccountInfoAssertion::IsSigner(is_signer, operator) => {
                operator.evaluate(&account.is_signer, is_signer, include_output)
            }
            AccountInfoAssertion::IsWritable(is_writable, operator) => {
                operator.evaluate(&account.is_writable, is_writable, include_output)
            }
            AccountInfoAssertion::RentEpoch(rent_epoch, operator) => {
                operator.evaluate(&account.rent_epoch as &u64, rent_epoch, include_output)
            }
            AccountInfoAssertion::VerifyDatahash(expected_hash, start, end) => {
                let account_data = account.try_borrow_data()?;

                let start = start.unwrap_or(0);
                let end = end.unwrap_or(account_data.len() as u16);

                let account_data = &account_data[start as usize..end as usize];
                let actual_hash = keccak::hashv(&[&account_data]).0;

                EquatableOperator::Equal.evaluate(&actual_hash, expected_hash, include_output)
            }
        };

        Ok(result)
    }
}
