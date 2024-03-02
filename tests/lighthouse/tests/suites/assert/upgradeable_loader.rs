use crate::utils::context::TestContext;
use crate::utils::create_user_with_balance;
use crate::utils::utils::{process_transaction_assert_success, set_account_from_rpc};
use anchor_lang::accounts::program;
use anchor_lang::AccountDeserialize;
use lighthouse_client::instructions::AssertUpgradeableLoaderAccountBuilder;
use lighthouse_client::types::{
    ComparableOperator, EquatableOperator, MetaAssertion, StakeAccountAssertion,
    UpgradeableLoaderStateAssertion, UpgradeableLoaderStateType, UpgradeableProgramAssertion,
    UpgradeableProgramDataAssertion,
};
use solana_banks_interface::BanksClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_program_test::tokio;
use solana_program_test::tokio::task::spawn_blocking;
use solana_sdk::account::AccountSharedData;
use solana_sdk::account_utils::StateMut;
use solana_sdk::bpf_loader_upgradeable::{
    self, deploy_with_max_program_len, UpgradeableLoaderState,
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::loader_v4::{create_buffer, LoaderV4Status};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::stake::instruction::{delegate_stake, initialize};
use solana_sdk::stake::state::Lockup;
use solana_sdk::system_instruction::create_account_with_seed;
use solana_sdk::transaction::Transaction;
use solana_sdk::{bpf_loader, bpf_loader_deprecated, loader_v4, system_instruction};
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
        String::from("https://devnet.helius-rpc.com/?api-key=8444b7e1-30fa-4f7d-9ba1-32dcd09e5fa5"),
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
                .upgradeable_loader_state_assertion(UpgradeableLoaderStateAssertion::State {
                    value: UpgradeableLoaderStateType::Program,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(program_pubkey)
                .upgradeable_loader_state_assertion(UpgradeableLoaderStateAssertion::Program(
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
                .upgradeable_loader_state_assertion(UpgradeableLoaderStateAssertion::State {
                    value: UpgradeableLoaderStateType::ProgramData,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .upgradeable_loader_state_assertion(UpgradeableLoaderStateAssertion::ProgramData(
                    UpgradeableProgramDataAssertion::UpgradeAuthority {
                        value: upgrade_authority_address,
                        operator: EquatableOperator::Equal,
                    },
                ))
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .upgradeable_loader_state_assertion(UpgradeableLoaderStateAssertion::ProgramData(
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

    // println!("{:?}", program_account);

    // let state: UpgradeableLoaderState = bincode::deserialize(&program_account.data).unwrap();

    // let deployer = create_user_with_balance(context, 1000).await;
    // let program_keypair = Keypair::new();
    // let program_id = program_keypair.encodable_pubkey();

    // if account.owner == bpf_loader::id() || account.owner == bpf_loader_deprecated::id() {
    //     println!("{:?}", account);
    //     println!("Account is a BPF loader account");
    // } else if account.owner == bpf_loader_upgradeable::id() {
    //     if let Ok(UpgradeableLoaderState::Program {
    //         programdata_address,
    //     }) = account.state()
    //     {
    //         set_account_from_rpc(context, &connection, &programdata_address).await;

    //         if let Some(programdata_account) = context
    //             .client()
    //             .get_account(programdata_address)
    //             .await
    //             .unwrap()
    //         {
    //             let state: UpgradeableLoaderState = programdata_account.state().unwrap();

    //             println!("{:?}", state);

    //             // if let Ok(UpgradeableLoaderState::ProgramData {
    //             //     upgrade_authority_address,
    //             //     slot,
    //             // }) = programdata_account.state()
    //             // {
    //             //     println!("{:?}", programdata_account);
    //             // } else {
    //             //     println!("Program {account_pubkey} has been closed")
    //             // }
    //         } else {
    //             println!("Program {account_pubkey} has been closed")
    //         }
    //     } else if let Ok(UpgradeableLoaderState::Buffer { authority_address }) = account.state() {
    //         println!("{:?}", authority_address);

    //         // Ok(config
    //         //     .output_format
    //         //     .formatted_string(&CliUpgradeableBuffer {
    //         //         address: account_pubkey.to_string(),
    //         //         authority: authority_address
    //         //             .map(|pubkey| pubkey.to_string())
    //         //             .unwrap_or_else(|| "none".to_string()),
    //         //         data_len: account.data.len()
    //         //             - UpgradeableLoaderState::size_of_buffer_metadata(),
    //         //         lamports: account.lamports,
    //         //         use_lamports_unit,
    //         //     }))
    //     } else {
    //         println!("{account_pubkey} is not an upgradeable loader Buffer or Program account")
    //     }
    // } else {
    //     println!("{account_pubkey} is not an SBF program")
    // }

    // if loader_v4::check_id(&program_account.owner) {
    //     if let Ok(state) = solana_loader_v4_program::get_state(&program_account.data) {
    //         let status = match state.status {
    //             LoaderV4Status::Retracted => "retracted",
    //             LoaderV4Status::Deployed => "deployed",
    //             LoaderV4Status::Finalized => "finalized",
    //         };

    //         println!("{:?}", state);
    //     } else {
    //         println!("SBF program state is invalid")
    //     }
    // } else {
    //     println!("is not an SBF program")
    // }

    // let tx = Transaction::new_signed_with_payer(
    //     &[
    //         create_buffer
    //         deploy_with_max_program_len(
    //         &deployer.pubkey(),
    //         &program_id,
    //         bpf_loader_upgradeable::id(),
    //         &deployer.pubkey(),
    //         &deployer.pubkey(),
    //         1000,
    //     )],
    //     Some(&deployer.pubkey()),
    //     &[&deployer, &program_keypair],
    //     context.get_blockhash(),
    // );

    // println!("{:?}", state);

    // let deployer = create_user_with_balance(context, 1000u64 * 1e9 as u64)
    //     .await
    //     .unwrap();
    // let program_id = Keypair::new();

    // create_bpf_upgradable_program(context, &deployer, &program_id, 1000).await;

    // SNPRohhBurQwrpwAptw1QYtpFdfEKitr4WSJ125cN1g
}

// async fn create_bpf_upgradable_program(
//     context: &mut TestContext,
//     deployer: &Keypair,
//     program_id: &Keypair,
//     program_len: usize,
//     loader_id: &Pubkey,
// ) {
//     let account_size = UpgradeableLoaderState::size_of_buffer(10_000);
//     let min_rent_exempt_program_data_balance = context
//         .get_minimum_balance_for_rent_exemption(account_size)
//         .await;

//     let buffer = Keypair::new();
//     let buffer_pubkey = buffer.encodable_pubkey();

//     let create_msg = |offset: u32, bytes: Vec<u8>| {
//         let instruction = if loader_id == &bpf_loader_upgradeable::id() {
//             bpf_loader_upgradeable::write(
//                 buffer_pubkey,
//                 &buffer_authority_signer.pubkey(),
//                 offset,
//                 bytes,
//             )
//         } else {
//             loader_instruction::write(buffer_pubkey, loader_id, offset, bytes)
//         };
//         Message::new_with_blockhash(&[instruction], Some(&fee_payer_signer.pubkey()), &blockhash)
//     };

//     // let ixs = vec![
//     //     system_instruction::transfer(
//     //         &deployer.encodable_pubkey(),
//     //         &buffer_key,
//     //         min_rent_exempt_program_data_balance,
//     //     ),
//     //     system_instruction::allocate(&buffer_key, account_size as u64),
//     //     system_instruction::assign(&buffer_key, &bpf_loader_upgradeable::id()),
//     // ];

//     // let tx = Transaction::new_signed_with_payer(
//     //     &ixs,
//     //     Some(&deployer.encodable_pubkey()),
//     //     &[&deployer, &buffer],
//     //     context.get_blockhash().await,
//     // );

//     // process_transaction_assert_success(context, tx)
//     //     .await
//     //     .unwrap();

//     println!("Deploying program");

//     let ixs = bpf_loader_upgradeable::create_buffer(
//         &deployer.encodable_pubkey(),
//         &buffer_pubkey,
//         &deployer.encodable_pubkey(),
//         min_rent_exempt_program_data_balance,
//         program_len,
//     )
//     .unwrap();

//     let tx = Transaction::new_signed_with_payer(
//         &ixs,
//         Some(&deployer.encodable_pubkey()),
//         &[&deployer, &buffer],
//         context.get_blockhash().await,
//     );

//     process_transaction_assert_success(context, tx)
//         .await
//         .unwrap();

//     let deploy_ixs = bpf_loader_upgradeable::deploy_with_max_program_len(
//         &deployer.encodable_pubkey(),
//         &program_id.encodable_pubkey(),
//         &buffer_pubkey,
//         &deployer.encodable_pubkey(),
//         context
//             .get_minimum_balance_for_rent_exemption(UpgradeableLoaderState::size_of_program())
//             .await,
//         program_len,
//     )
//     .unwrap();

//     let tx = Transaction::new_signed_with_payer(
//         &deploy_ixs,
//         Some(&deployer.encodable_pubkey()),
//         &[&deployer, &program_id],
//         context.get_blockhash().await,
//     );

//     process_transaction_assert_success(context, tx)
//         .await
//         .unwrap();
// }
