use crate::{find_cache_account, find_test_account};

use super::{
    clone_keypair,
    tx_builder::{
        AssertBuilder, CacheLoadAccountV1Builder, CreateCacheAccountBuilder,
        CreateTestAccountV1Builder, TxBuilder,
    },
    Error, Result,
};
use anchor_lang::*;
use lighthouse::structs::{Assertion, Expression};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program, sysvar,
};
use solana_program_test::BanksClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    signer::signers::Signers,
    transaction::Transaction,
};

// A convenience object that records some of the parameters for compressed
// trees and generates TX builders with the default configuration for each
// operation.
pub struct Program {
    client: BanksClient,
}

impl Program {
    // This and `with_creator` use a bunch of defaults; things can be
    // customized some more via the public access, or we can add extra
    // methods to make things even easier.
    pub fn new(client: BanksClient) -> Self {
        Self::with_creator(&Keypair::new(), client)
    }

    pub fn with_creator(tree_creator: &Keypair, client: BanksClient) -> Self {
        Program { client }
    }

    // Helper method to execute a transaction with the specified arguments
    // (i.e. single instruction) via the inner Banks client.
    pub async fn process_tx<T: Signers>(
        &mut self,
        instruction: Instruction,
        payer: &Pubkey,
        signing_keypairs: &T,
    ) -> Result<()> {
        let recent_blockhash = self
            .client
            .get_latest_blockhash()
            .await
            .map_err(Error::BanksClient)?;

        self.client
            .process_transaction(Transaction::new_signed_with_payer(
                &[instruction],
                Some(payer),
                signing_keypairs,
                recent_blockhash,
            ))
            .await
            .map_err(|err| Box::new(Error::BanksClient(err)))
    }

    pub async fn rent(&mut self) -> Result<Rent> {
        self.client
            .get_rent()
            .await
            .map_err(|err| Box::new(Error::BanksClient(err)))
    }

    // Helper fn to instantiate the various `TxBuilder` based concrete types
    // associated with each operation.
    fn tx_builder<T, U, V>(
        &mut self,
        accounts: T,
        data: U,
        inner: V,
        payer: Pubkey,
        default_signers: &[&Keypair],
        additional_accounts: Vec<AccountMeta>,
    ) -> TxBuilder<T, U, V> {
        let def_signers = default_signers.iter().map(|k| clone_keypair(k)).collect();

        TxBuilder {
            accounts,
            additional_accounts,
            data,
            payer,
            client: self.client.clone(),
            signers: def_signers,
            inner,
        }
    }

    pub fn create_assertion(
        &mut self,
        payer: &Keypair,
        assertions: Vec<Assertion>,
        additional_accounts: Vec<Pubkey>,
        logical_expressions: Option<Vec<Expression>>,
    ) -> AssertBuilder {
        let accounts = lighthouse::accounts::AssertV1 {
            system_program: system_program::id(),
        };

        // The conversions below should not fail.
        let data = lighthouse::instruction::AssertV1 {
            assertions,
            logical_expression: logical_expressions,
            // options: None,
        };

        self.tx_builder(
            accounts,
            data,
            (),
            payer.pubkey(),
            &[payer],
            additional_accounts
                .into_iter()
                .map(|pubkey| AccountMeta::new_readonly(pubkey, false))
                .collect(),
        )
    }

    pub fn create_cache_account(
        &mut self,
        payer: &Keypair,
        cache_index: u8,
        cache_account_size: u64,
    ) -> CreateCacheAccountBuilder {
        let accounts = lighthouse::accounts::CreateCacheAccountV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            cache_account: find_cache_account(payer.pubkey(), cache_index).0,
            rent: sysvar::rent::id(),
        };

        // The conversions below should not fail.
        let data = lighthouse::instruction::CreateCacheAccountV1 {
            cache_index,
            cache_account_size,
        };

        self.tx_builder(accounts, data, (), payer.pubkey(), &[payer], vec![])
    }

    pub fn load_cache_account(
        &mut self,
        payer: &Keypair,
        source_account: Pubkey,
        cache_index: u8,
        cache_start: u16,
        dest_start: u16,
        slice_length: u16,
    ) -> CacheLoadAccountV1Builder {
        let accounts = lighthouse::accounts::CacheLoadAccountV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            cache_account: find_cache_account(payer.pubkey(), cache_index).0,
            rent: sysvar::rent::id(),
            source_account,
        };

        // The conversions below should not fail.
        let data = lighthouse::instruction::CacheLoadAccountV1 {
            cache_index,
            cache_start,
            dest_start,
            slice_length,
        };

        self.tx_builder(accounts, data, (), payer.pubkey(), &[payer], vec![])
    }

    pub fn create_test_account(&mut self, payer: &Keypair) -> CreateTestAccountV1Builder {
        let accounts = lighthouse::accounts::CreateTestAccountV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            test_account: find_test_account().0,
            rent: sysvar::rent::id(),
        };

        // The conversions below should not fail.
        let data = lighthouse::instruction::CreateTestAccountV1 {};

        self.tx_builder(accounts, data, (), payer.pubkey(), &[payer], vec![])
    }
}
