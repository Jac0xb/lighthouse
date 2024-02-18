use crate::utils::utils::{
    build_tx, process_transaction_assert_failure, process_transaction_assert_success,
    to_transaction_error,
};
use crate::utils::CreateMintParameters;
use crate::utils::{context::TestContext, create_mint, create_user};
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::types::{
    AccountInfoFieldAssertion, ComparableOperator, EquatableOperator, TokenAccountFieldAssertion,
};
use lighthouse::{error::LighthouseError, types::Assertion};
use rust_sdk::{blackhat_program::BlackhatProgram, LighthouseProgram};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn test_drain_solana() {
    let context = &mut TestContext::new().await.unwrap();
    let mut lighthouse_program = LighthouseProgram {};
    let mut blackhat_program = BlackhatProgram {};
    let user = create_user(context).await.unwrap();

    let drainer = Keypair::new();
    let user_balance = context
        .client()
        .get_account(user.encodable_pubkey())
        .await
        .unwrap()
        .unwrap()
        .lamports;

    let drainer_ixs = blackhat_program
        .drain_solana(&user, &drainer.encodable_pubkey())
        .ixs;
    let assert_ix = lighthouse_program
        .create_assert_multi(
            &user,
            vec![Assertion::AccountInfoField(
                AccountInfoFieldAssertion::Lamports(
                    user_balance - 10_000,
                    ComparableOperator::GreaterThan,
                ),
            )],
            vec![user.encodable_pubkey()],
        )
        .ixs;

    let tx = build_tx(
        [drainer_ixs, assert_ix].concat(),
        vec![&user],
        &user.encodable_pubkey(),
        &mut context.client(),
    )
    .await
    .unwrap();

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_drain_token_account() {
    let context = &mut TestContext::new().await.unwrap();
    let mut lighthouse_program = LighthouseProgram {};
    let mut blackhat_program = BlackhatProgram {};

    let drainer = Keypair::new();
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

    let tx = blackhat_program
        .drain_token_account(&user, &drainer.encodable_pubkey(), &mint.pubkey())
        .append(lighthouse_program.create_assert(
            &user,
            user_ata,
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Amount(
                69_000,
                ComparableOperator::Equal,
            )),
            None,
        ))
        .append(lighthouse_program.create_assert(
            &user,
            user_ata,
            Assertion::TokenAccountField(TokenAccountFieldAssertion::Delegate(
                None,
                EquatableOperator::Equal,
            )),
            None,
        ))
        .to_transaction_and_sign(vec![&user], context.get_blockhash())
        .unwrap();

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(1, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();
}

// TODO: Delegate attacher
// TODO: Bitflip delegate attacher
// TODO: Bitflip solana account drainer
// TODO: Account owner attacher

// #[tokio::test]
// async fn test_bitflip_drain_token_account() {
//     let context = &mut TestContext::new().await.unwrap();
//     let mut lighthouse_program = LighthouseProgram {};
//     let mut blackhat_program = BlackhatProgram {};

//     let drainer = Keypair::new();
//     let user = create_user(context).await.unwrap();

//     let (tx, mint) = create_mint(context, &user).await.unwrap();
//     process_transaction_assert_success(context, tx)
//         .await
//         .unwrap();

//     let tx = mint_to(context, &mint.pubkey(), &user, &user.pubkey(), 69_000)
//         .await
//         .unwrap();
//     process_transaction_assert_success(context, tx)
//         .await
//         .unwrap();

//     let user_ata = get_associated_token_address(&user.pubkey(), &mint.pubkey());

//     let mut rng = thread_rng();
//     let mut bytes = [0u8; 32];
//     rng.fill_bytes(&mut bytes);

//     let tx = blackhat_program
//         .bitflip_drain_token_account(&user, &drainer.encodable_pubkey(), &mint.pubkey(), bytes)
//         .append(lighthouse_program.create_assert(
//             &user,
//             user_ata,
//             Assertion::TokenAccountField(TokenAccountField::Amount(69_000), Operator::Equal),
//             None,
//         ))
//         .to_transaction_and_sign(vec![&user], context.get_blockhash())
//         .unwrap();

//     let simulation_result = context
//         .client()
//         .simulate_transaction(VersionedTransaction::from(tx))
//         .await
//         .unwrap();

//     println!("{:?}", simulation_result);

//     panic!("Not implemented");
//     // process_transaction_assert_failure(
//     //     context,
//     //     tx,
//     //     to_transaction_error(1, LighthouseError::AssertionFailed),
//     //     None,
//     // )
//     // .await
//     // .unwrap();
// }
