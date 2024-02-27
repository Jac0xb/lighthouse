use super::{clone_keypair, error::Error, program_test, Result};
use solana_program::{hash::Hash, pubkey::Pubkey};
use solana_program_test::{BanksClient, ProgramTestContext, ProgramTestError};
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

pub struct TestContext {
    pub program_context: ProgramTestContext,
}

pub const DEFAULT_LAMPORTS_FUND_AMOUNT: u64 = 1_000_000_000;

#[allow(deprecated)]
impl TestContext {
    pub fn test_context(&self) -> &ProgramTestContext {
        &self.program_context
    }

    pub fn mut_test_context(&mut self) -> &mut ProgramTestContext {
        &mut self.program_context
    }

    pub fn owned_test_context(self) -> ProgramTestContext {
        self.program_context
    }

    pub fn warp_to_slot(&mut self, slot: u64) -> std::result::Result<(), ProgramTestError> {
        self.program_context.warp_to_slot(slot)
    }

    pub async fn get_blockhash(&mut self) -> Hash {
        self.program_context
            .get_new_latest_blockhash()
            .await
            .unwrap()
    }

    pub async fn new() -> Result<Self> {
        let program_context = program_test().start_with_context().await;
        Ok(TestContext { program_context })
    }

    pub fn client(&self) -> BanksClient {
        self.program_context.banks_client.clone()
    }

    pub async fn get_account(&mut self, pubkey: Pubkey) -> Option<solana_sdk::account::Account> {
        self.program_context
            .banks_client
            .get_account(pubkey)
            .await
            .unwrap()
    }

    pub async fn fund_account(&mut self, address: Pubkey, lamports: u64) -> Result<()> {
        let payer = &self.program_context.payer;

        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &payer.pubkey(),
                &address,
                lamports,
            )],
            Some(&payer.pubkey()),
            &[payer],
            self.program_context.last_blockhash,
        );

        self.program_context
            .banks_client
            .process_transaction(tx)
            .await
            .map_err(|err| Box::new(Error::BanksClient(err)))
    }

    pub fn payer(&self) -> Keypair {
        clone_keypair(&self.program_context.payer)
    }
}
