// use super::{clone_keypair, instruction, Error, Result};
use anchor_lang::{self, InstructionData, ToAccountMetas};
use solana_sdk::{
    instruction::{self, AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
};
// use solana_program::{instruction::Instruction, pubkey::Pubkey};

pub struct TxBuilder<T, U, V> {
    // pub accounts: T,
    pub additional_accounts: Vec<AccountMeta>,
    // pub data: U,
    pub payer: Pubkey,
    pub signers: Vec<Keypair>,
    pub inner: V,
    pub ixs: Vec<Instruction>,
}

pub trait OnSuccessfulTxExec {
    fn on_successful_execute(&mut self) -> Result<()>;
}

impl<'a, T, U, V> TxBuilder<T, U, V>
where
    T: ToAccountMetas,
    U: InstructionData,
{
    pub async fn execute(&mut self) -> Result<Signature>
    where
        Self: OnSuccessfulTxExec,
    {
        // let recent_blockhash = self
        //     .client
        //     .get_latest_blockhash()
        //     .await
        //     .map_err(Error::BanksClient)?;

        // let mut ix = instruction(&self.accounts, &self.data);

        // if self.additional_accounts.is_empty() {
        //     // We're only automatically adding the proof if there are no additional
        //     // accounts explicitly configured.
        //     if let Some(leaf_idx) = self.need_proof {
        //         ix.accounts
        //             .append(&mut self.tree.proof_of_leaf_metas(leaf_idx))
        //     }
        // } else {
        //     // Add the additional accounts metas (if any).
        // sefl.instruction.accounts.append(&mut self.additional_accounts.clone());
        // }

        let ixs = vec![ix];
        let ixs = ixs
            .into_iter()
            .chain(additional_ix.into_iter())
            .collect::<Vec<_>>();

        let tx = &mut Transaction::new_with_payer(&ixs, Some(&self.payer));

        // Using `try_partial_sign` to avoid panics (and get an error when something is
        // wrong instead) no matter what signers are configured.
        tx.try_partial_sign(&self.signers.iter().collect::<Vec<_>>(), recent_blockhash)
            .map_err(Error::Signer)?;

        self.client
            .process_transaction(tx.clone())
            .await
            .map_err(Error::BanksClient)?;

        self.on_successful_execute()?;

        // Check the expected tree root matches on-chain state post tx.
        // self.tree.check_expected_root().await

        let signature = tx.signatures[0];

        Ok(signature)
    }

    // pub fn to_instructions(&mut self) -> Vec<Instruction>
    // where
    //     Self: OnSuccessfulTxExec,
    // {
    //     return self.ixs;
    // }

    pub async fn to_transaction(
        &mut self,
        blockhash: String,
        additional_ix: Vec<Instruction>,
    ) -> Result<Transaction>
    where
        Self: OnSuccessfulTxExec,
    {
        // let mut ix = instruction(&self.accounts, &self.data);
        ix.accounts.append(&mut self.additional_accounts.clone());

        let ixs = &mut additional_ix
            .into_iter()
            .chain(vec![ix])
            .collect::<Vec<_>>();

        let tx = &mut Transaction::new_with_payer(ixs, Some(&self.payer));

        // Using `try_partial_sign` to avoid panics (and get an error when something is
        // wrong instead) no matter what signers are configured.
        tx.try_partial_sign(&self.signers.iter().collect::<Vec<_>>(), recent_blockhash)
            .map_err(Error::Signer)?;

        Ok(tx.clone())
    }

    // Returning `&mut Self` to allow method chaining.
    pub fn set_signers(&mut self, signers: &[&Keypair]) -> &mut Self {
        self.signers = signers.iter().map(|k| clone_keypair(k)).collect();
        self
    }

    pub fn set_payer(&mut self, key: Pubkey) -> &mut Self {
        self.payer = key;
        self
    }

    pub fn set_additional_account_metas(&mut self, metas: &[AccountMeta]) -> &mut Self {
        self.additional_accounts = metas.to_vec();
        self
    }

    // Populate the `additional_account` member with read-only and non-signer accounts based
    // on the provided public keys.
    pub fn set_additional_accounts(&mut self, keys: &[Pubkey]) -> &mut Self {
        self.additional_accounts = keys
            .iter()
            .map(|key| AccountMeta::new_readonly(*key, false))
            .collect();
        self
    }

    pub fn set_additional_accounts_from_arrays(&mut self, keys: &[[u8; 32]]) -> &mut Self {
        self.set_additional_accounts(
            keys.iter()
                .copied()
                .map(Pubkey::new_from_array)
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}

// The types below have "builder" in their names because we're essentially
// implementing a lightweight builder patter to instantiate, customize, and
// execute transactions.
pub type AssertBuilder<'a> =
    TxBuilder<lighthouse::accounts::AssertV1, lighthouse::instruction::AssertV1, ()>;

impl<'a> OnSuccessfulTxExec for AssertBuilder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}

pub type CreateCacheAccountBuilder<'a> = TxBuilder<
    lighthouse::accounts::CreateCacheAccountV1,
    lighthouse::instruction::CreateCacheAccountV1,
    (),
>;

impl<'a> OnSuccessfulTxExec for CreateCacheAccountBuilder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}

pub type CacheLoadAccountV1Builder<'a> =
    TxBuilder<lighthouse::accounts::WriteV1, lighthouse::instruction::WriteV1, ()>;

impl<'a> OnSuccessfulTxExec for CacheLoadAccountV1Builder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}

pub type CreateTestAccountV1Builder<'a> = TxBuilder<
    lighthouse::accounts::CreateTestAccountV1,
    lighthouse::instruction::CreateTestAccountV1,
    (),
>;

impl<'a> OnSuccessfulTxExec for CreateTestAccountV1Builder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}
