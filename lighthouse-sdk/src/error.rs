use solana_program::pubkey::Pubkey;
use solana_sdk::signer::SignerError;

#[derive(Debug)]
pub enum Error {
    AccountNotFound(Pubkey),
    Signer(SignerError),
}
