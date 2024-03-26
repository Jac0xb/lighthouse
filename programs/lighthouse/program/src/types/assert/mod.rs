pub mod account_data;
pub mod account_delta;
pub mod account_info;
pub mod bubblegum_tree_config;
pub mod clock;
pub mod evaluate;
pub mod known_program;
pub mod log_level;
pub mod merkle_tree;
pub mod mint_account;
pub mod stake_account;
pub mod token_account;
pub mod upgradable_loader_state;

pub use account_data::*;
pub use account_delta::*;
pub use account_info::*;
pub use bubblegum_tree_config::*;
pub use clock::*;
pub use evaluate::*;
pub use known_program::*;
pub use log_level::*;
pub use merkle_tree::*;
pub use mint_account::*;
pub use stake_account::*;
pub use token_account::*;
pub use upgradable_loader_state::*;

use crate::utils::Result;

pub trait Assert<T> {
    fn evaluate(&self, parameters: T, log_level: LogLevel) -> Result<()>;
}
