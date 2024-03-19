use crate::utils::context::TestContext;
use crate::utils::{create_test_account, create_user};
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error_u8,
};
use lighthouse_sdk::cpi::AssertAccountInfoMultiBuilder;
use lighthouse_sdk::instructions::AssertAccountInfoBuilder;
use lighthouse_sdk::types::{
    AccountInfoAssertion, EquatableOperator, IntegerOperator, KnownProgram,
};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;
use solana_sdk::{keccak, system_program};

#[tokio::test]
async fn simple() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();

    let test_account = create_test_account(ctx, &user, false).await.unwrap();
    let test_balance = ctx
        .client()
        .get_balance(test_account.encodable_pubkey())
        .await
        .unwrap();
    let rent_epoch = ctx
        .client()
        .get_account(user.encodable_pubkey())
        .await
        .unwrap()
        .unwrap()
        .rent_epoch;

    let user_balance = ctx
        .client()
        .get_balance(user.encodable_pubkey())
        .await
        .unwrap();

    // Test Owner

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoMultiBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
            .assertions(vec![
                AccountInfoAssertion::Owner {
                    value: system_program::ID,
                    operator: EquatableOperator::Equal,
                },
                // Test KnownOwner
                AccountInfoAssertion::KnownOwner {
                    value: KnownProgram::System,
                    operator: EquatableOperator::Equal,
                },
                // Test Lamports
                AccountInfoAssertion::Lamports {
                    value: user_balance - 5000,
                    operator: IntegerOperator::Equal,
                },
                AccountInfoAssertion::DataLength {
                    value: 0,
                    operator: IntegerOperator::Equal,
                },
                AccountInfoAssertion::Executable {
                    value: true,
                    operator: EquatableOperator::NotEqual,
                },
                AccountInfoAssertion::Executable {
                    value: false,
                    operator: EquatableOperator::Equal,
                },
                AccountInfoAssertion::Executable {
                    value: true,
                    operator: EquatableOperator::NotEqual,
                },
                AccountInfoAssertion::RentEpoch {
                    value: rent_epoch,
                    operator: IntegerOperator::Equal,
                },
            ])
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let assertions = vec![
        AccountInfoAssertion::Owner {
            value: system_program::ID,
            operator: EquatableOperator::Equal,
        },
        AccountInfoAssertion::KnownOwner {
            value: KnownProgram::System,
            operator: EquatableOperator::Equal,
        },
        AccountInfoAssertion::Lamports {
            value: user_balance - 100_000,
            operator: IntegerOperator::GreaterThanOrEqual,
        },
        AccountInfoAssertion::DataLength {
            value: 0,
            operator: IntegerOperator::Equal,
        },
        AccountInfoAssertion::Executable {
            value: true,
            operator: EquatableOperator::NotEqual,
        },
        AccountInfoAssertion::Executable {
            value: false,
            operator: EquatableOperator::Equal,
        },
        AccountInfoAssertion::Executable {
            value: true,
            operator: EquatableOperator::NotEqual,
        },
        AccountInfoAssertion::RentEpoch {
            value: rent_epoch,
            operator: IntegerOperator::Equal,
        },
    ];

    // insert bad instruction and assert failure
    for i in 0..assertions.len() {
        let mut assertions = assertions.clone();
        assertions[i] = AccountInfoAssertion::Owner {
            value: system_program::ID,
            operator: EquatableOperator::NotEqual,
        };

        let tx = Transaction::new_signed_with_payer(
            &[AssertAccountInfoMultiBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertions(assertions)
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            ctx.get_blockhash().await,
        );

        process_transaction_assert_failure(
            ctx,
            tx,
            to_transaction_error_u8(0, 0x1900 + i as u32),
            None,
        )
        .await
        .unwrap();
    }

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoMultiBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_sdk::types::LogLevel::Silent)
            .assertions(vec![
                AccountInfoAssertion::Lamports {
                    value: test_balance / 2,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
                AccountInfoAssertion::Lamports {
                    value: test_balance * 2,
                    operator: IntegerOperator::LessThanOrEqual,
                },
                AccountInfoAssertion::Lamports {
                    value: test_balance,
                    operator: IntegerOperator::Equal,
                },
                AccountInfoAssertion::Lamports {
                    value: 0,
                    operator: IntegerOperator::NotEqual,
                },
            ])
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    let test_account_len = ctx
        .client()
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .unwrap()
        .data
        .len() as u64;

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: 0,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: 128,
                    operator: IntegerOperator::NotEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: test_account_len,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(test_account.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::DataLength {
                    value: test_account_len + 1,
                    operator: IntegerOperator::LessThan,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Test IsSigner

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsSigner {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsSigner {
                    value: false,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(lighthouse_sdk::ID)
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsSigner {
                    value: true,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Test IsWritable
    let tx = Transaction::new_signed_with_payer(
        &[
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsWritable {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(user.encodable_pubkey())
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsWritable {
                    value: false,
                    operator: EquatableOperator::NotEqual,
                })
                .instruction(),
            AssertAccountInfoBuilder::new()
                .target_account(lighthouse_sdk::ID)
                .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
                .assertion(AccountInfoAssertion::IsWritable {
                    value: false,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();
}

#[tokio::test]
async fn verify_hash() {
    let ctx = &mut TestContext::new().await.unwrap();
    let user = create_user(ctx).await.unwrap();
    let test_account = create_test_account(ctx, &user, false).await.unwrap();
    let test_account_data = ctx
        .client()
        .get_account(test_account.encodable_pubkey())
        .await
        .unwrap()
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoMultiBuilder::new()
            .target_account(test_account.encodable_pubkey())
            .log_level(lighthouse_sdk::types::LogLevel::Silent)
            .assertions(vec![
                AccountInfoAssertion::VerifyDatahash {
                    expected_hash: keccak::hashv(&[&test_account_data.data]).0,
                    start: None,
                    length: None,
                },
                // (none, Some)
                AccountInfoAssertion::VerifyDatahash {
                    expected_hash: keccak::hashv(&[&test_account_data.data[0..128]]).0,
                    start: None,
                    length: Some(128),
                },
                // (Some, Some)
                AccountInfoAssertion::VerifyDatahash {
                    expected_hash: keccak::hashv(&[&test_account_data.data[128..256]]).0,
                    start: Some(128),
                    length: Some(128),
                },
                // (Some, None)
                AccountInfoAssertion::VerifyDatahash {
                    expected_hash: keccak::hashv(&[&test_account_data.data[128..]]).0,
                    start: Some(128),
                    length: None,
                },
            ])
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();

    // Empty account
    let tx = Transaction::new_signed_with_payer(
        &[AssertAccountInfoMultiBuilder::new()
            .target_account(user.encodable_pubkey())
            .log_level(lighthouse_sdk::types::LogLevel::Silent)
            .assertions(vec![
                AccountInfoAssertion::VerifyDatahash {
                    expected_hash: keccak::hashv(&[&[]]).0,
                    start: None,
                    length: None,
                },
                AccountInfoAssertion::VerifyDatahash {
                    expected_hash: keccak::hashv(&[&[]]).0,
                    start: Some(0),
                    length: Some(0),
                },
            ])
            .instruction()],
        Some(&user.encodable_pubkey()),
        &[&user],
        ctx.get_blockhash().await,
    );

    process_transaction_assert_success(ctx, tx).await.unwrap();
}
