use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::BanksClient;
use solana_sdk::{
    signature::{Keypair, Signature},
    transaction::Transaction,
};

use super::{Error, Result};

pub struct TxBuilder {
    pub payer: Pubkey,
    pub client: BanksClient,
    pub signers: Vec<Keypair>,
    pub ixs: Vec<Instruction>,
}

pub trait OnSuccessfulTxExec {
    fn on_successful_execute(&mut self) -> Result<()>;
}

impl<'a> TxBuilder {
    pub async fn execute(&mut self) -> Result<Signature> {
        let recent_blockhash = self
            .client
            .get_latest_blockhash()
            .await
            .map_err(Error::BanksClient)?;

        let tx = &mut Transaction::new_with_payer(&self.ixs, Some(&self.payer));

        // Using `try_partial_sign` to avoid panics (and get an error when something is
        // wrong instead) no matter what signers are configured.
        tx.try_partial_sign(&self.signers.iter().collect::<Vec<_>>(), recent_blockhash)
            .map_err(Error::Signer)?;

        self.client
            .process_transaction(tx.clone())
            .await
            .map_err(Error::BanksClient)?;

        let signature = tx.signatures[0];

        Ok(signature)
    }

    pub async fn to_transaction(&mut self) -> Result<Transaction> {
        let recent_blockhash = self
            .client
            .get_latest_blockhash()
            .await
            .map_err(Error::BanksClient)?;

        let tx = &mut Transaction::new_with_payer(&self.ixs, Some(&self.payer));

        // Using `try_partial_sign` to avoid panics (and get an error when something is
        // wrong instead) no matter what signers are configured.
        tx.try_partial_sign(&self.signers.iter().collect::<Vec<_>>(), recent_blockhash)
            .map_err(Error::Signer)?;

        Ok(tx.clone())
    }

    pub fn prepend(&mut self, mut tx_builder: TxBuilder) -> &mut Self {
        tx_builder.ixs.append(&mut self.ixs);
        tx_builder.signers.append(&mut self.signers);

        self.ixs = tx_builder.ixs;
        self.signers = tx_builder.signers;

        self
    }

    pub fn append(&mut self, mut tx_builder: TxBuilder) -> &mut Self {
        self.ixs.append(&mut tx_builder.ixs);
        self.signers.append(&mut tx_builder.signers);

        self
    }
}
