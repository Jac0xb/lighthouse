use super::{
    clone_keypair,
    context::{TestContext, DEFAULT_LAMPORTS_FUND_AMOUNT},
    process_transaction_assert_success,
    tx_builder::TxBuilder,
    Error, Result,
};
use anchor_lang::*;
use anchor_spl::associated_token::{self, get_associated_token_address};
use blackhat;
use lighthouse::structs::{Assertion, AssertionConfig, WriteTypeParameter};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
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
use spl_token::state::Mint;

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
    fn tx_builder(
        &mut self,
        ixs: Vec<Instruction>,
        payer: Pubkey,
        signers: &[&Keypair],
    ) -> TxBuilder {
        TxBuilder {
            payer,
            ixs,
            client: self.client.clone(),
            signers: signers.iter().map(|k| clone_keypair(k)).collect(),
        }
    }

    pub fn create_assert_compact(
        &mut self,
        payer: &Keypair,
        target_account: Pubkey,
        assertion: Assertion,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: (lighthouse::accounts::AssertCompactV1 { target_account })
                    .to_account_metas(None),
                data: lighthouse::instruction::AssertCompactV1 { assertion }.data(),
            }],
            payer.pubkey(),
            &[payer],
        )
    }

    pub fn create_assertion(
        &mut self,
        payer: &Keypair,
        assertions: Vec<Assertion>,
        additional_accounts: Vec<Pubkey>,
    ) -> TxBuilder {
        let mut accounts = (lighthouse::accounts::AssertMultiV1 {
            system_program: system_program::id(),
        })
        .to_account_metas(None);

        // append additional_accounts to accounts
        accounts.append(
            &mut additional_accounts
                .into_iter()
                .map(|pubkey| AccountMeta::new_readonly(pubkey, false))
                .collect(),
        );

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts,
                data: (lighthouse::instruction::AssertMultiV1 {
                    assertions,
                    config: Some(AssertionConfig { verbose: true }),
                })
                .data(),
            }],
            payer.pubkey(),
            &[payer],
        )
    }

    pub fn create_cache_account(
        &mut self,
        payer: &Keypair,
        cache_index: u8,
        cache_account_size: u64,
    ) -> TxBuilder {
        self.tx_builder(
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
        )
    }

    pub fn write_v1(
        &mut self,
        payer: &Keypair,
        source_account: Pubkey,
        cache_index: u8,
        write_type_parameter: WriteTypeParameter,
    ) -> TxBuilder {
        let write_type_clone = write_type_parameter.clone();
        let mut ix_accounts = lighthouse::accounts::WriteV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            cache_account: find_cache_account(payer.pubkey(), cache_index).0,
        }
        .to_account_metas(None);
        ix_accounts.append(&mut vec![AccountMeta::new(source_account, false)]);

        self.tx_builder(
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
        )
    }

    pub fn create_test_account(&mut self, payer: &Keypair) -> TxBuilder {
        let accounts = lighthouse::accounts::CreateTestAccountV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            test_account: find_test_account().0,
            rent: sysvar::rent::id(),
        };

        let data = lighthouse::instruction::CreateTestAccountV1 {};

        self.tx_builder(
            vec![
                Instruction {
                    program_id: lighthouse::id(),
                    accounts: accounts.to_account_metas(None),
                    data: data.data(),
                },
                system_instruction::transfer(
                    &payer.pubkey(),
                    &find_test_account().0,
                    DEFAULT_LAMPORTS_FUND_AMOUNT,
                ),
            ],
            payer.pubkey(),
            &[payer],
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
            &[victim],
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
            &[victim],
        )
    }
}

pub async fn create_test_account(context: &mut TestContext, payer: &Keypair) -> Result<()> {
    let mut program = Program::new(context.client());
    let mut tx_builder = program.create_test_account(payer);
    process_transaction_assert_success(context, tx_builder.to_transaction().await).await;
    Ok(())
}

pub async fn create_cache_account(
    context: &mut TestContext,
    user: &Keypair,
    size: u64,
) -> Result<()> {
    let mut program = Program::new(context.client());
    let mut tx_builder = program.create_cache_account(user, 0, size);
    process_transaction_assert_success(context, tx_builder.to_transaction().await).await;
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

pub async fn create_mint(ctx: &mut TestContext, payer: &Keypair) -> Result<(Transaction, Keypair)> {
    let mint = Keypair::new();

    let mint_rent = Rent::default().minimum_balance(Mint::get_packed_len());
    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        Mint::get_packed_len() as u64,
        &spl_token::id(),
    );

    let mint_ix = spl_token::instruction::initialize_mint2(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        100,
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[create_ix, mint_ix], Some(&payer.pubkey()));

    let signers: &[Keypair; 2] = &[payer.insecure_clone(), mint.insecure_clone()];

    // print all the accounts in tx and is_signer
    for (i, account) in tx.message().account_keys.iter().enumerate() {
        println!("account: {} {}", account, tx.message.is_signer(i));
    }

    // print the signers pubkey in array
    for signer in signers.iter() {
        let pos = tx.get_signing_keypair_positions(&[signer.pubkey()]);
        println!(
            "signer: {} {}",
            signer.insecure_clone().pubkey(),
            pos.unwrap()[0].unwrap_or(0)
        );
    }

    tx.try_partial_sign(
        &signers.iter().collect::<Vec<_>>(),
        ctx.client().get_latest_blockhash().await.unwrap(),
    )
    .unwrap();

    Ok((tx, mint))
}

pub async fn mint_to(
    ctx: &mut TestContext,
    mint: &Pubkey,
    authority: &Keypair,
    dest: &Pubkey,
    amount: u64,
) -> Result<Transaction> {
    let token_account = associated_token::get_associated_token_address(dest, mint);
    let create_account_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            &authority.pubkey(),
            dest,
            mint,
            &spl_token::id(),
        );

    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        mint,
        &token_account,
        &authority.pubkey(),
        &[],
        amount,
    )
    .unwrap();

    let mut tx =
        Transaction::new_with_payer(&[create_account_ix, mint_to_ix], Some(&authority.pubkey()));

    let signers: &[Keypair; 1] = &[authority.insecure_clone()];

    tx.try_partial_sign(
        &signers.iter().collect::<Vec<_>>(),
        ctx.client().get_latest_blockhash().await.unwrap(),
    )
    .unwrap();

    Ok(tx)
}
