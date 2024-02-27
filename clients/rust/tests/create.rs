// #![cfg(feature = "test-sbf")]

// use acme_counter_client::{accounts::Counter, instructions::CreateBuilder};
// use borsh::BorshDeserialize;
// use solana_program_test::{tokio, ProgramTest};
// use solana_sdk::{signature::Signer, transaction::Transaction};

// #[tokio::test]
// async fn create() {
//     let mut context = ProgramTest::new("acme_counter", acme_counter_client::ID, None)
//         .start_with_context()
//         .await;

//     // Given a PDA derived from the payer's public key.

//     let address = Counter::find_pda(&context.payer.pubkey()).0;

//     let ix = CreateBuilder::new()
//         .counter(address)
//         .authority(context.payer.pubkey())
//         .payer(context.payer.pubkey())
//         .instruction();

//     // When we create a new counter.

//     let tx = Transaction::new_signed_with_payer(
//         &[ix],
//         Some(&context.payer.pubkey()),
//         &[&context.payer],
//         context.last_blockhash,
//     );
//     context.banks_client.process_transaction(tx).await.unwrap();

//     // Then an account was created with the correct data.

//     let account = context.banks_client.get_account(address).await.unwrap();

//     assert!(account.is_some());

//     let account = account.unwrap();
//     assert_eq!(account.data.len(), Counter::LEN);

//     let mut account_data = account.data.as_ref();
//     let counter = Counter::deserialize(&mut account_data).unwrap();
//     assert_eq!(counter.authority, context.payer.pubkey());
//     assert_eq!(counter.value, 0);
// }
