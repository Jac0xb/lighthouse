use crate::utils::context::TestContext;
use crate::utils::{create_user_with_balance, set_account_from_refs, to_transaction_error_u8};
use crate::utils::{process_transaction_assert_failure, process_transaction_assert_success};
use borsh::BorshDeserialize;
use lighthouse_sdk::cpi::AssertStakeAccountMultiBuilder;
use lighthouse_sdk::types::{
    EquatableOperator, IntegerOperator, MetaAssertion, StakeAccountAssertion, StakeAssertion,
    StakeStateType,
};
use solana_program_test::tokio;
use solana_sdk::clock::Clock;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::stake::instruction::{delegate_stake, initialize};
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

    let tx: Transaction = Transaction::new_signed_with_payer(
        &[AssertStakeAccountMultiBuilder::new()
            .target_account(derived_account)
            .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
            .assertions(
                vec![
                    // ()
                    (StakeAccountAssertion::State {
                        value: StakeStateType::Stake,
                        operator: EquatableOperator::Equal,
                    }),
                    // Negation
                    (StakeAccountAssertion::State {
                        value: StakeStateType::Initialized,
                        operator: EquatableOperator::NotEqual,
                    }),
                    // ()
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupCustodian {
                        value: meta.lockup.custodian,
                        operator: EquatableOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupCustodian {
                        value: Keypair::new().encodable_pubkey(),
                        operator: EquatableOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupEpoch {
                        value: meta.lockup.epoch,
                        operator: IntegerOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupEpoch {
                        value: 69,
                        operator: IntegerOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupUnixTimestamp {
                        value: meta.lockup.unix_timestamp,
                        operator: IntegerOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupUnixTimestamp {
                        value: 69,
                        operator: IntegerOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedStaker {
                        value: meta.authorized.staker,
                        operator: EquatableOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedStaker {
                        value: Keypair::new().encodable_pubkey(),
                        operator: EquatableOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedWithdrawer {
                        value: meta.authorized.withdrawer,
                        operator: EquatableOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedWithdrawer {
                        value: Keypair::new().encodable_pubkey(),
                        operator: EquatableOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::RentExemptReserve {
                        value: meta.rent_exempt_reserve,
                        operator: IntegerOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::MetaAssertion(MetaAssertion::RentExemptReserve {
                        value: 69,
                        operator: IntegerOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::StakeAssertion(StakeAssertion::CreditsObserved {
                        value: stake.credits_observed,
                        operator: IntegerOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::StakeAssertion(StakeAssertion::CreditsObserved {
                        value: 69,
                        operator: IntegerOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationStake {
                        value: stake.delegation.stake,
                        operator: IntegerOperator::Equal,
                    })),
                    // Negation
                    (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationStake {
                        value: 69,
                        operator: IntegerOperator::NotEqual,
                    })),
                    // ()
                    (StakeAccountAssertion::StakeAssertion(
                        StakeAssertion::DelegationDeactivationEpoch {
                            value: stake.delegation.deactivation_epoch,
                            operator: IntegerOperator::Equal,
                        },
                    )),
                    // Negation
                    (StakeAccountAssertion::StakeAssertion(
                        StakeAssertion::DelegationDeactivationEpoch {
                            value: 69,
                            operator: IntegerOperator::NotEqual,
                        },
                    )),
                    // ()
                    (StakeAccountAssertion::StakeAssertion(
                        StakeAssertion::DelegationActivationEpoch {
                            value: stake.delegation.activation_epoch,
                            operator: IntegerOperator::Equal,
                        },
                    )),
                    // Negation
                    (StakeAccountAssertion::StakeAssertion(
                        StakeAssertion::DelegationActivationEpoch {
                            value: 69,
                            operator: IntegerOperator::NotEqual,
                        },
                    )),
                    // ()
                    (StakeAccountAssertion::StakeAssertion(
                        StakeAssertion::DelegationVoterPubkey {
                            value: stake.delegation.voter_pubkey,
                            operator: EquatableOperator::Equal,
                        },
                    )),
                    // Negation
                    (StakeAccountAssertion::StakeAssertion(
                        StakeAssertion::DelegationVoterPubkey {
                            value: Keypair::new().encodable_pubkey(),
                            operator: EquatableOperator::NotEqual,
                        },
                    )),
                    (StakeAccountAssertion::StakeFlags {
                        value: u8::MAX,
                        operator: IntegerOperator::DoesNotContain,
                    }),
                    (StakeAccountAssertion::StakeFlags {
                        value: u8::MIN,
                        operator: IntegerOperator::Contains,
                    }),
                    (StakeAccountAssertion::StakeFlags {
                        value: u8::MIN,
                        operator: IntegerOperator::Equal,
                    }),
                ]
                .into(),
            )
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let assertions = vec![
        // ()
        (StakeAccountAssertion::State {
            value: StakeStateType::Stake,
            operator: EquatableOperator::Equal,
        }),
        // Negation
        (StakeAccountAssertion::State {
            value: StakeStateType::Initialized,
            operator: EquatableOperator::NotEqual,
        }),
        // ()
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupCustodian {
            value: meta.lockup.custodian,
            operator: EquatableOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupCustodian {
            value: Keypair::new().encodable_pubkey(),
            operator: EquatableOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupEpoch {
            value: meta.lockup.epoch,
            operator: IntegerOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupEpoch {
            value: 69,
            operator: IntegerOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupUnixTimestamp {
            value: meta.lockup.unix_timestamp,
            operator: IntegerOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::LockupUnixTimestamp {
            value: 69,
            operator: IntegerOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedStaker {
            value: meta.authorized.staker,
            operator: EquatableOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedStaker {
            value: Keypair::new().encodable_pubkey(),
            operator: EquatableOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedWithdrawer {
            value: meta.authorized.withdrawer,
            operator: EquatableOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::AuthorizedWithdrawer {
            value: Keypair::new().encodable_pubkey(),
            operator: EquatableOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::RentExemptReserve {
            value: meta.rent_exempt_reserve,
            operator: IntegerOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::MetaAssertion(MetaAssertion::RentExemptReserve {
            value: 69,
            operator: IntegerOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::CreditsObserved {
            value: stake.credits_observed,
            operator: IntegerOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::CreditsObserved {
            value: 69,
            operator: IntegerOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationStake {
            value: stake.delegation.stake,
            operator: IntegerOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationStake {
            value: 69,
            operator: IntegerOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationDeactivationEpoch {
            value: stake.delegation.deactivation_epoch,
            operator: IntegerOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationDeactivationEpoch {
            value: 69,
            operator: IntegerOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationActivationEpoch {
            value: stake.delegation.activation_epoch,
            operator: IntegerOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationActivationEpoch {
            value: 69,
            operator: IntegerOperator::NotEqual,
        })),
        // ()
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationVoterPubkey {
            value: stake.delegation.voter_pubkey,
            operator: EquatableOperator::Equal,
        })),
        // Negation
        (StakeAccountAssertion::StakeAssertion(StakeAssertion::DelegationVoterPubkey {
            value: Keypair::new().encodable_pubkey(),
            operator: EquatableOperator::NotEqual,
        })),
        (StakeAccountAssertion::StakeFlags {
            value: u8::MAX,
            operator: IntegerOperator::DoesNotContain,
        }),
        (StakeAccountAssertion::StakeFlags {
            value: u8::MIN,
            operator: IntegerOperator::Contains,
        }),
        (StakeAccountAssertion::StakeFlags {
            value: u8::MIN,
            operator: IntegerOperator::Equal,
        }),
    ];

    // iterate through every assertion and insert failed assertion and assert failure

    for (i, _) in assertions.iter().enumerate() {
        let mut assertions = assertions.clone();
        assertions.insert(
            i,
            StakeAccountAssertion::State {
                value: StakeStateType::Initialized,
                operator: EquatableOperator::Equal,
            },
        );

        let tx: Transaction = Transaction::new_signed_with_payer(
            &[AssertStakeAccountMultiBuilder::new()
                .target_account(derived_account)
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertions(assertions.into())
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error_u8(0, 0x1900 + i as u32),
            None,
        )
        .await
        .unwrap();
    }
}
