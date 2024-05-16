use crate::types::{
    AccountDataAssertion, AccountInfoAssertion, MintAccountAssertion, StakeAccountAssertion,
    TokenAccountAssertion, UpgradeableLoaderStateAssertion,
};

use lighthouse_common::integer_operator;

pub use lighthouse_common::CompactU64;
pub use lighthouse_common::LEB128Vec;

pub type AccountDataAssertions = LEB128Vec<AccountDataAssertion>;
pub type AccountInfoAssertions = LEB128Vec<AccountInfoAssertion>;
pub type MintAccountAssertions = LEB128Vec<MintAccountAssertion>;
pub type TokenAccountAssertions = LEB128Vec<TokenAccountAssertion>;
pub type StakeAccountAssertions = LEB128Vec<StakeAccountAssertion>;
pub type UpgradeableLoaderStateAssertions = LEB128Vec<UpgradeableLoaderStateAssertion>;
pub type CompactBytes = LEB128Vec<u8>;
pub type IntegerOperator = integer_operator::IntegerOperator;
