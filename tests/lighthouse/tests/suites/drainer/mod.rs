use crate::utils::CreateMintParameters;
use crate::utils::{
    build_tx, process_transaction_assert_failure, process_transaction_assert_success,
    to_transaction_error,
};
use crate::utils::{context::TestContext, create_mint, create_user};
use anchor_spl::associated_token::get_associated_token_address;
use lighthouse::types::{
    AccountInfoAssertion, ComparableOperator, EquatableOperator, TokenAccountAssertion,
};
use lighthouse::{error::LighthouseError, types::Assertion};
use lighthouse_sdk::{blackhat_program::BlackhatProgram, LighthouseProgram};
use solana_program_test::tokio;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::{signature::Keypair, signer::Signer};

// TODO: Delegate attacher
// TODO: Bitflip delegate attacher
// TODO: Bitflip solana account drainer
// TODO: Account owner attacher

// #[tokio::test]
// async fn test_bitflip_drain_token_account() {
//     let context = &mut TestContext::new().await.unwrap();
//     let mut lighthouse_program = LighthouseProgram {};
//     let blackhat_program = BlackhatProgram {};

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
//             Assertion::TokenAccount(TokenAccount::Amount(69_000), Operator::Equal),
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
