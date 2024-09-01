use crate::utils::context::TestContext;
use crate::utils::{create_mint, create_user, CreateMintParameters};
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
};
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse_sdk::errors::LighthouseError;
use lighthouse_sdk::instructions::AssertTokenAccountBuilder;
use lighthouse_sdk::types::{
    AssertionResult, EquatableOperator, IntegerOperator, TokenAccountAssertion,
};
use lighthouse_sdk::utils::parse_evaluation_payloads_from_logs;
use solana_program_test::tokio;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn encoded_message() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let random_user = Keypair::new().pubkey();
    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let builder_fn = |assertion: TokenAccountAssertion| {
        AssertTokenAccountBuilder::new()
            .target_account(user_ata)
            .log_level(lighthouse_sdk::types::LogLevel::EncodedMessage)
            .assertion(assertion)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(TokenAccountAssertion::Mint {
                value: mint.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Owner {
                value: user.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Owner {
                value: random_user,
                operator: EquatableOperator::NotEqual,
            }),
            builder_fn(TokenAccountAssertion::Amount {
                value: 69_000,
                operator: IntegerOperator::Equal,
            }),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    let result = process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    println!("Result: {:?}", result);

    // find log with "Program data: ", base64 deserialize text after that
    let metadata = result.metadata.unwrap();

    let logs = metadata
        .log_messages
        .iter()
        .filter(|log| log.contains("Program data: "))
        .collect::<Vec<&String>>();

    let expected_payloads = [
        AssertionResult::Pubkey(
            Some(mint.pubkey()),
            Some(mint.pubkey()),
            EquatableOperator::Equal as u8,
            true,
        ),
        AssertionResult::Pubkey(
            Some(user.pubkey()),
            Some(user.pubkey()),
            EquatableOperator::Equal as u8,
            true,
        ),
        AssertionResult::Pubkey(
            Some(user.pubkey()),
            Some(random_user),
            EquatableOperator::NotEqual as u8,
            true,
        ),
        AssertionResult::U64(
            Some(69_000),
            Some(69_000),
            IntegerOperator::Equal as u8,
            true,
        ),
    ];

    assert_eq!(logs.len(), 4);

    let payloads = parse_evaluation_payloads_from_logs(logs.to_vec()).unwrap();
    for (i, payload) in payloads.iter().enumerate() {
        assert_eq!(expected_payloads[i], *payload);
    }
}

#[tokio::test]
async fn encoded_message_fail_only() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let builder_fn = |assertion: TokenAccountAssertion| {
        AssertTokenAccountBuilder::new()
            .target_account(user_ata)
            .log_level(lighthouse_sdk::types::LogLevel::FailedEncodedMessage)
            .assertion(assertion)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(TokenAccountAssertion::Mint {
                value: mint.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Owner {
                value: user.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Amount {
                value: 69_000,
                operator: IntegerOperator::NotEqual,
            }),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    let result = process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(2, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // find log with "Program data: ", base64 deserialize text after that
    let metadata = result.metadata.unwrap();

    let logs = metadata
        .log_messages
        .iter()
        .filter(|log| log.contains("Program data: "))
        .collect::<Vec<&String>>();

    let expected_payloads: [AssertionResult; 1] = [AssertionResult::U64(
        Some(69_000),
        Some(69_000),
        IntegerOperator::NotEqual as u8,
        false,
    )];

    assert_eq!(logs.len(), 1);

    let payloads = parse_evaluation_payloads_from_logs(logs.to_vec()).unwrap();
    for (i, payload) in payloads.iter().enumerate() {
        assert_eq!(expected_payloads[i], *payload);
    }
}

#[tokio::test]
async fn encoded_noop() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let builder_fn = |assertion: TokenAccountAssertion| {
        AssertTokenAccountBuilder::new()
            .target_account(user_ata)
            .log_level(lighthouse_sdk::types::LogLevel::EncodedNoop)
            .assertion(assertion)
            .add_remaining_account(AccountMeta {
                pubkey: spl_noop::id(),
                is_signer: false,
                is_writable: false,
            })
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(TokenAccountAssertion::Mint {
                value: mint.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Owner {
                value: user.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Amount {
                value: 69_000,
                operator: IntegerOperator::Equal,
            }),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    let result = process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Waiting for BankClient fixes (https://github.com/anza-xyz/agave/pull/1504) to deserialize actual payload

    println!("Result: {:?}", result);

    let mut noop_count = 0;

    for log in result.metadata.unwrap().log_messages.iter() {
        if log.contains("Program noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV success") {
            noop_count += 1;
        }
    }

    assert_eq!(noop_count, 3);
}

#[tokio::test]
async fn encoded_noop_fail_only() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let builder_fn = |assertion: TokenAccountAssertion| {
        AssertTokenAccountBuilder::new()
            .target_account(user_ata)
            .log_level(lighthouse_sdk::types::LogLevel::FailedEncodedNoop)
            .assertion(assertion)
            .add_remaining_account(AccountMeta {
                pubkey: spl_noop::id(),
                is_signer: false,
                is_writable: false,
            })
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(TokenAccountAssertion::Mint {
                value: mint.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Owner {
                value: user.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Amount {
                value: 69_000,
                operator: IntegerOperator::NotEqual,
            }),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    let result = process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(2, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Waiting for BankClient fixes (https://github.com/anza-xyz/agave/pull/1504) to deserialize actual payload

    println!("Result: {:?}", result);

    let mut noop_count = 0;

    for log in result.metadata.unwrap().log_messages.iter() {
        if log.contains("Program noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV success") {
            noop_count += 1;
        }
    }

    assert_eq!(noop_count, 1);
}

#[tokio::test]
async fn plaintext_message() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let random_user = Keypair::new().pubkey();
    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let builder_fn = |assertion: TokenAccountAssertion| {
        AssertTokenAccountBuilder::new()
            .target_account(user_ata)
            .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
            .assertion(assertion)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(TokenAccountAssertion::Mint {
                value: mint.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Owner {
                value: random_user,
                operator: EquatableOperator::NotEqual,
            }),
            builder_fn(TokenAccountAssertion::Amount {
                value: 69_000,
                operator: IntegerOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::DelegatedAmount {
                value: 10,
                operator: IntegerOperator::LessThan,
            }),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    let result = process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    println!("Result: {:?}", result);

    // find log with "Program data: ", base64 deserialize text after that
    let metadata = result.metadata.unwrap();

    let logs = metadata.log_messages.iter().collect::<Vec<&String>>();

    let expected_logs = [
        "Program log: Result: ".to_string(),
        format!("Program log: {:?}", mint.pubkey()).to_string(),
        "Program log: ==".to_string(),
        format!("Program log: {:?}", mint.pubkey()).to_string(),
        "Program log: Result: ".to_string(),
        format!("Program log: {:?}", user.pubkey()).to_string(),
        "Program log: !=".to_string(),
        format!("Program log: {:?}", random_user).to_string(),
        "Program log: Result (Passed): Some(69000) == Some(69000)".to_string(),
        "Program log: Result (Passed): Some(0) < Some(10)".to_string(),
    ];

    // parse through logs looking for sequential logs
    let mut log_pos = 0;

    for expected_log in expected_logs.iter() {
        println!("Expected: {}", expected_log);

        for i in log_pos..logs.len() {
            println!(
                "Expected: {}, at {} was true {}",
                expected_log,
                i,
                logs[i].contains(expected_log)
            );

            if logs[i].contains(expected_log) {
                log_pos = i + 1;
                break;
            }

            if i == logs.len() - 1 {
                panic!("Logs not found for: {}", expected_log);
            }
        }
    }
}

#[tokio::test]
async fn plaintext_message_fail_only() {
    let context = &mut TestContext::new().await.unwrap();
    let user = create_user(context).await.unwrap();

    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(user.pubkey())),
            freeze_authority: None,
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let random_user = Keypair::new().pubkey();
    let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());
    let builder_fn = |assertion: TokenAccountAssertion| {
        AssertTokenAccountBuilder::new()
            .target_account(user_ata)
            .log_level(lighthouse_sdk::types::LogLevel::FailedPlaintextMessage)
            .assertion(assertion)
            .instruction()
    };

    let tx = Transaction::new_signed_with_payer(
        &[
            builder_fn(TokenAccountAssertion::Mint {
                value: mint.pubkey(),
                operator: EquatableOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::Owner {
                value: random_user,
                operator: EquatableOperator::NotEqual,
            }),
            builder_fn(TokenAccountAssertion::Amount {
                value: 69_000,
                operator: IntegerOperator::Equal,
            }),
            builder_fn(TokenAccountAssertion::DelegatedAmount {
                value: 10,
                operator: IntegerOperator::GreaterThan,
            }),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    let result = process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(3, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    println!("Result: {:?}", result);

    // find log with "Program data: ", base64 deserialize text after that
    let metadata = result.metadata.unwrap();

    let logs = metadata.log_messages.iter().collect::<Vec<&String>>();

    // make sure there is only one log with 'Result'
    let result_logs = logs
        .iter()
        .filter(|log| log.contains("Program log: Result"))
        .collect::<Vec<&&String>>();

    assert_eq!(result_logs.len(), 1);

    let expected_logs = ["Program log: Result (Failed): Some(0) > Some(10)".to_string()];

    // parse through logs looking for sequential logs
    let mut log_pos = 0;

    for expected_log in expected_logs.iter() {
        println!("Expected: {}", expected_log);

        for i in log_pos..logs.len() {
            println!(
                "Expected: {}, at {} was true {}",
                expected_log,
                i,
                logs[i].contains(expected_log)
            );

            if logs[i].contains(expected_log) {
                log_pos = i + 1;
                break;
            }

            if i == logs.len() - 1 {
                panic!("Logs not found for: {}", expected_log);
            }
        }
    }
}
