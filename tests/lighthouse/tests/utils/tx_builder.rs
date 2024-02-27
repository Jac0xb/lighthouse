use solana_program::{hash::Hash, instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{signature::Keypair, transaction::Transaction};

use super::error::Error;

pub struct TxBuilder {
    pub ixs: Vec<Instruction>,
    pub look_up_tables: Option<Vec<Pubkey>>,
}

impl TxBuilder {
    pub fn ix(&self) -> Instruction {
        if self.ixs.is_empty() {
            panic!("No instructions to build transaction");
        } else if self.ixs.len() > 1 {
            panic!("More than one instruction to build transaction");
        }

        self.ixs[0].clone()
    }

    pub fn to_transaction_and_sign(
        &mut self,
        signers: Vec<&Keypair>,
        payer: Pubkey,
        recent_blockhash: Hash,
    ) -> Result<Transaction, Error> {
        let tx = &mut Transaction::new_with_payer(&self.ixs, Some(&payer));
        tx.partial_sign(&signers, recent_blockhash);

        Ok(tx.clone())
    }

    pub fn prepend(&mut self, mut tx_builder: TxBuilder) -> &mut Self {
        tx_builder.ixs.append(&mut self.ixs);

        self.ixs = tx_builder.ixs;

        self
    }

    pub fn append(&mut self, mut tx_builder: TxBuilder) -> &mut Self {
        self.ixs.append(&mut tx_builder.ixs);

        self
    }
}
