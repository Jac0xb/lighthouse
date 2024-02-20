pub mod context;
pub mod error;
pub mod utils;

use anchor_spl::{associated_token, token::Mint};
use rust_sdk::{blackhat_program::BlackhatProgram, LighthouseProgram};
use solana_program::{pubkey::Pubkey, rent::Rent, system_instruction};
use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::{
    signature::Keypair,
    signer::{EncodableKeypair, Signer},
    transaction::Transaction,
};
use std::result;

use self::{
    context::{TestContext, DEFAULT_LAMPORTS_FUND_AMOUNT},
    utils::process_transaction_assert_success,
};

pub type Result<T> = result::Result<T, Box<error::Error>>;
pub type BanksResult<T> = std::result::Result<T, BanksClientError>;

pub fn program_test() -> ProgramTest {
    // program.add_program("<program_name>", <program_name>::id(), processor!(<program_name>::entry));

    let mut test = ProgramTest::new("lighthouse", lighthouse::id(), None);
    test.add_program("blackhat", blackhat::id(), None);
    test.set_compute_max_units(1_400_000);

    test
}

// Helper method to copy keypairs for testing, since they don't implement
// `Copy/Clone` themselves (for some good reasons).
pub fn clone_keypair(k: &Keypair) -> Keypair {
    Keypair::from_bytes(k.to_bytes().as_slice()).unwrap()
}

pub async fn create_memory_account(
    context: &mut TestContext,
    user: &Keypair,
    size: u64,
) -> Result<()> {
    let mut program = LighthouseProgram {};
    let mut tx_builder = program.create_memory_account(user.encodable_pubkey(), 0, size);
    let mut tx = tx_builder.to_transaction().unwrap();

    tx.try_partial_sign(&[user], context.get_blockhash())
        .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
    Ok(())
}

pub async fn create_user(ctx: &mut TestContext) -> Result<Keypair> {
    let user = Keypair::new();
    let _ = ctx
        .fund_account(user.pubkey(), DEFAULT_LAMPORTS_FUND_AMOUNT)
        .await;

    Ok(user)
}

pub struct CreateMintParameters {
    pub token_program: Pubkey,
    pub mint_authority: Option<Option<Pubkey>>,
    pub freeze_authority: Option<Pubkey>,
    pub decimals: u8,
    pub mint_to: Option<(Pubkey, u64)>,
}

pub async fn create_mint(
    ctx: &mut TestContext,
    payer: &Keypair,
    parameters: CreateMintParameters,
) -> Result<(Transaction, Keypair)> {
    let mint = Keypair::new();

    let mint_rent = Rent::default().minimum_balance(Mint::LEN);

    let mut ixs = Vec::new();

    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &parameters.token_program,
    );
    let mint_ix = spl_token::instruction::initialize_mint2(
        &parameters.token_program,
        &mint.pubkey(),
        &payer.pubkey(),
        parameters.freeze_authority.as_ref(),
        parameters.decimals,
    )
    .unwrap();

    ixs.push(create_ix);
    ixs.push(mint_ix);

    if let Some((dest, amount)) = parameters.mint_to {
        let token_account = associated_token::get_associated_token_address(&dest, &mint.pubkey());
        let create_account_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &payer.pubkey(),
                &dest,
                &mint.pubkey(),
                &spl_token::id(),
            );

        let mint_to_ix = spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint.pubkey(),
            &token_account,
            &payer.pubkey(),
            &[],
            amount,
        )
        .unwrap();

        ixs.push(create_account_ix);
        ixs.push(mint_to_ix);
    }

    if let Some(mint_authority) = parameters.mint_authority {
        let set_authority_ix = spl_token::instruction::set_authority(
            &parameters.token_program,
            &mint.pubkey(),
            mint_authority.as_ref(),
            spl_token::instruction::AuthorityType::MintTokens,
            &payer.pubkey(),
            &[],
        )
        .unwrap();
        ixs.push(set_authority_ix);
    }

    let mut tx = Transaction::new_with_payer(&ixs, Some(&payer.pubkey()));
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

pub async fn set_authority_mint(
    ctx: &mut TestContext,
    mint: &Pubkey,
    authority: &Keypair,
    new_authority: Option<Pubkey>,
    authority_type: spl_token::instruction::AuthorityType,
) -> Result<Transaction> {
    let ix = spl_token::instruction::set_authority(
        &spl_token::id(),
        mint,
        new_authority.as_ref(),
        authority_type,
        &authority.pubkey(),
        &[],
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&authority.pubkey()));

    let signers: &[Keypair; 1] = &[authority.insecure_clone()];

    tx.try_partial_sign(
        &signers.iter().collect::<Vec<_>>(),
        ctx.client().get_latest_blockhash().await.unwrap(),
    )
    .unwrap();

    Ok(tx)
}

pub async fn set_authority_token_account(
    ctx: &mut TestContext,
    token_account: &Pubkey,
    authority: &Keypair,
    new_authority: Option<Pubkey>,
    authority_type: spl_token::instruction::AuthorityType,
) -> Result<Transaction> {
    let ix = spl_token::instruction::set_authority(
        &spl_token::id(),
        token_account,
        new_authority.as_ref(),
        authority_type,
        &authority.pubkey(),
        &[],
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&authority.pubkey()));

    let signers: &[Keypair; 1] = &[authority.insecure_clone()];

    tx.try_partial_sign(
        &signers.iter().collect::<Vec<_>>(),
        ctx.client().get_latest_blockhash().await.unwrap(),
    )
    .unwrap();

    Ok(tx)
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

pub async fn create_test_account(
    ctx: &mut TestContext,
    payer: &Keypair,
    random: bool,
) -> Result<Keypair> {
    let account_keypair = Keypair::new();
    let program = BlackhatProgram {};

    let mut tx_builder =
        program.create_test_account(&payer.pubkey(), account_keypair.encodable_pubkey(), random);
    let mut tx = tx_builder.to_transaction().unwrap();

    tx.try_partial_sign(
        &[payer, &account_keypair],
        ctx.client().get_latest_blockhash().await.unwrap(),
    )
    .unwrap();

    process_transaction_assert_success(ctx, tx).await.unwrap();

    Ok(account_keypair)
}
