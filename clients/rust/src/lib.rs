#[allow(unused)]
mod generated;

pub use generated::programs::LIGHTHOUSE_ID;
pub use generated::programs::LIGHTHOUSE_ID as ID;

pub mod types {
    pub use crate::generated::types::*;
}

pub mod instructions {

    pub use crate::generated::instructions::{
        AssertAccountDataBuilder, AssertAccountDeltaBuilder, AssertAccountInfoBuilder,
        AssertMerkleTreeAccountBuilder, AssertMintAccountMultiBuilder, AssertStakeAccountBuilder,
        AssertStakeAccountMultiBuilder, AssertSysvarClockBuilder, AssertTokenAccountBuilder,
        AssertTokenAccountMultiBuilder, AssertUpgradeableLoaderAccountBuilder,
        AssertUpgradeableLoaderAccountMultiBuilder, MemoryWriteBuilder,
    };
}

pub mod errors {
    pub use crate::generated::errors::*;
}
