pub mod blackhat_program;
pub mod bubblegum;
pub mod context;
pub mod error;
pub mod test_program;
pub mod tx_builder;

use anchor_spl::{associated_token, token::Mint};
use lighthouse_client::errors::LighthouseError;
use solana_banks_interface::BanksTransactionResultWithMetadata;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::{BanksClient, BanksClientError, ProgramTest};
use solana_sdk::{
    account::AccountSharedData,
    instruction::{Instruction, InstructionError},
    pubkey::Pubkey,
    rent::Rent,
    signature::Keypair,
    signer::{EncodableKeypair, Signer},
    system_instruction,
    transaction::{Transaction, TransactionError},
};
use std::result;

use self::{
    context::{TestContext, DEFAULT_LAMPORTS_FUND_AMOUNT},
    error::Error,
    test_program::TestProgram,
};

pub type Result<T> = result::Result<T, Box<error::Error>>;
pub type BanksResult<T> = std::result::Result<T, BanksClientError>;

pub fn program_test() -> ProgramTest {
    let mut test = ProgramTest::new("lighthouse", lighthouse_client::ID, None);
    test.add_program("blackhat", blackhat::id(), None);
    test.add_program("test_program", test_program::id(), None);
    test.add_program(
        "spl_account_compression",
        spl_account_compression::id(),
        None,
    );
    test.add_program("spl_noop", spl_noop::id(), None);
    test.add_program("mpl_token_metadata", mpl_token_metadata::ID, None);
    test.add_program("mpl_bubblegum", mpl_bubblegum::ID, None);

    test.set_compute_max_units(1_400_000);

    test.prefer_bpf(true);

    test
}

// Helper method to copy keypairs for testing, since they don't implement
// `Copy/Clone` themselves (for some good reasons).
pub fn clone_keypair(k: &Keypair) -> Keypair {
    Keypair::from_bytes(k.to_bytes().as_slice()).unwrap()
}

pub async fn create_user(ctx: &mut TestContext) -> Result<Keypair> {
    let user = Keypair::new();
    let _ = ctx
        .fund_account(user.pubkey(), DEFAULT_LAMPORTS_FUND_AMOUNT)
        .await;

    Ok(user)
}

pub async fn create_user_with_balance(ctx: &mut TestContext, balance: u64) -> Result<Keypair> {
    let user = Keypair::new();
    let _ = ctx.fund_account(user.pubkey(), balance).await;

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

pub async fn create_and_transfer_token_account_ix(
    ctx: &mut TestContext,
    sender: &Pubkey,
    mint: &Pubkey,
    dest: &Pubkey,
    amount: u64,
) -> Result<Vec<Instruction>> {
    let src_token_account = associated_token::get_associated_token_address(sender, mint);
    let dest_token_account = associated_token::get_associated_token_address(dest, mint);

    let mut ixs = Vec::new();

    if let Some(account) = ctx.get_account(dest_token_account).await {
        if account.lamports == 0 {
            ixs.push(
                spl_associated_token_account::instruction::create_associated_token_account(
                    sender,
                    dest,
                    mint,
                    &spl_token::id(),
                ),
            )
        }
    } else {
        ixs.push(
            spl_associated_token_account::instruction::create_associated_token_account(
                sender,
                dest,
                mint,
                &spl_token::id(),
            ),
        )
    }

    ixs.push(
        spl_token::instruction::transfer(
            &spl_token::id(),
            &src_token_account,
            &dest_token_account,
            sender,
            &[],
            amount,
        )
        .unwrap(),
    );

    Ok(ixs)
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
    let program = TestProgram {};

    let tx = program
        .create_test_account(
            payer.encodable_pubkey(),
            account_keypair.encodable_pubkey(),
            random,
        )
        .to_transaction_and_sign(
            vec![payer],
            payer.encodable_pubkey(),
            ctx.get_blockhash().await,
        )
        .unwrap();

    process_transaction_assert_success(ctx, tx).await.unwrap();

    Ok(account_keypair)
}

pub async fn process_transaction(
    client: &mut BanksClient,
    tx: &Transaction,
) -> Result<BanksTransactionResultWithMetadata> {
    let result: solana_banks_interface::BanksTransactionResultWithMetadata = client
        .process_transaction_with_metadata(tx.clone())
        .await
        .unwrap();

    Ok(result)
}

pub async fn process_transaction_assert_success(
    context: &mut TestContext,
    tx: Transaction,
) -> Result<BanksTransactionResultWithMetadata> {
    let tx_metadata = process_transaction(&mut context.client(), &tx).await;

    if let Err(err) = tx_metadata {
        panic!("Transaction failed to process: {:?}", err);
    }

    let tx_metadata = tx_metadata.unwrap();

    if let Some(logs) = tx_metadata.metadata.clone().map(|m| m.log_messages) {
        println!("Transaction Logs:");
        for log in logs {
            println!("{}", log);
        }
    }

    if tx_metadata.result.is_err() {
        return Err(Box::new(Error::TransactionFailed(format!(
            "Tx Result {:?}",
            tx_metadata.result.clone().err()
        ))));
    }

    Ok(tx_metadata)
}

pub async fn process_transaction_assert_failure(
    context: &mut TestContext,
    tx: Transaction,
    expected_tx_error: TransactionError,
    log_match_regex: Option<&[String]>,
) -> Result<()> {
    let tx_metadata = process_transaction(&mut context.client(), &tx)
        .await
        .unwrap();

    if tx_metadata.metadata.is_none() {
        println!("No metadata found in transaction");
        println!("{:?}", tx_metadata.result);

        return Err(Box::new(Error::TransactionFailed(
            "No metadata found in transaction".to_string(),
        )));
    }

    let logs = tx_metadata.metadata.clone().unwrap().log_messages;
    println!("Transaction Logs:");
    for log in logs {
        println!("{:?}", log);
    }

    if tx_metadata.result.is_ok() {
        return Err(Box::new(Error::TransactionExpectedFailure(
            "Transaction was expected to fail".to_string(),
        )));
    }

    let actual_tx_error = tx_metadata.result.unwrap_err();

    if actual_tx_error != expected_tx_error {
        match &actual_tx_error {
            TransactionError::InstructionError(ix_index, program_error) => {
                match &expected_tx_error {
                    TransactionError::InstructionError(
                        expected_ix_index,
                        expected_program_error,
                    ) => {
                        if ix_index != expected_ix_index || program_error != expected_program_error
                        {
                            return Err(Box::new(Error::TransactionExpectedFailure(format!(
                                "Expected error code: {:?}, got: {:?}",
                                expected_tx_error, &actual_tx_error
                            ))));
                        }
                    }
                    _ => {
                        return Err(Box::new(Error::TransactionExpectedFailure(format!(
                            "Expected error code: {:?}, got: {:?}",
                            expected_tx_error, actual_tx_error
                        ))));
                    }
                }

                return Err(Box::new(Error::TransactionExpectedFailure(format!(
                    "Expected error code: {:?}, got: {:?}",
                    expected_tx_error, program_error
                ))));
            }
            _ => {
                return Err(Box::new(Error::TransactionExpectedFailure(format!(
                    "Expected error code: {:?}, got: {:?}",
                    expected_tx_error, actual_tx_error
                ))));
            }
        }
    }

    if let Some(log_regex) = log_match_regex {
        let regexes = log_regex
            .iter()
            .map(|s| regex::Regex::new(s).unwrap())
            .collect::<Vec<regex::Regex>>();

        let logs = tx_metadata.metadata.unwrap().log_messages;
        for log in &logs {
            println!("{:?}", log);
        }

        // find one log that matches each regex
        for regex in regexes {
            let mut found = false;
            for log in &logs {
                if regex.is_match(log) {
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(Box::new(Error::LogNotFound(format!(
                    "Log not found: {}",
                    regex
                ))));
            }
        }
    }

    Ok(())
}

pub fn to_transaction_error(ix_index: u8, program_error: LighthouseError) -> TransactionError {
    TransactionError::InstructionError(
        ix_index,
        InstructionError::Custom(6000 + program_error as u32),
    )
}

pub fn to_transaction_error_u8(ix_index: u8, program_error: u32) -> TransactionError {
    TransactionError::InstructionError(ix_index, InstructionError::Custom(program_error))
}

pub async fn set_account_from_rpc(
    context: &mut TestContext,
    connection: &RpcClient,
    account_pubkey: &Pubkey,
) {
    let account = connection.get_account(account_pubkey).await.unwrap();

    let mut shared_account =
        AccountSharedData::new(account.lamports, account.data.len(), &account.owner);
    shared_account.set_data_from_slice(account.data.as_slice());

    context
        .program_context
        .set_account(account_pubkey, &shared_account);
}

pub async fn set_account_from_refs(
    context: &mut TestContext,
    account_pubkey: &Pubkey,
    data: &[u8],
    owner: &Pubkey,
) {
    let lamports = context
        .get_minimum_balance_for_rent_exemption(data.len())
        .await;

    let mut shared_account = AccountSharedData::new(lamports, data.len(), owner);
    shared_account.set_data_from_slice(data);

    context
        .program_context
        .set_account(account_pubkey, &shared_account);
}
