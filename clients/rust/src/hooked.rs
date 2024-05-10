use crate::types::{
    AccountInfoAssertion, MintAccountAssertion, StakeAccountAssertion, TokenAccountAssertion,
    UpgradeableLoaderStateAssertion,
};

pub use lighthouse_common::CompactU64;
pub use lighthouse_common::LEB128Vec;

pub type AccountInfoAssertions = LEB128Vec<AccountInfoAssertion>;
pub type MintAccountAssertions = LEB128Vec<MintAccountAssertion>;
pub type TokenAccountAssertions = LEB128Vec<TokenAccountAssertion>;
pub type StakeAccountAssertions = LEB128Vec<StakeAccountAssertion>;
pub type UpgradeableLoaderStateAssertions = LEB128Vec<UpgradeableLoaderStateAssertion>;
pub type CompactBytes = LEB128Vec<u8>;
