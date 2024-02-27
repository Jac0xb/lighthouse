use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClientError;
use solana_sdk::signer::SignerError;

#[derive(Debug)]
pub enum Error {
    AccountNotFound(Pubkey),
    Anchor(anchor_lang::error::Error),
    BanksClient(BanksClientError),
    Signer(SignerError),
    TransactionFailed(String),
    TransactionExpectedFailure(String),
    UnexpectedErrorCode,
    LogNotFound(String),
}
