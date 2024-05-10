pub mod memory_close {
    use crate::utils::{context::TestContext, create_user};
    use crate::utils::{
        process_transaction_assert_failure, process_transaction_assert_success,
        to_transaction_error,
    };
    use lighthouse_sdk::errors::LighthouseError;
    use lighthouse_sdk::instructions::{MemoryCloseBuilder, MemoryWriteBuilder};
    use lighthouse_sdk::types::{AccountInfoField, WriteType};
    use lighthouse_sdk::{find_memory_pda, find_memory_pda_bump_iterate};
    use solana_program_test::tokio;
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::EncodableKeypair;
    use solana_sdk::transaction::Transaction;

    //
    //  Test memory close with closed memory account
    //
    #[tokio::test]
    async fn memory_already_closed() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[MemoryCloseBuilder::new()
                .payer(user.encodable_pubkey())
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_bump(memory_bump)
                .memory_id(0)
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[
                MemoryWriteBuilder::new()
                    .payer(user.encodable_pubkey())
                    .source_account(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_id(0)
                    .write_offset(0u8.into())
                    .memory_bump(memory_bump)
                    .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                    .instruction(),
                MemoryCloseBuilder::new()
                    .payer(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_bump(memory_bump)
                    .memory_id(0)
                    .instruction(),
                MemoryCloseBuilder::new()
                    .payer(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_bump(memory_bump)
                    .memory_id(0)
                    .instruction(),
            ],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(2, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    //
    //  Test bad memory id and memory bump
    //
    #[tokio::test]
    async fn memory_bad_seeds() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();
        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[
                MemoryWriteBuilder::new()
                    .payer(user.encodable_pubkey())
                    .source_account(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_id(0)
                    .write_offset(0u8.into())
                    .memory_bump(memory_bump)
                    .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                    .instruction(),
                MemoryCloseBuilder::new()
                    .payer(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_bump(memory_bump)
                    .memory_id(8)
                    .instruction(),
            ],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(1, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();

        let (_, next_memory_bump) =
            find_memory_pda_bump_iterate(user.encodable_pubkey(), 0, 0, Some(memory_bump - 1))
                .unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[
                MemoryWriteBuilder::new()
                    .payer(user.encodable_pubkey())
                    .source_account(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_id(0)
                    .write_offset(0u8.into())
                    .memory_bump(memory_bump)
                    .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                    .instruction(),
                MemoryCloseBuilder::new()
                    .payer(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_bump(next_memory_bump)
                    .memory_id(0)
                    .instruction(),
            ],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(1, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    //
    //  Test wrong program id
    //
    #[tokio::test]
    async fn wrong_program_id() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();
        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[
                MemoryWriteBuilder::new()
                    .payer(user.encodable_pubkey())
                    .source_account(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_id(0)
                    .write_offset(0u8.into())
                    .memory_bump(memory_bump)
                    .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                    .instruction(),
                MemoryCloseBuilder::new()
                    .payer(user.encodable_pubkey())
                    .program_id(Keypair::new().encodable_pubkey())
                    .memory(memory)
                    .memory_bump(memory_bump)
                    .memory_id(0)
                    .instruction(),
            ],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(1, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    //
    //  Test unauthorized memory close (payer is not the owner of the memory)
    //
    #[tokio::test]
    async fn unauthorized_memory_close() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();
        let bad_user = create_user(context).await.unwrap();
        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[
                MemoryWriteBuilder::new()
                    .payer(user.encodable_pubkey())
                    .source_account(user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_id(0)
                    .write_offset(0u8.into())
                    .memory_bump(memory_bump)
                    .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                    .instruction(),
                MemoryCloseBuilder::new()
                    .payer(bad_user.encodable_pubkey())
                    .program_id(lighthouse_sdk::ID)
                    .memory(memory)
                    .memory_bump(memory_bump)
                    .memory_id(0)
                    .instruction(),
            ],
            Some(&bad_user.encodable_pubkey()),
            &[&user, &bad_user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(1, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    //
    //  Test unauthorized memory close (payer is not the owner of the memory)
    //
    #[tokio::test]
    async fn unwritable_memory() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();
        let bad_user = create_user(context).await.unwrap();
        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_id(0)
                .write_offset(0u8.into())
                .memory_bump(memory_bump)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_success(context, tx)
            .await
            .unwrap();

        let mut ix = MemoryCloseBuilder::new()
            .payer(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(0)
            .instruction();

        // find memory account and set writable to false
        for account in ix.accounts.iter_mut() {
            if account.pubkey == memory {
                account.is_writable = false;
            }
        }

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();

        let mut ix = MemoryCloseBuilder::new()
            .payer(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(0)
            .instruction();

        // find payer account and set writable to false
        for account in ix.accounts.iter_mut() {
            if account.pubkey == user.encodable_pubkey() {
                account.is_writable = false;
            }
        }

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bad_user.encodable_pubkey()),
            &[&user, &bad_user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();

        let mut ix = MemoryCloseBuilder::new()
            .payer(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(0)
            .instruction();

        // find payer account and set writable to false
        for account in ix.accounts.iter_mut() {
            if account.pubkey == user.encodable_pubkey() {
                account.is_signer = false;
            }
        }

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bad_user.encodable_pubkey()),
            &[&bad_user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();

        let mut ix = MemoryCloseBuilder::new()
            .payer(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_bump(memory_bump)
            .memory_id(0)
            .instruction();

        // find payer account and set writable and signer to false
        for account in ix.accounts.iter_mut() {
            if account.pubkey == user.encodable_pubkey() {
                account.is_signer = false;
                account.is_writable = false;
            }
        }

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bad_user.encodable_pubkey()),
            &[&bad_user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }
}

pub mod memory_write {
    use crate::utils::{context::TestContext, create_user};
    use crate::utils::{
        create_test_account, process_transaction_assert_failure, to_transaction_error,
    };
    use lighthouse_sdk::errors::LighthouseError;
    use lighthouse_sdk::instructions::MemoryWriteBuilder;
    use lighthouse_sdk::types::{AccountInfoField, WriteType};
    use lighthouse_sdk::{find_memory_pda, find_memory_pda_bump_iterate};
    use solana_program_test::tokio;
    use solana_sdk::instruction::InstructionError;
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::EncodableKeypair;
    use solana_sdk::transaction::{Transaction, TransactionError};

    //
    //  Test bad memory id and memory bump
    //
    #[tokio::test]
    async fn memory_bad_seeds() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let (_, bad_id_memory_bump) = find_memory_pda(user.encodable_pubkey(), 8);

        let (_, next_memory_bump) =
            find_memory_pda_bump_iterate(user.encodable_pubkey(), 0, 0, Some(memory_bump - 1))
                .unwrap();

        // Wrong memory id
        let tx = Transaction::new_signed_with_payer(
            &[MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_id(8)
                .write_offset(0u8.into())
                .memory_bump(bad_id_memory_bump)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();

        // Wrong memory bump
        let tx = Transaction::new_signed_with_payer(
            &[MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_id(0)
                .write_offset(0u8.into())
                .memory_bump(next_memory_bump)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn wrong_program_id() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(Keypair::new().encodable_pubkey())
                .memory(memory)
                .memory_id(0)
                .write_offset(0u8.into())
                .memory_bump(memory_bump)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn unauthorized_memory_write() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();
        let bad_user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[MemoryWriteBuilder::new()
                .payer(bad_user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_id(0)
                .write_offset(0u8.into())
                .memory_bump(memory_bump)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .instruction()],
            Some(&bad_user.encodable_pubkey()),
            &[&bad_user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn write_offset_too_large() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let tx = Transaction::new_signed_with_payer(
            &[MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(user.encodable_pubkey())
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_id(0)
                .write_offset(u16::MAX.into())
                .memory_bump(memory_bump)
                .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            TransactionError::InstructionError(0, InstructionError::InvalidRealloc),
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn memory_account_not_writable() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let mut ix = MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_id(0)
            .write_offset(0u8.into())
            .memory_bump(memory_bump)
            .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
            .instruction();

        // find memory account and set writable to false
        for account in ix.accounts.iter_mut() {
            if account.pubkey == memory {
                account.is_writable = false;
            }
        }

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn payer_not_writable() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();
        let bad_user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let mut ix = MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_id(0)
            .write_offset(0u8.into())
            .memory_bump(memory_bump)
            .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
            .instruction();

        // find payer account and set writable to false
        for account in ix.accounts.iter_mut() {
            if account.pubkey == user.encodable_pubkey() {
                account.is_writable = false;
                account.is_signer = true;
            }
        }

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bad_user.encodable_pubkey()),
            &[&bad_user, &user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn payer_not_signer() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();
        let bad_user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let mut ix = MemoryWriteBuilder::new()
            .payer(user.encodable_pubkey())
            .source_account(user.encodable_pubkey())
            .program_id(lighthouse_sdk::ID)
            .memory(memory)
            .memory_id(0)
            .write_offset(0u8.into())
            .memory_bump(memory_bump)
            .write_type(WriteType::AccountInfoField(AccountInfoField::Lamports))
            .instruction();

        // find payer account and set writable to false
        for account in ix.accounts.iter_mut() {
            if account.pubkey == user.encodable_pubkey() {
                account.is_signer = false;
            }
        }

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&bad_user.encodable_pubkey()),
            &[&bad_user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::AccountValidationFailed),
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn write_type_account_data_out_of_range() {
        let context = &mut TestContext::new().await.unwrap();
        let user = create_user(context).await.unwrap();

        let (memory, memory_bump) = find_memory_pda(user.encodable_pubkey(), 0);

        let test_account = create_test_account(context, &user, false).await.unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[MemoryWriteBuilder::new()
                .payer(user.encodable_pubkey())
                .source_account(test_account.encodable_pubkey())
                .program_id(lighthouse_sdk::ID)
                .memory(memory)
                .memory_id(0)
                .write_offset(0u8.into())
                .memory_bump(memory_bump)
                .write_type(WriteType::AccountData {
                    offset: 1024,
                    data_length: 64,
                })
                .instruction()],
            Some(&user.encodable_pubkey()),
            &[&user],
            context.get_blockhash().await,
        );

        process_transaction_assert_failure(
            context,
            tx,
            to_transaction_error(0, LighthouseError::RangeOutOfBounds),
            None,
        )
        .await
        .unwrap();
    }
}
