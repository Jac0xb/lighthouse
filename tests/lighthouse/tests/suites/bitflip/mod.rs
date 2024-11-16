use crate::utils::blackhat_program::BlackhatProgram;
use crate::utils::process_transaction_assert_failure;
use crate::utils::process_transaction_assert_success;
use crate::utils::to_transaction_error;
use crate::utils::CreateMintParameters;
use crate::utils::{context::TestContext, create_mint, create_user};
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse_sdk::errors::LighthouseError;
use lighthouse_sdk::instructions::AssertTokenAccountBuilder;
use lighthouse_sdk::types::{IntegerOperator, TokenAccountAssertion};
use rand::{thread_rng, RngCore};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

// TODO: Bitflip delegate attacher
// TODO: Bitflip solana account drainer

#[tokio::test]
async fn test_bitflip_drain_token_account() {
    let context = &mut TestContext::new().await.unwrap();

    let user = create_user(context).await.unwrap();
    let drainer = create_user(context).await.unwrap();

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

    let mut rng = thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_associated_token_account::instruction::create_associated_token_account(
                &user.pubkey(),
                &drainer.pubkey(),
                &mint.pubkey(),
                &spl_token::id(),
            ),
            BlackhatProgram::bitflip_drain_token_account(
                &user.encodable_pubkey(),
                &drainer.encodable_pubkey(),
                &mint.pubkey(),
                bytes,
            ),
            AssertTokenAccountBuilder::new()
                .target_account(user_ata)
                .assertion(TokenAccountAssertion::Amount {
                    value: 69_000,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
        ],
        Some(&user.pubkey()),
        &[&user],
        context.get_blockhash().await,
    );

    let simulation_result = context
        .client()
        .simulate_transaction(tx.clone())
        .await
        .unwrap();

    // Assert successful simulation (Bitflip not activated).
    assert!(simulation_result.result.unwrap().is_ok());

    // Activating bitflip.
    let activation_tx = Transaction::new_signed_with_payer(
        &[BlackhatProgram::enable_bitflip(
            &drainer.encodable_pubkey(),
            bytes,
        )],
        Some(&drainer.pubkey()),
        &[&drainer],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, activation_tx)
        .await
        .unwrap();

    // Lighthouse should fail because the drainer has taken the tokens.
    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(2, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}
