use crate::utils::context::TestContext;
use crate::utils::create_user_with_balance;
use crate::utils::utils::process_transaction_assert_success;
use lighthouse_client::instructions::AssertStakeAccountBuilder;
use lighthouse_client::types::{
    ComparableOperator, EquatableOperator, MetaAssertion, StakeAccountAssertion, StakeAccountState,
};
use solana_client::rpc_client::RpcClient;
use solana_program_test::tokio;
use solana_program_test::tokio::task::spawn_blocking;
use solana_sdk::account::AccountSharedData;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::stake;
use solana_sdk::stake::instruction::{delegate_stake, initialize};
use solana_sdk::stake::state::Lockup;
use solana_sdk::system_instruction::create_account_with_seed;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;

///
/// Tests all data types using the `StakeAccount` assertion.
///
#[tokio::test]
async fn test_borsh_account_data() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    // Clone a vote account from devnet.
    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let vote_pubkey =
        solana_sdk::pubkey::Pubkey::from_str("HRACkkKxJHZ22QRfky7QEsSRgxiskQVdK23XS13tjEGM")
            .unwrap();

    let vote_account = spawn_blocking(move || {
        connection
            .get_account(&vote_pubkey)
            .expect("Failed to get account data.")
    })
    .await
    .unwrap();

    let mut account = AccountSharedData::new(
        vote_account.lamports,
        vote_account.data.len(),
        &vote_account.owner,
    );
    account.set_data_from_slice(vote_account.data.as_slice());
    context.program_context.set_account(&vote_pubkey, &account);

    let derived_account =
        Pubkey::create_with_seed(&user.encodable_pubkey(), "stake:0", &stake::program::id())
            .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            create_account_with_seed(
                &user.encodable_pubkey(),
                &derived_account,
                &user.encodable_pubkey(),
                "stake:0",
                2e9 as u64,
                200,
                &stake::program::id(),
            ),
            initialize(
                &derived_account,
                &stake::state::Authorized {
                    staker: user.encodable_pubkey(),
                    withdrawer: user.encodable_pubkey(),
                },
                &Lockup {
                    epoch: 0,
                    unix_timestamp: 0,
                    custodian: user.encodable_pubkey(),
                },
            ),
            delegate_stake(&derived_account, &user.encodable_pubkey(), &vote_pubkey),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx: Transaction = Transaction::new_signed_with_payer(
        &[
            AssertStakeAccountBuilder::new()
                .target_account(derived_account)
                .stake_account_assertion(StakeAccountAssertion::State {
                    value: StakeAccountState::Stake as u8,
                    operator: ComparableOperator::Equal,
                })
                .instruction(),
            AssertStakeAccountBuilder::new()
                .target_account(derived_account)
                .stake_account_assertion(StakeAccountAssertion::MetaAssertion(
                    MetaAssertion::AuthorizedStaker(
                        user.encodable_pubkey(),
                        EquatableOperator::Equal,
                    ),
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

// TODO: Test all states of a stake account
