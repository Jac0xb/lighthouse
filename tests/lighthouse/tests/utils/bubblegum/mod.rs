pub mod context;
pub mod digital_asset;
pub mod tree;
pub mod tx_builder;

use anchor_lang::{InstructionData, ToAccountMetas};
use mpl_bubblegum::{
    state::metaplex_adapter::MetadataArgs,
    utils::{hash_creators, hash_metadata},
};
pub use tree::*;

use bytemuck::PodCastError;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::BanksClientError;
use solana_sdk::signature::{Keypair, SignerError};
use std::result;

#[derive(Debug)]
pub enum Error {
    AccountNotFound(Pubkey),
    Anchor(anchor_lang::error::Error),
    BanksClient(BanksClientError),
    BytemuckPod(PodCastError),
    // The on-chain (via banks) and locally computed roots for a tree do not match.
    RootMismatch,
    Signer(SignerError),
    TransactionFailed(String),
}

pub type Result<T> = result::Result<T, Box<Error>>;

fn instruction<T, U>(accounts: &T, data: &U) -> Instruction
where
    T: ToAccountMetas,
    U: InstructionData,
{
    Instruction {
        program_id: mpl_bubblegum::ID,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    }
}

// Helper method to copy keypairs for testing, since they don't implement
// `Copy/Clone` themselves (for some good reasons).
pub fn clone_keypair(k: &Keypair) -> Keypair {
    Keypair::from_bytes(k.to_bytes().as_slice()).unwrap()
}

// Computes the `data_hash` and `creator_hash`. Taken from the contract code where something
// similar is computed. Needs subsequent cleanup/refactoring.
pub fn compute_metadata_hashes(metadata_args: &MetadataArgs) -> Result<([u8; 32], [u8; 32])> {
    let data_hash = hash_metadata(metadata_args).map_err(Error::Anchor)?;
    let creator_hash = hash_creators(metadata_args.creators.as_slice()).map_err(Error::Anchor)?;
    Ok((data_hash, creator_hash))
}

#[derive(Debug)]
pub struct LeafArgs {
    pub owner: Keypair,
    pub delegate: Keypair,
    pub metadata: MetadataArgs,
    pub nonce: u64,
    pub index: u32,
}

impl Clone for LeafArgs {
    fn clone(&self) -> Self {
        LeafArgs {
            owner: clone_keypair(&self.owner),
            delegate: clone_keypair(&self.delegate),
            metadata: self.metadata.clone(),
            nonce: self.nonce,
            index: self.index,
        }
    }
}

impl LeafArgs {
    // Creates a new object with some default values.
    pub fn new(owner: &Keypair, metadata: MetadataArgs) -> Self {
        LeafArgs {
            owner: clone_keypair(owner),
            delegate: clone_keypair(owner),
            metadata,
            nonce: 0,
            index: 0,
        }
    }
}

pub trait DirtyClone {
    fn dirty_clone(&self) -> Self;
}

impl DirtyClone for Keypair {
    fn dirty_clone(&self) -> Self {
        Keypair::from_bytes(&self.to_bytes()).unwrap()
    }
}
