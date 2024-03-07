use crate::utils::context::TestContext;
use crate::utils::create_user_with_balance;
use crate::utils::utils::{process_transaction_assert_success, set_account_from_rpc};
use lighthouse_client::instructions::AssertUpgradeableLoaderAccountBuilder;
use lighthouse_client::types::{
    ComparableOperator, EquatableOperator, UpgradeableLoaderStateAssertion,
    UpgradeableLoaderStateType, UpgradeableProgramAssertion, UpgradeableProgramDataAssertion,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::tokio;
use solana_sdk::account_utils::StateMut;
use solana_sdk::bpf_loader_upgradeable::UpgradeableLoaderState;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;

///
/// Tests all data types using the `StakeAccount` assertion.
///
#[tokio::test]
async fn test_upgradeable_loader() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    // Clone a vote account from devnet.
    let connection = RpcClient::new_with_commitment(
        String::from("https://api.devnet.solana.com/"),
        CommitmentConfig::confirmed(),
    );

    let program_pubkey =
        solana_sdk::pubkey::Pubkey::from_str("M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K")
            .unwrap();

    set_account_from_rpc(context, &connection, &program_pubkey).await;

    let program_account = context
        .client()
        .get_account(program_pubkey)
        .await
        .unwrap()
        .unwrap();

    let programdata_address = if let Ok(UpgradeableLoaderState::Program {
        programdata_address,
    }) = program_account.state()
    {
        programdata_address
    } else {
        panic!(
            "{} is not an upgradeable loader Buffer or Program account",
            program_pubkey
        )
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(program_pubkey)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::State {
                    value: UpgradeableLoaderStateType::Program,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(program_pubkey)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::Program(
                    UpgradeableProgramAssertion::ProgramDataAddress {
                        value: programdata_address,
                        operator: EquatableOperator::Equal,
                    },
                ))
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    set_account_from_rpc(context, &connection, &programdata_address).await;
    let program_data_address_account = context
        .client()
        .get_account(programdata_address)
        .await
        .unwrap()
        .unwrap();

    let state: UpgradeableLoaderState = program_data_address_account.state().unwrap();

    let (slot, upgrade_authority_address) = if let UpgradeableLoaderState::ProgramData {
        slot,
        upgrade_authority_address,
    } = state
    {
        (slot, upgrade_authority_address)
    } else {
        panic!("Not a program")
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::State {
                    value: UpgradeableLoaderStateType::ProgramData,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                    UpgradeableProgramDataAssertion::UpgradeAuthority {
                        value: upgrade_authority_address,
                        operator: EquatableOperator::Equal,
                    },
                ))
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                    UpgradeableProgramDataAssertion::Slot {
                        value: slot,
                        operator: ComparableOperator::Equal,
                    },
                ))
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}
