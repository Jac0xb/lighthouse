use super::{
    clone_keypair,
    context::{TestContext, DEFAULT_LAMPORTS_FUND_AMOUNT},
    process_transaction_assert_success,
    tx_builder::{
        AssertBuilder, CacheLoadAccountV1Builder, CreateCacheAccountBuilder,
        CreateTestAccountV1Builder, TxBuilder,
    },
    Error, Result,
};
use anchor_lang::*;
use lighthouse::{
    processor::Config,
    structs::{Assertion, Expression, WriteTypeParameter},
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::Rent,
    system_program, sysvar,
};
use solana_program_test::BanksClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    signer::signers::Signers,
    transaction::Transaction,
};

pub struct Program {
    client: BanksClient,
}

impl Program {
    pub fn new(client: BanksClient) -> Self {
        Program { client }
    }

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

    #[allow(clippy::too_many_arguments)]
    fn tx_builder<T, U, V>(
        &mut self,
        accounts: T,
        data: U,
        inner: V,
        ixs: Vec<Instruction>,
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
            ixs,
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
        logical_expression: Option<Vec<Expression>>,
        cache: Option<Pubkey>,
    ) -> AssertBuilder {
        let accounts = lighthouse::accounts::AssertV1 { cache };

        let assertion_clone = (assertions).clone();
        let logical_expression_clone = (logical_expression).clone();

        // The conversions below should not fail.
        let data = lighthouse::instruction::AssertV1 {
            assertions,
            logical_expression,
            options: Some(Config { verbose: true }),
        };

        self.tx_builder(
            accounts,
            data,
            (),
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: (lighthouse::accounts::AssertV1 { cache }).to_account_metas(None),
                data: (lighthouse::instruction::AssertV1 {
                    assertions: assertion_clone,
                    logical_expression: logical_expression_clone,
                    options: Some(Config { verbose: true }),
                })
                .data(),
            }],
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

        let data = lighthouse::instruction::CreateCacheAccountV1 {
            cache_index,
            cache_account_size,
        };

        self.tx_builder(
            accounts,
            data,
            (),
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: (lighthouse::accounts::CreateCacheAccountV1 {
                    system_program: system_program::id(),
                    signer: payer.pubkey(),
                    cache_account: find_cache_account(payer.pubkey(), cache_index).0,
                    rent: sysvar::rent::id(),
                })
                .to_account_metas(None),
                data: (lighthouse::instruction::CreateCacheAccountV1 {
                    cache_index,
                    cache_account_size,
                })
                .data(),
            }],
            payer.pubkey(),
            &[payer],
            vec![],
        )
    }

    pub fn write_v1(
        &mut self,
        payer: &Keypair,
        source_account: Pubkey,
        cache_index: u8,
        write_type_parameter: WriteTypeParameter,
    ) -> CacheLoadAccountV1Builder {
        let accounts = lighthouse::accounts::WriteV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            cache_account: find_cache_account(payer.pubkey(), cache_index).0,
        };

        let write_type_clone = write_type_parameter.clone();

        let data = lighthouse::instruction::WriteV1 {
            write_type: write_type_parameter,
            cache_index,
        };

        let mut ix_accounts = lighthouse::accounts::WriteV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            cache_account: find_cache_account(payer.pubkey(), cache_index).0,
        }
        .to_account_metas(None);
        ix_accounts.append(&mut vec![AccountMeta::new(source_account, false)]);

        self.tx_builder(
            accounts,
            data,
            (),
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: ix_accounts,
                data: (lighthouse::instruction::WriteV1 {
                    write_type: write_type_clone,
                    cache_index,
                })
                .data(),
            }],
            payer.pubkey(),
            &[payer],
            vec![AccountMeta::new(source_account, false)],
        )
    }

    pub fn create_test_account(&mut self, payer: &Keypair) -> CreateTestAccountV1Builder {
        let accounts = lighthouse::accounts::CreateTestAccountV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            test_account: find_test_account().0,
            rent: sysvar::rent::id(),
        };

        let data = lighthouse::instruction::CreateTestAccountV1 {};

        self.tx_builder(accounts, data, (), vec![], payer.pubkey(), &[payer], vec![])
    }
}

pub async fn create_test_account(context: &mut TestContext, payer: &Keypair) -> Result<()> {
    let mut program = Program::new(context.client());
    let mut tx_builder = program.create_test_account(payer);
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;
    Ok(())
}

pub async fn create_cache_account(
    context: &mut TestContext,
    user: &Keypair,
    size: u64,
) -> Result<()> {
    let mut program = Program::new(context.client());
    let mut tx_builder = program.create_cache_account(user, 0, size);
    process_transaction_assert_success(context, tx_builder.to_transaction(vec![]).await).await;
    Ok(())
}

pub fn find_test_account() -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &["test_account".to_string().as_ref()],
        &lighthouse::ID,
    )
}

pub fn find_cache_account(user: Pubkey, cache_index: u8) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &["cache".to_string().as_ref(), user.as_ref(), &[cache_index]],
        &lighthouse::ID,
    )
}

pub async fn create_user(ctx: &mut TestContext) -> Result<Keypair> {
    let user = Keypair::new();
    let _ = ctx
        .fund_account(user.pubkey(), DEFAULT_LAMPORTS_FUND_AMOUNT)
        .await;

    Ok(user)
}
