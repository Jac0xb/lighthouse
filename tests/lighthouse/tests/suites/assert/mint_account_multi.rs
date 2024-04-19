use crate::utils::context::TestContext;
use crate::utils::{create_mint, create_user, to_transaction_error_u8, CreateMintParameters};
use crate::utils::{process_transaction_assert_failure, process_transaction_assert_success};
use lighthaus_sdk::cpi::AssertMintAccountMultiBuilder;
use lighthaus_sdk::instructions::AssertMintAccountBuilder;
use lighthaus_sdk::types::{EquatableOperator, IntegerOperator, LogLevel, MintAccountAssertion};
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn simple() {
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

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::MintAuthority {
                    value: Some(user.pubkey()),
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::Supply {
                    value: 69_000,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::Decimals {
                    value: 9,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::IsInitialized {
                    value: true,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertMintAccountBuilder::new()
                .target_account(mint.encodable_pubkey())
                .log_level(LogLevel::Silent)
                .assertion(MintAccountAssertion::FreezeAuthority {
                    value: None,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Mint with freeze authority

    let freezer = Keypair::new();
    let mint_authority = Keypair::new();
    let (tx, mint) = create_mint(
        context,
        &user,
        CreateMintParameters {
            token_program: spl_token::id(),
            mint_authority: Some(Some(mint_authority.encodable_pubkey())),
            freeze_authority: Some(freezer.encodable_pubkey()),
            mint_to: Some((user.pubkey(), 69_000)),
            decimals: 9,
        },
    )
    .await
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertMintAccountMultiBuilder::new()
            .target_account(mint.encodable_pubkey())
            .assertions(vec![
                MintAccountAssertion::FreezeAuthority {
                    value: Some(freezer.pubkey()),
                    operator: EquatableOperator::Equal,
                },
                MintAccountAssertion::FreezeAuthority {
                    value: Some(user.pubkey()),
                    operator: EquatableOperator::NotEqual,
                },
                MintAccountAssertion::Supply {
                    value: 69_000,
                    operator: IntegerOperator::Equal,
                },
                MintAccountAssertion::Decimals {
                    value: 9,
                    operator: IntegerOperator::Equal,
                },
                MintAccountAssertion::IsInitialized {
                    value: true,
                    operator: EquatableOperator::Equal,
                },
                MintAccountAssertion::MintAuthority {
                    value: Some(mint_authority.pubkey()),
                    operator: EquatableOperator::Equal,
                },
            ])
            .instruction()],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let assertions = vec![
        MintAccountAssertion::FreezeAuthority {
            value: Some(freezer.pubkey()),
            operator: EquatableOperator::Equal,
        },
        MintAccountAssertion::FreezeAuthority {
            value: Some(user.pubkey()),
            operator: EquatableOperator::NotEqual,
        },
        MintAccountAssertion::Supply {
            value: 69_000,
            operator: IntegerOperator::Equal,
        },
        MintAccountAssertion::Decimals {
            value: 9,
            operator: IntegerOperator::Equal,
        },
        MintAccountAssertion::IsInitialized {
            value: true,
            operator: EquatableOperator::Equal,
        },
        MintAccountAssertion::MintAuthority {
            value: Some(mint_authority.pubkey()),
            operator: EquatableOperator::Equal,
        },
    ];

    // insert bad assertion at each index and assert failure

    for i in 0..assertions.len() {
        let mut bad_assertions = assertions.clone();
        bad_assertions[i] = MintAccountAssertion::Supply {
            value: 69_001,
            operator: IntegerOperator::Equal,
        };

        let tx = Transaction::new_signed_with_payer(
            &[AssertMintAccountMultiBuilder::new()
                .target_account(mint.encodable_pubkey())
                .assertions(bad_assertions)
                .instruction()],
            Some(&user.pubkey()),
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
