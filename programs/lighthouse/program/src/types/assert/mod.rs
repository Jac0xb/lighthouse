pub mod account_data;
pub mod account_data_delta;
pub mod account_info;
pub mod clock;
pub mod log_level;
pub mod merkle_tree;
pub mod mint_account;
pub mod stake_account;
pub mod token_account;
pub mod upgradable_loader_state;

pub use account_data::*;
pub use account_data_delta::*;
pub use account_info::*;
pub use clock::*;
pub use log_level::*;
pub use merkle_tree::*;
pub use mint_account::*;
pub use stake_account::*;
pub use token_account::*;
pub use upgradable_loader_state::*;

use super::operator::EvaluationResult;
use crate::utils::Result;

pub trait Assert<T> {
    fn evaluate(&self, parameters: &T, log_level: &LogLevel) -> Result<Box<EvaluationResult>>;
}
