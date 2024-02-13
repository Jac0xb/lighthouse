use super::{clone_keypair, error::Error, program_test, Result};
use solana_program::{hash::Hash, pubkey::Pubkey};
use solana_program_test::{BanksClient, ProgramTestContext, ProgramTestError};
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

pub struct TestContext {
    program_context: ProgramTestContext,
    user_accounts: Vec<Keypair>,
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

    pub fn get_blockhash(&self) -> Hash {
        self.program_context.last_blockhash
    }

    pub async fn new() -> Result<Self> {
        let program_context = program_test().start_with_context().await;

        let mut ctx = TestContext {
            program_context,
            user_accounts: vec![],
        };

        let user_accounts = vec![
            Keypair::new(),
            Keypair::new(),
            Keypair::new(),
            Keypair::new(),
        ];

        for user in user_accounts.iter() {
            ctx.fund_account(user.pubkey(), DEFAULT_LAMPORTS_FUND_AMOUNT)
                .await?;
        }

        ctx.user_accounts = user_accounts;

        Ok(ctx)
    }

    pub fn client(&self) -> BanksClient {
        self.program_context.banks_client.clone()
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
