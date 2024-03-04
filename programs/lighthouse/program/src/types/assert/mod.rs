pub mod account_data;
pub mod account_data_delta;
pub mod account_info;
pub mod clock;
pub mod log_level;
pub mod mint_account;
pub mod stake_account;
pub mod token_account;
mod upgradable_loader_state;

pub use account_data::*;
pub use account_data_delta::*;
pub use account_info::*;
pub use clock::*;
pub use log_level::*;
pub use mint_account::*;
pub use stake_account::*;
pub use token_account::*;
pub use upgradable_loader_state::*;

use crate::{types::EvaluationResult, utils::Result};

pub trait Assert<T: core::fmt::Debug> {
    fn evaluate(&self, parameters: &T, log_level: &LogLevel) -> Result<Box<EvaluationResult>>;
}
