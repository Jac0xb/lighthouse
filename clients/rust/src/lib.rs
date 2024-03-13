#[allow(unused)]
#[allow(clippy::identity_op)]
mod generated;

pub use generated::programs::LIGHTHOUSE_ID;
pub use generated::programs::LIGHTHOUSE_ID as ID;
use solana_program::pubkey::{Pubkey, PubkeyError};

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

pub fn find_memory_pda(payer: Pubkey, memory_id: u8) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &["memory".to_string().as_ref(), payer.as_ref(), &[memory_id]],
        &crate::ID,
    )
}

pub fn find_memory_pda_bump_iterate(
    payer: Pubkey,
    memory_id: u8,
    bump_skip: u8,
    start_bump: Option<u8>,
) -> Option<(solana_program::pubkey::Pubkey, u8)> {
    let memory_ref = "memory".to_string();
    let seeds = [memory_ref.as_ref(), payer.as_ref(), &[memory_id]];

    let mut bump_seed = [start_bump.unwrap_or(std::u8::MAX)];
    let mut bump_skip = bump_skip as usize;

    for _ in 0..std::u8::MAX {
        let mut seeds_with_bump = seeds.to_vec();
        seeds_with_bump.push(&bump_seed);
        match Pubkey::create_program_address(&seeds_with_bump, &crate::ID) {
            Ok(address) => {
                if bump_skip == 0 {
                    return Some((address, bump_seed[0]));
                } else {
                    bump_skip -= 1;
                }
            }
            Err(PubkeyError::InvalidSeeds) => {}
            _ => break,
        }
        bump_seed[0] -= 1;

        println!("bump_seed: {:?}", bump_seed[0])
    }

    None
}
