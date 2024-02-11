use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClientError;
use solana_sdk::signer::SignerError;

#[derive(Debug)]
pub enum Error {
    AccountNotFound(Pubkey),
    Anchor(anchor_lang::error::Error),
    BanksClient(BanksClientError),
    // BytemuckPod(PodCastError),
    // The on-chain (via banks) and locally computed roots for a tree do not match.
    RootMismatch,
    Signer(SignerError),
    TransactionFailed,
}
