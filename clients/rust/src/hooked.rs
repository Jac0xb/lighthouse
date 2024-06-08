use crate::types::DataValueAssertion;
use crate::types::{
    AccountInfoAssertion, MintAccountAssertion, StakeAccountAssertion, TokenAccountAssertion,
    UpgradeableLoaderStateAssertion,
};

use borsh::{BorshDeserialize, BorshSerialize};
pub use lighthouse_common::CompactU64;
pub use lighthouse_common::LEB128Vec;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountDataAssertion {
    pub offset: CompactU64,
    pub assertion: DataValueAssertion,
}

pub type AccountInfoAssertions = LEB128Vec<AccountInfoAssertion>;
pub type AccountDataAssertions = LEB128Vec<AccountDataAssertion>;
pub type MintAccountAssertions = LEB128Vec<MintAccountAssertion>;
pub type TokenAccountAssertions = LEB128Vec<TokenAccountAssertion>;
pub type StakeAccountAssertions = LEB128Vec<StakeAccountAssertion>;
pub type UpgradeableLoaderStateAssertions = LEB128Vec<UpgradeableLoaderStateAssertion>;
pub type CompactBytes = LEB128Vec<u8>;
