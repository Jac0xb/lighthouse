#[allow(unused)]
#[allow(clippy::identity_op)]
mod generated;

pub use generated::programs::LIGHTHOUSE_ID;
pub use generated::programs::LIGHTHOUSE_ID as ID;
use solana_program::pubkey::Pubkey;

pub mod types {
    pub use crate::generated::types::*;
}

pub mod instructions {
    pub use crate::generated::instructions::{
        AssertAccountDataBuilder, AssertAccountDeltaBuilder, AssertAccountInfoBuilder,
        AssertMerkleTreeAccountBuilder, AssertMintAccountMultiBuilder, AssertStakeAccountBuilder,
        AssertStakeAccountMultiBuilder, AssertSysvarClockBuilder, AssertTokenAccountBuilder,
        AssertTokenAccountMultiBuilder, AssertUpgradeableLoaderAccountBuilder,
        AssertUpgradeableLoaderAccountMultiBuilder, MemoryCloseBuilder, MemoryWriteBuilder,
    };
}

pub mod errors {
    pub use crate::generated::errors::*;
}

pub fn get_memory_account(user: Pubkey, memory_index: u8) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[
            "memory".to_string().as_ref(),
            user.as_ref(),
            &[memory_index],
        ],
        &crate::ID,
    )
}
