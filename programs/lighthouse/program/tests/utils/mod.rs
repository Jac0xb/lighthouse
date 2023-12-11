pub mod context;
pub mod program;
pub mod tx_builder;
pub mod utils;

use anchor_lang::{self, InstructionData, ToAccountMetas};
use bytemuck::PodCastError;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::signature::{Keypair, SignerError};
use std::result;
pub use utils::{process_transaction_assert_failure, process_transaction_assert_success};

#[derive(Debug)]
pub enum Error {
    AccountNotFound(Pubkey),
    Anchor(anchor_lang::error::Error),
    BanksClient(BanksClientError),
    BytemuckPod(PodCastError),
    // The on-chain (via banks) and locally computed roots for a tree do not match.
    RootMismatch,
    Signer(SignerError),
}

pub type Result<T> = result::Result<T, Box<Error>>;
pub type BanksResult<T> = std::result::Result<T, BanksClientError>;

pub fn program_test() -> ProgramTest {
    let mut test = ProgramTest::new("lighthouse", lighthouse::id(), None);
    // test.add_program("spl_noop", spl_noop::id(), None);
    // test.add_program(
    //     "spl_account_compression",
    //     spl_account_compression::id(),
    //     None,
    // );
    // test.add_program("mpl_token_metadata", mpl_token_metadata::id(), None);
    test.set_compute_max_units(1_400_000);
    test
}

fn instruction<T, U>(accounts: &T, data: &U) -> Instruction
where
    T: ToAccountMetas,
    U: InstructionData,
{
    Instruction {
        program_id: lighthouse::id(),
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    }
}

// Helper method to copy keypairs for testing, since they don't implement
// `Copy/Clone` themselves (for some good reasons).
pub fn clone_keypair(k: &Keypair) -> Keypair {
    Keypair::from_bytes(k.to_bytes().as_slice()).unwrap()
}
