use solana_program::{hash::Hash, instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{signature::Keypair, transaction::Transaction};

use crate::Error;

pub struct TxBuilder {
    pub payer: Pubkey,
    pub ixs: Vec<Instruction>,
    pub look_up_tables: Vec<Pubkey>,
}

impl TxBuilder {
    pub fn to_transaction(&mut self) -> Result<Transaction, Error> {
        let tx = &mut Transaction::new_with_payer(&self.ixs, Some(&self.payer));

        Ok(tx.clone())
    }

    pub fn to_transaction_and_sign(
        &mut self,
        signers: Vec<&Keypair>,
        recent_blockhash: Hash,
    ) -> Result<Transaction, Error> {
        let tx = &mut Transaction::new_with_payer(&self.ixs, Some(&self.payer));
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
