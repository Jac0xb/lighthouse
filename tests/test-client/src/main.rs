use anchor_spl::associated_token;
use borsh::BorshDeserialize;
use clap::{Parser, Subcommand};
use lighthaus_sdk::find_memory_pda;
use lighthaus_sdk::instructions::{
    AssertAccountDeltaBuilder, AssertAccountInfoBuilder, AssertStakeAccountBuilder,
    MemoryCloseBuilder, MemoryWriteBuilder,
};
use lighthaus_sdk::types::{
    AccountDeltaAssertion, AccountInfoAssertion, DataValueDeltaAssertion, EquatableOperator,
    IntegerOperator, KnownProgram, LogLevel, MetaAssertion, StakeAccountAssertion, StakeStateType,
    WriteType,
};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::stake::state::StakeStateV2;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Mint;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    keypair: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    SafeSendSol {
        to_pubkey: String,
    },
    SafeSendToken {
        mint: String,
        to_pubkey: String,
    },
    AssertStake {
        stake_pubkey: String,
    },
    MintToken {
        mint_authority: Option<String>,
        freeze_authority: Option<String>,
        decimals: u8,
        mint_to: String,
        mint_to_amount: u64,
    },
}

fn main() {
    let cli = Cli::parse();

    let keypair_path = cli.keypair.as_path();
    let data = std::fs::read_to_string(keypair_path).unwrap();
    let numbers: Vec<u8> = serde_json::from_str(&data).unwrap();
    let wallet_keypair = Keypair::from_bytes(&numbers).unwrap();

    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let tx = match &cli.command {
        Some(Commands::SafeSendSol { to_pubkey }) => {
            let to_pubkey = solana_sdk::pubkey::Pubkey::from_str(to_pubkey).unwrap();

            build_safe_send_transaction(&connection, &wallet_keypair, &to_pubkey, 1_000_000)
        }
        Some(Commands::SafeSendToken { mint, to_pubkey }) => {
            let mint = solana_sdk::pubkey::Pubkey::from_str(mint).unwrap();
            let to_pubkey = solana_sdk::pubkey::Pubkey::from_str(to_pubkey).unwrap();

            build_safe_send_token_transaction(&connection, &wallet_keypair, &mint, &to_pubkey)
        }
        Some(Commands::AssertStake { stake_pubkey }) => {
            let stake_pubkey = solana_sdk::pubkey::Pubkey::from_str(stake_pubkey).unwrap();

            build_assert_stake_transaction(&connection, &wallet_keypair, stake_pubkey)
        }
        Some(Commands::MintToken {
            mint_authority,
            freeze_authority,
            decimals,
            mint_to,
            mint_to_amount,
        }) => {
            let mint_authority = mint_authority.as_ref().map(|mint_authority| {
                Some(solana_sdk::pubkey::Pubkey::from_str(mint_authority).unwrap())
            });
            let freeze_authority = freeze_authority.as_ref().map(|freeze_authority| {
                solana_sdk::pubkey::Pubkey::from_str(freeze_authority).unwrap()
            });

            let mint_to = Some((
                solana_sdk::pubkey::Pubkey::from_str(mint_to).unwrap(),
                *mint_to_amount,
            ));

            build_create_mint(
                &connection,
                &wallet_keypair,
                mint_authority,
                freeze_authority,
                *decimals,
                mint_to,
            )
        }
        None => {
            panic!("No command specified.")
        }
    };

    // Sending the transfer sol transaction
    let sig = connection.send_and_confirm_transaction_with_spinner_and_config(
        &tx,
        CommitmentConfig {
            commitment: solana_sdk::commitment_config::CommitmentLevel::Confirmed,
        },
        RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
            max_retries: Some(5),
            min_context_slot: None,
        },
    );

    match sig {
        Ok(sig) => loop {
            if let Ok(confirmed) = connection.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(e) => println!("Error transferring Sol:, {:?} {}", tx.signatures.first(), e),
    }
}

pub fn build_create_mint(
    connection: &RpcClient,
    wallet_keypair: &Keypair,
    mint_authority: Option<Option<Pubkey>>,
    freeze_authority: Option<Pubkey>,
    decimals: u8,
    mint_to: Option<(Pubkey, u64)>,
) -> Transaction {
    let (tx, _) = create_mint(
        connection,
        wallet_keypair,
        CreateMintParameters {
            token_program: spl_token::ID,
            mint_authority,
            freeze_authority,
            decimals,
            mint_to: Some((
                mint_to.unwrap().0,
                mint_to.unwrap().1 * 10u64.pow(u32::from(decimals)),
            )),
        },
    )
    .unwrap();

    tx
}

pub fn build_safe_send_transaction(
    connection: &RpcClient,
    from_keypair: &Keypair,
    to_pubkey: &solana_sdk::pubkey::Pubkey,
    amount: u64,
) -> Transaction {
    let (balance, (recent_blockhash, _)) = rayon::join(
        || {
            connection
                .get_balance(&from_keypair.encodable_pubkey())
                .expect("Failed to get balance.")
        },
        || {
            connection
                .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
                .expect("Failed to get latest blockhash.")
        },
    );

    let ix = system_instruction::transfer(&from_keypair.pubkey(), to_pubkey, amount);

    Transaction::new_signed_with_payer(
        &[
            ix,
            AssertAccountInfoBuilder::new()
                .target_account(from_keypair.pubkey())
                .assertion(AccountInfoAssertion::KnownOwner {
                    value: KnownProgram::System,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(from_keypair.pubkey())
                .assertion(AccountInfoAssertion::Lamports {
                    value: balance - amount - 5000,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
        ],
        Some(&from_keypair.pubkey()),
        &[from_keypair],
        recent_blockhash,
    )
}

fn build_safe_send_token_transaction(
    connection: &RpcClient,
    wallet_keypair: &Keypair,
    mint: &Pubkey,
    destination_user: &Pubkey,
) -> Transaction {
    let token_account = get_associated_token_address(&wallet_keypair.pubkey(), mint);
    let (memory, memory_bump) = find_memory_pda(wallet_keypair.pubkey(), 0);
    let dest_token_account = get_associated_token_address(destination_user, mint);

    let tx = Transaction::new_signed_with_payer(
        &[
            MemoryWriteBuilder::new()
                .payer(wallet_keypair.pubkey())
                .source_account(token_account)
                .memory(memory)
                .memory_id(0)
                .write_offset(0)
                .memory_bump(memory_bump)
                .write_type(WriteType::AccountData {
                    offset: 0,
                    data_length: 72,
                })
                .instruction(),
            spl_token::instruction::transfer(
                &spl_token::id(),
                &token_account,
                &dest_token_account,
                &wallet_keypair.pubkey(),
                &[],
                69,
            )
            .unwrap(),
            AssertAccountDeltaBuilder::new()
                .account_a(memory)
                .account_b(token_account)
                .assertion(AccountDeltaAssertion::Data {
                    a_offset: 0,
                    b_offset: 0,
                    assertion: DataValueDeltaAssertion::Bytes {
                        operator: EquatableOperator::Equal,
                        length: 64,
                    },
                })
                .log_level(LogLevel::Silent)
                .instruction(),
            AssertAccountDeltaBuilder::new()
                .account_a(memory)
                .account_b(token_account)
                .assertion(AccountDeltaAssertion::Data {
                    a_offset: 64,
                    b_offset: 64,
                    assertion: DataValueDeltaAssertion::U64 {
                        value: -100,
                        operator: IntegerOperator::GreaterThan,
                    },
                })
                .log_level(LogLevel::PlaintextMessage)
                .instruction(),
            MemoryCloseBuilder::new()
                .payer(wallet_keypair.pubkey())
                .memory(memory)
                .memory_bump(memory_bump)
                .memory_id(0)
                .instruction(),
        ],
        Some(&wallet_keypair.pubkey()),
        &[&wallet_keypair],
        connection.get_latest_blockhash().unwrap(),
    );

    tx
}

fn build_assert_stake_transaction(
    connection: &RpcClient,
    payer: &Keypair,
    stake_pubkey: Pubkey,
) -> Transaction {
    let (blockhash, _) = connection
        .get_latest_blockhash_with_commitment(CommitmentConfig::finalized())
        .expect("Failed to get latest blockhash.");

    let account_data = connection
        .get_account(&stake_pubkey)
        .expect("Failed to get account data.");
    let stake_data = &mut account_data.data.as_slice();
    let stake_state = StakeStateV2::deserialize(stake_data).unwrap();

    match stake_state {
        StakeStateV2::Uninitialized => panic!("Stake account is not initialized."),
        StakeStateV2::Initialized(meta) => Transaction::new_signed_with_payer(
            &[
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::State {
                        value: StakeStateType::Initialized,
                        operator: EquatableOperator::Equal,
                    })
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedStaker {
                            value: meta.authorized.staker,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedWithdrawer {
                            value: meta.authorized.withdrawer,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupEpoch {
                            value: meta.lockup.epoch,
                            operator: IntegerOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupUnixTimestamp {
                            value: meta.lockup.unix_timestamp,
                            operator: IntegerOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupCustodian {
                            value: meta.lockup.custodian,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::RentExemptReserve {
                            value: meta.rent_exempt_reserve,
                            operator: IntegerOperator::Equal,
                        },
                    ))
                    .instruction(),
            ],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        ),
        StakeStateV2::RewardsPool => panic!("Stake account is a rewards pool."),
        StakeStateV2::Stake(meta, _stake, _stake_flags) => Transaction::new_signed_with_payer(
            &[
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::State {
                        value: StakeStateType::Stake,
                        operator: EquatableOperator::Equal,
                    })
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedStaker {
                            value: meta.authorized.staker,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedWithdrawer {
                            value: meta.authorized.withdrawer,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupEpoch {
                            value: meta.lockup.epoch,
                            operator: IntegerOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupUnixTimestamp {
                            value: meta.lockup.unix_timestamp,
                            operator: IntegerOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupCustodian {
                            value: meta.lockup.custodian,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_pubkey)
                    .assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::RentExemptReserve {
                            value: meta.rent_exempt_reserve,
                            operator: IntegerOperator::Equal,
                        },
                    ))
                    .instruction(),
            ],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        ),
    }
}

pub struct CreateMintParameters {
    pub token_program: Pubkey,
    pub mint_authority: Option<Option<Pubkey>>,
    pub freeze_authority: Option<Pubkey>,
    pub decimals: u8,
    pub mint_to: Option<(Pubkey, u64)>,
}

pub fn create_mint(
    client: &RpcClient,
    payer: &Keypair,
    parameters: CreateMintParameters,
) -> Result<(Transaction, Keypair), Box<dyn Error>> {
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
        client.get_latest_blockhash().unwrap(),
    )
    .unwrap();

    Ok((tx, mint))
}
