use core::panic;

use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::types::{ComparableOperator, EquatableOperator, TokenAccountFieldAssertion};
use rust_sdk::{blackhat_program, LighthouseProgram, TxBuilder};
use solana_program::program_pack::Pack;
use solana_program::system_instruction::transfer;
use solana_program_test::tokio;
use solana_sdk::fee;
use solana_sdk::signer::{EncodableKeypair, Signer};
use solana_sdk::transaction::Transaction;
use spl_token::state::AccountState as TokenAccountState;

use crate::utils::context::TestContext;
use crate::utils::utils::process_transaction_assert_success;
use crate::utils::{create_mint, create_user, CreateMintParameters};

#[tokio::test]
async fn test_hijack_account_ownership() {
    let context = &mut TestContext::new().await.unwrap();
    let mut program = LighthouseProgram {};
    let mut blackhat_program = blackhat_program::BlackhatProgram {};
    let unprotected_user = create_user(context).await.unwrap();
    let bad_fee_payer = create_user(context).await.unwrap();

    // User loses control of their account to malicious actor.
    let tx = blackhat_program
        .hijack_account_ownership(unprotected_user.pubkey())
        .change_fee_payer(bad_fee_payer.pubkey())
        .to_transaction_and_sign(
            vec![&unprotected_user, &bad_fee_payer],
            context.get_blockhash(),
        )
        .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    let user_account = context
        .client()
        .get_account(unprotected_user.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user_account.owner, blackhat::ID);

    let protected_user = create_user(context).await.unwrap();
    let tx = TxBuilder {
        payer: protected_user.pubkey(),
        look_up_tables: None,
        ixs: vec![
            blackhat_program
                .hijack_account_ownership(protected_user.pubkey())
                .ix(),
            program
                .assert_account_info(
                    protected_user.pubkey(),
                    protected_user.pubkey(),
                    lighthouse::types::AccountInfoFieldAssertion::Owner(
                        system_program.pubkey(),
                        EquatableOperator::Equal,
                    ),
                    None,
                )
                .ix(),
        ],
    }
    .to_transaction_and_sign(vec![&protected_user], context.get_blockhash())
    .unwrap();

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}
