use crate::utils::context::TestContext;
use crate::utils::{create_user_with_balance, set_account_from_refs};
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use borsh::{BorshDeserialize, BorshSerialize};
use lighthouse_client::errors::LighthouseError;
use lighthouse_client::instructions::AssertStakeAccountBuilder;
use lighthouse_client::types::{
    ComparableOperator, EquatableOperator, IntegerOperator, MetaAssertion, StakeAccountAssertion,
    StakeAssertion, StakeStateType,
};
use solana_program_test::tokio;
use solana_sdk::clock::Clock;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::stake::instruction::{delegate_stake, initialize};
use solana_sdk::stake::stake_flags::StakeFlags;
use solana_sdk::stake::state::{Lockup, StakeStateV2};
use solana_sdk::system_instruction::create_account_with_seed;
use solana_sdk::transaction::Transaction;
use solana_sdk::{stake, vote};
use solana_vote_program::vote_state::{VoteInit, VoteState, VoteStateVersions};
use std::mem::size_of;
use std::str::FromStr;

///
/// Tests all data types using the `StakeAccount` assertion.
///
#[tokio::test]
async fn test() {
    let context: &mut TestContext = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    context.warp_to_slot(100_000_000).unwrap();

    let vote_pubkey =
        solana_sdk::pubkey::Pubkey::from_str("HRACkkKxJHZ22QRfky7QEsSRgxiskQVdK23XS13tjEGM")
            .unwrap();

    let vote_state = vote::state::VoteState::new(
        &VoteInit {
            node_pubkey: user.encodable_pubkey(),
            authorized_voter: user.encodable_pubkey(),
            authorized_withdrawer: user.encodable_pubkey(),
            commission: 0,
        },
        &Clock::default(),
    );

    let output = &mut vec![0; size_of::<VoteState>()];
    VoteState::serialize(&VoteStateVersions::Current(Box::new(vote_state)), output).unwrap();
    set_account_from_refs(context, &vote_pubkey, output, &vote::program::id()).await;

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

    let data = context
        .client()
        .get_account(derived_account)
        .await
        .unwrap()
        .unwrap()
        .data;
    let stake_account = StakeStateV2::deserialize(&mut data.as_ref()).unwrap();

    let (meta, stake, _flags) = match stake_account {
        StakeStateV2::Stake(meta, stake, flags) => (meta, stake, flags),
        _ => panic!("Stake account is not in the initialized state"),
    };

    let builder_fn = |assertion: StakeAccountAssertion| {
        AssertStakeAccountBuilder::new()
            .target_account(derived_account)
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(assertion)
            .instruction()
    };

    let tx: Transaction = Transaction::new_signed_with_payer(
        &[
            // ()
            builder_fn(StakeAccountAssertion::State {
                value: StakeStateType::Stake,
                operator: EquatableOperator::Equal,
            }),
            // Negation
            builder_fn(StakeAccountAssertion::State {
                value: StakeStateType::Initialized,
                operator: EquatableOperator::NotEqual,
            }),
            // ()
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupCustodian {
                    value: meta.lockup.custodian,
                    operator: EquatableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupCustodian {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupEpoch {
                    value: meta.lockup.epoch,
                    operator: ComparableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupEpoch {
                    value: 69,
                    operator: ComparableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupUnixTimestamp {
                    value: meta.lockup.unix_timestamp,
                    operator: ComparableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupUnixTimestamp {
                    value: 69,
                    operator: ComparableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedStaker {
                    value: meta.authorized.staker,
                    operator: EquatableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedStaker {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedWithdrawer {
                    value: meta.authorized.withdrawer,
                    operator: EquatableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedWithdrawer {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::RentExemptReserve {
                    value: meta.rent_exempt_reserve,
                    operator: ComparableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::RentExemptReserve {
                    value: 69,
                    operator: ComparableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::CreditsObserved {
                    value: stake.credits_observed,
                    operator: ComparableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::CreditsObserved {
                    value: 69,
                    operator: ComparableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationStake {
                    value: stake.delegation.stake,
                    operator: ComparableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationStake {
                    value: 69,
                    operator: ComparableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationDeactivationEpoch {
                    value: stake.delegation.deactivation_epoch,
                    operator: ComparableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationDeactivationEpoch {
                    value: 69,
                    operator: ComparableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationActivationEpoch {
                    value: stake.delegation.activation_epoch,
                    operator: ComparableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationActivationEpoch {
                    value: 69,
                    operator: ComparableOperator::NotEqual,
                },
            )),
            // ()
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationVoterPubkey {
                    value: stake.delegation.voter_pubkey,
                    operator: EquatableOperator::Equal,
                },
            )),
            // Negation
            builder_fn(StakeAccountAssertion::StakeAssertion(
                StakeAssertion::DelegationVoterPubkey {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::NotEqual,
                },
            )),
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: u8::MAX,
                operator: IntegerOperator::DoesNotContain,
            }),
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: u8::MIN,
                operator: IntegerOperator::Contains,
            }),
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: u8::MIN,
                operator: IntegerOperator::Equal,
            }),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let fail_cases = [
        builder_fn(StakeAccountAssertion::State {
            value: StakeStateType::Initialized,
            operator: EquatableOperator::Equal,
        }),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::LockupCustodian {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::LockupEpoch {
                value: 69,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::LockupUnixTimestamp {
                value: 69,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::AuthorizedStaker {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::AuthorizedWithdrawer {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::RentExemptReserve {
                value: 69,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::StakeAssertion(
            StakeAssertion::CreditsObserved {
                value: 69,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::StakeAssertion(
            StakeAssertion::DelegationStake {
                value: 69,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::StakeAssertion(
            StakeAssertion::DelegationDeactivationEpoch {
                value: 69,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::StakeAssertion(
            StakeAssertion::DelegationActivationEpoch {
                value: 69,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::StakeAssertion(
            StakeAssertion::DelegationVoterPubkey {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
    ];

    for ix in fail_cases.iter() {
        let tx = Transaction::new_signed_with_payer(
            &[ix.clone()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AssertionFailed),
            None,
        )
        .await
        .unwrap();
    }

    let stake_pubkey = Keypair::new().encodable_pubkey();
    let stake_state = StakeStateV2::Uninitialized;

    set_account_from_refs(
        context,
        &stake_pubkey,
        &stake_state.try_to_vec().unwrap(),
        &stake::program::id(),
    )
    .await;

    let builder_fn = |assertion: StakeAccountAssertion| {
        AssertStakeAccountBuilder::new()
            .target_account(stake_pubkey)
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(assertion)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[builder_fn(StakeAccountAssertion::State {
            value: StakeStateType::Uninitialized,
            operator: EquatableOperator::Equal,
        })],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let fail_ixs = [
        builder_fn(StakeAccountAssertion::StakeAssertion(
            StakeAssertion::CreditsObserved {
                value: 0,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::AuthorizedStaker {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::StakeFlags {
            value: 0,
            operator: IntegerOperator::Equal,
        }),
    ];

    for ix in fail_ixs.iter() {
        let tx = Transaction::new_signed_with_payer(
            &[ix.clone()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AssertionFailed),
            None,
        )
        .await
        .unwrap();
    }

    let stake_state = StakeStateV2::RewardsPool;

    set_account_from_refs(
        context,
        &stake_pubkey,
        &stake_state.try_to_vec().unwrap(),
        &stake::program::id(),
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[builder_fn(StakeAccountAssertion::State {
            value: StakeStateType::RewardsPool,
            operator: EquatableOperator::Equal,
        })],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let fail_ixs = [
        builder_fn(StakeAccountAssertion::StakeAssertion(
            StakeAssertion::CreditsObserved {
                value: 0,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::AuthorizedStaker {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::StakeFlags {
            value: 0,
            operator: IntegerOperator::Equal,
        }),
    ];

    for ix in fail_ixs.iter() {
        let tx = Transaction::new_signed_with_payer(
            &[ix.clone()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AssertionFailed),
            None,
        )
        .await
        .unwrap();
    }

    let staker = Keypair::new().encodable_pubkey();
    let withdrawer = Keypair::new().encodable_pubkey();
    let custodian = Keypair::new().encodable_pubkey();

    let stake_state = StakeStateV2::Initialized(stake::state::Meta {
        rent_exempt_reserve: 0,
        authorized: stake::state::Authorized { staker, withdrawer },
        lockup: Lockup {
            epoch: 69,
            unix_timestamp: 69,
            custodian,
        },
    });

    set_account_from_refs(
        context,
        &stake_pubkey,
        &stake_state.try_to_vec().unwrap(),
        &stake::program::id(),
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedStaker {
                    value: staker,
                    operator: EquatableOperator::Equal,
                },
            )),
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::AuthorizedWithdrawer {
                    value: withdrawer,
                    operator: EquatableOperator::Equal,
                },
            )),
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupCustodian {
                    value: custodian,
                    operator: EquatableOperator::Equal,
                },
            )),
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupEpoch {
                    value: 69,
                    operator: ComparableOperator::Equal,
                },
            )),
            builder_fn(StakeAccountAssertion::MetaAssertion(
                MetaAssertion::LockupUnixTimestamp {
                    value: 69,
                    operator: ComparableOperator::Equal,
                },
            )),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let fail_ixs = [
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::AuthorizedStaker {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::AuthorizedWithdrawer {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::LockupCustodian {
                value: Keypair::new().encodable_pubkey(),
                operator: EquatableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::LockupEpoch {
                value: 0,
                operator: ComparableOperator::Equal,
            },
        )),
        builder_fn(StakeAccountAssertion::MetaAssertion(
            MetaAssertion::LockupUnixTimestamp {
                value: 0,
                operator: ComparableOperator::Equal,
            },
        )),
    ];

    for ix in fail_ixs.iter() {
        let tx = Transaction::new_signed_with_payer(
            &[ix.clone()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AssertionFailed),
            None,
        )
        .await
        .unwrap();
    }

    let stake_state = StakeStateV2::Stake(
        meta,
        stake,
        StakeFlags::MUST_FULLY_ACTIVATE_BEFORE_DEACTIVATION_IS_PERMITTED,
    );

    set_account_from_refs(
        context,
        &stake_pubkey,
        &stake_state.try_to_vec().unwrap(),
        &stake::program::id(),
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: 1,
                operator: IntegerOperator::Contains,
            }),
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: u8::MAX,
                operator: IntegerOperator::NotEqual,
            }),
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: u8::MAX - 1,
                operator: IntegerOperator::DoesNotContain,
            }),
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: u8::MAX,
                operator: IntegerOperator::LessThan,
            }),
            builder_fn(StakeAccountAssertion::StakeFlags {
                value: 0,
                operator: IntegerOperator::GreaterThan,
            }),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}

#[tokio::test]
async fn not_owned_by_stake_program() {
    let context: &mut TestContext = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let builder_fn = |assertion: StakeAccountAssertion| {
        AssertStakeAccountBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::PlaintextMessage)
            .assertion(assertion)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[builder_fn(StakeAccountAssertion::State {
            value: StakeStateType::Uninitialized,
            operator: EquatableOperator::Equal,
        })],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AccountOwnerMismatch),
        None,
    )
    .await
    .unwrap();
}
