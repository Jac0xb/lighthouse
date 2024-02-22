use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use lighthouse::types::{
    AccountInfoAssertion, ComparableOperator, EquatableOperator, KnownProgram,
};
use rust_sdk::LighthouseProgram;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;

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
    SafeSend { to_pubkey: String },
}

fn main() {
    let cli = Cli::parse();

    // if let Some(config_path) = cli.keypair.as_deref() {
    // println!("Value for config: {}", config_path.display());

    let lighthouse_program = LighthouseProgram {};
    let keypair_path = cli.keypair.as_path();
    let data = std::fs::read_to_string(keypair_path).unwrap();

    let numbers: Vec<u8> = serde_json::from_str(&data).unwrap();
    // }

    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let wallet_keypair = Keypair::from_bytes(&numbers).unwrap();

    // let ix = system_instruction::transfer(&from_pubkey, &to_pubkey, 1_000_000);

    let txn = match &cli.command {
        Some(Commands::SafeSend { to_pubkey }) => {
            let to_pubkey = solana_sdk::pubkey::Pubkey::from_str(to_pubkey).unwrap();

            build_safe_send_transaction(&connection, &wallet_keypair, &to_pubkey, 1_000_000)
        }
        None => {
            panic!("No command specified.")
            // Transaction::new_signed_with_payer(
            //     &[ix],
            //     Some(&from_pubkey),
            //     &[&wallet_keypair],
            //     recent_blockhash.0,
            // )
        }
    };

    // Sending the transfer sol transaction
    let sig = connection.send_and_confirm_transaction_with_spinner_and_config(
        &txn,
        CommitmentConfig {
            commitment: solana_sdk::commitment_config::CommitmentLevel::Confirmed,
        },
        RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
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
        Err(e) => println!(
            "Error transferring Sol:, {:?} {}",
            txn.signatures.first(),
            e
        ),
    }
    // // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level cmd
    // match &cli.command {
    //     Some(Commands::Test { list }) => {
    //         if *list {
    //             println!("Printing testing lists...");
    //         } else {
    //             println!("Not printing testing lists...");
    //         }
    //     }
    //     None => {}
    // }

    // Continued program logic goes here...
}

pub fn build_safe_send_transaction(
    connection: &RpcClient,
    from_keypair: &Keypair,
    to_pubkey: &solana_sdk::pubkey::Pubkey,
    amount: u64,
) -> Transaction {
    let (balance, recent_blockhash) = rayon::join(
        || {
            connection
                .get_balance(&from_keypair.encodable_pubkey())
                .expect("Failed to get balance.")
        },
        || {
            connection
                .get_latest_blockhash()
                .expect("Failed to get latest blockhash.")
        },
    );

    let lighthouse_program = LighthouseProgram {};
    let ix = system_instruction::transfer(&from_keypair.pubkey(), to_pubkey, amount);

    Transaction::new_signed_with_payer(
        &[
            ix,
            lighthouse_program
                .assert_account_info(
                    from_keypair.pubkey(),
                    AccountInfoAssertion::KnownOwner(
                        KnownProgram::System,
                        EquatableOperator::Equal,
                    ),
                    None,
                )
                .ix(),
            lighthouse_program
                .assert_account_info(
                    from_keypair.pubkey(),
                    AccountInfoAssertion::Lamports(
                        balance - amount - 5000,
                        ComparableOperator::Equal,
                    ),
                    None,
                )
                .ix(),
        ],
        Some(&from_keypair.pubkey()),
        &[from_keypair],
        recent_blockhash,
    )
}
