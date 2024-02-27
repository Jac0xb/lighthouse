use anchor_lang::*;
use lighthouse_sdk::TxBuilder;
use solana_program::{instruction::Instruction, pubkey::Pubkey, system_program, sysvar};

pub use test_program::id;

pub struct TestProgram {}

impl TestProgram {
    fn tx_builder(&self, ixs: Vec<Instruction>) -> TxBuilder {
        TxBuilder {
            ixs,
            look_up_tables: None,
        }
    }

    pub fn create_test_account(&self, signer: Pubkey, account: Pubkey, random: bool) -> TxBuilder {
        let accounts = test_program::accounts::CreateTestAccountV1 {
            system_program: system_program::id(),
            signer,
            test_account: account,
            rent: sysvar::rent::id(),
            slot_hashes: sysvar::slot_hashes::id(),
        };

        let data = test_program::instruction::CreateTestAccountV1 { random };

        self.tx_builder(vec![Instruction {
            program_id: test_program::id(),
            accounts: accounts.to_account_metas(None),
            data: data.data(),
        }])
    }
}
