use std::path::PathBuf;
use std::str::FromStr;

use borsh::BorshDeserialize;
use clap::{Parser, Subcommand};
use lighthouse_client::instructions::{AssertAccountInfoBuilder, AssertStakeAccountBuilder};
use lighthouse_client::types::{
    AccountInfoAssertion, ComparableOperator, EquatableOperator, KnownProgram, MetaAssertion,
    StakeAccountAssertion, StakeStateType,
};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_program::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::stake::state::StakeStateV2;
use solana_sdk::transaction::Transaction;
use solana_sdk::vote::state::VoteState;

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

    let keypair_path = cli.keypair.as_path();
    let data = std::fs::read_to_string(keypair_path).unwrap();
    let numbers: Vec<u8> = serde_json::from_str(&data).unwrap();
    let wallet_keypair = Keypair::from_bytes(&numbers).unwrap();

    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let vote_pubkey =
        solana_sdk::pubkey::Pubkey::from_str("HRACkkKxJHZ22QRfky7QEsSRgxiskQVdK23XS13tjEGM")
            .unwrap();

    let account_data = connection
        .get_account(&vote_pubkey)
        .expect("Failed to get account data.");

    let vote_account = VoteState::deserialize(account_data.data.as_slice()).unwrap();

    let stake_pubkey = "AnmMkv5yfHVszKAsTYjjNp2xj71zjt6NoUYu2ebkVztc";
    let stake_pubkey = solana_sdk::pubkey::Pubkey::from_str(stake_pubkey).unwrap();

    let account_data = connection
        .get_account(&stake_pubkey)
        .expect("Failed to get account data.");

    println!("Account: {:?}", account_data);

    let stake_data = &mut account_data.data.as_slice();

    let stake_account = StakeStateV2::deserialize(stake_data).unwrap();

    println!("Vote account data: {:?}", vote_account);
    println!("Stake account data: {:?}", stake_account);

    // StakeStateV2::try_from(account_data.data.as_slice()).unwrap();

    // let txn = match &cli.command {
    //     Some(Commands::SafeSend { to_pubkey }) => {
    //         let to_pubkey = solana_sdk::pubkey::Pubkey::from_str(to_pubkey).unwrap();

    //         build_safe_send_transaction(&connection, &wallet_keypair, &to_pubkey, 1_000_000)
    //     }
    //     None => {
    //         panic!("No command specified.")
    //         // Transaction::new_signed_with_payer(
    //         //     &[ix],
    //         //     Some(&from_pubkey),
    //         //     &[&wallet_keypair],
    //         //     recent_blockhash.0,
    //         // )
    //     }
    // };

    let tx =
        build_assert_stake_transaction(&connection, &wallet_keypair, stake_pubkey, &stake_account);

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
                .account_info_assertion(AccountInfoAssertion::KnownOwner {
                    value: KnownProgram::System,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(from_keypair.pubkey())
                .account_info_assertion(AccountInfoAssertion::Lamports {
                    value: balance - amount - 5000,
                    operator: ComparableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&from_keypair.pubkey()),
        &[from_keypair],
        recent_blockhash,
    )
}

fn build_assert_stake_transaction(
    connection: &RpcClient,
    payer: &Keypair,
    stake_state_pubkey: Pubkey,
    stake_state: &StakeStateV2,
) -> Transaction {
    let (blockhash, _) = connection
        .get_latest_blockhash_with_commitment(CommitmentConfig::finalized())
        .expect("Failed to get latest blockhash.");

    match stake_state {
        StakeStateV2::Uninitialized => panic!("Stake account is not initialized."),
        StakeStateV2::Initialized(meta) => Transaction::new_signed_with_payer(
            &[
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::State {
                        value: StakeStateType::Initialized,
                        operator: EquatableOperator::Equal,
                    })
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedStaker {
                            value: meta.authorized.staker,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedWithdrawer {
                            value: meta.authorized.withdrawer,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupEpoch {
                            value: meta.lockup.epoch,
                            operator: ComparableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupUnixTimestamp {
                            value: meta.lockup.unix_timestamp,
                            operator: ComparableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupCustodian {
                            value: meta.lockup.custodian,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::RentExemptReserve {
                            value: meta.rent_exempt_reserve,
                            operator: ComparableOperator::Equal,
                        },
                    ))
                    .instruction(),
            ],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        ),
        StakeStateV2::RewardsPool => panic!("Stake account is a rewards pool."),
        StakeStateV2::Stake(meta, stake, _stake_flags) => Transaction::new_signed_with_payer(
            &[
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::State {
                        value: StakeStateType::Stake,
                        operator: EquatableOperator::Equal,
                    })
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedStaker {
                            value: meta.authorized.staker,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::AuthorizedWithdrawer {
                            value: meta.authorized.withdrawer,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupEpoch {
                            value: meta.lockup.epoch,
                            operator: ComparableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupUnixTimestamp {
                            value: meta.lockup.unix_timestamp,
                            operator: ComparableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::LockupCustodian {
                            value: meta.lockup.custodian,
                            operator: EquatableOperator::Equal,
                        },
                    ))
                    .instruction(),
                AssertStakeAccountBuilder::new()
                    .target_account(stake_state_pubkey)
                    .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                        MetaAssertion::RentExemptReserve {
                            value: meta.rent_exempt_reserve,
                            operator: ComparableOperator::Equal,
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
