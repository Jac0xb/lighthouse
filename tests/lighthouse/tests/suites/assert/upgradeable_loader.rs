use crate::utils::context::TestContext;
use crate::utils::{create_user_with_balance, set_account_from_refs, to_transaction_error};
use crate::utils::{process_transaction_assert_failure, process_transaction_assert_success};
use lighthouse_client::errors::LighthouseError;
use lighthouse_client::instructions::AssertUpgradeableLoaderAccountBuilder;
use lighthouse_client::types::{
    EquatableOperator, IntegerOperator, UpgradableBufferAssertion, UpgradeableLoaderStateAssertion,
    UpgradeableLoaderStateType, UpgradeableProgramAssertion, UpgradeableProgramDataAssertion,
};
use solana_program_test::tokio;
use solana_sdk::bpf_loader_upgradeable::{self, UpgradeableLoaderState};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;

///
/// Tests all data types using the `StakeAccount` assertion.
///
#[tokio::test]
async fn test_upgradeable_loader() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let upgrade_authority = Keypair::new().encodable_pubkey();
    let programdata_address = Keypair::new().encodable_pubkey();
    let program_pubkey = Keypair::new().encodable_pubkey();
    let program = UpgradeableLoaderState::Program {
        programdata_address,
    };
    let serialized_program = bincode::serialize(&program).unwrap();
    set_account_from_refs(
        context,
        &program_pubkey,
        &serialized_program,
        &bpf_loader_upgradeable::ID,
    )
    .await;

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

    let programdata_state = UpgradeableLoaderState::ProgramData {
        slot: u64::MAX,
        upgrade_authority_address: Some(upgrade_authority),
    };
    let serialized_programdata = bincode::serialize(&programdata_state).unwrap();
    set_account_from_refs(
        context,
        &programdata_address,
        &serialized_programdata,
        &bpf_loader_upgradeable::ID,
    )
    .await;

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
                        value: Some(upgrade_authority),
                        operator: EquatableOperator::Equal,
                    },
                ))
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                    UpgradeableProgramDataAssertion::Slot {
                        value: u64::MAX,
                        operator: IntegerOperator::Equal,
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

#[tokio::test]
async fn simple() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    // Assert an uninitialized program account

    let program_pubkey = Keypair::new().encodable_pubkey();
    let program = UpgradeableLoaderState::Uninitialized;
    let serialized_program = bincode::serialize(&program).unwrap();
    set_account_from_refs(
        context,
        &program_pubkey,
        &serialized_program,
        &bpf_loader_upgradeable::ID,
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::State {
                value: UpgradeableLoaderStateType::Uninitialized,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let failed_ixs = [
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                UpgradeableProgramDataAssertion::Slot {
                    value: u64::MAX,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::Buffer(
                UpgradableBufferAssertion::Authority {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::Program(
                UpgradeableProgramAssertion::ProgramDataAddress {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
    ];

    for ix in failed_ixs.iter() {
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

    // Assert an initialized buffer account

    let authority_address = Keypair::new().encodable_pubkey();
    let program = UpgradeableLoaderState::Buffer {
        authority_address: Some(authority_address),
    };
    let serialized_program = bincode::serialize(&program).unwrap();
    set_account_from_refs(
        context,
        &program_pubkey,
        &serialized_program,
        &bpf_loader_upgradeable::ID,
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(program_pubkey)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::State {
                    value: UpgradeableLoaderStateType::Buffer,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(program_pubkey)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::Buffer(
                    UpgradableBufferAssertion::Authority {
                        value: Some(authority_address),
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

    let failed_ixs = [
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                UpgradeableProgramDataAssertion::Slot {
                    value: u64::MAX,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::Program(
                UpgradeableProgramAssertion::ProgramDataAddress {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
    ];

    for ix in failed_ixs.iter() {
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

    // Assert an initialized program account

    let programdata_address = Keypair::new().encodable_pubkey();
    let program = UpgradeableLoaderState::Program {
        programdata_address,
    };
    let serialized_program = bincode::serialize(&program).unwrap();
    set_account_from_refs(
        context,
        &program_pubkey,
        &serialized_program,
        &bpf_loader_upgradeable::ID,
    )
    .await;

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

    let failed_ixs = [
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                UpgradeableProgramDataAssertion::Slot {
                    value: u64::MAX,
                    operator: IntegerOperator::Equal,
                },
            ))
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::Buffer(
                UpgradableBufferAssertion::Authority {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
    ];

    for ix in failed_ixs.iter() {
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

    // Assert an initialized programdata account

    let upgrade_authority = Keypair::new().encodable_pubkey();
    let programdata_state = UpgradeableLoaderState::ProgramData {
        slot: u64::MAX,
        upgrade_authority_address: Some(upgrade_authority),
    };
    let serialized_programdata = bincode::serialize(&programdata_state).unwrap();

    set_account_from_refs(
        context,
        &programdata_address,
        &serialized_programdata,
        &bpf_loader_upgradeable::ID,
    )
    .await;

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
                        value: Some(upgrade_authority),
                        operator: EquatableOperator::Equal,
                    },
                ))
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                    UpgradeableProgramDataAssertion::Slot {
                        value: u64::MAX,
                        operator: IntegerOperator::Equal,
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

    let failed_ixs = [
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::Buffer(
                UpgradableBufferAssertion::Authority {
                    value: None,
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
        AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(program_pubkey)
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::Program(
                UpgradeableProgramAssertion::ProgramDataAddress {
                    value: Keypair::new().encodable_pubkey(),
                    operator: EquatableOperator::Equal,
                },
            ))
            .instruction(),
    ];

    for ix in failed_ixs.iter() {
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

    // None authority test

    let programdata_state = UpgradeableLoaderState::ProgramData {
        slot: u64::MAX,
        upgrade_authority_address: None,
    };
    let serialized_programdata = bincode::serialize(&programdata_state).unwrap();

    set_account_from_refs(
        context,
        &programdata_address,
        &serialized_programdata,
        &bpf_loader_upgradeable::ID,
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                    UpgradeableProgramDataAssertion::UpgradeAuthority {
                        value: None,
                        operator: EquatableOperator::Equal,
                    },
                ))
                .instruction(),
            AssertUpgradeableLoaderAccountBuilder::new()
                .target_account(programdata_address)
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(UpgradeableLoaderStateAssertion::ProgramData(
                    UpgradeableProgramDataAssertion::UpgradeAuthority {
                        value: Some(Keypair::new().encodable_pubkey()),
                        operator: EquatableOperator::NotEqual,
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

#[tokio::test]
async fn not_owned_by_loader_program() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user_with_balance(context, 10e9 as u64)
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertUpgradeableLoaderAccountBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(UpgradeableLoaderStateAssertion::State {
                value: UpgradeableLoaderStateType::Buffer,
                operator: EquatableOperator::Equal,
            })
            .instruction()],
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
