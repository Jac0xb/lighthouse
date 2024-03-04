use std::str::FromStr;

use crate::utils::bubblegum::context::BubblegumTestContext;
use crate::utils::bubblegum::tree_manager::TreeManager;
use crate::utils::bubblegum::tx_builder::{self, CreateBuilder, TxBuilder};
use crate::utils::bubblegum::{compute_metadata_hashes, LeafArgs, Tree};
use crate::utils::context::TestContext;
use crate::utils::utils::process_transaction_assert_success;
use crate::utils::Result;
use crate::utils::{create_test_account, create_user_with_balance};
use anchor_lang::{InstructionData, ToAccountMetas};
use borsh::BorshSerialize;
use lighthouse_client::instructions::AssertAccountCompressionBuilder;
use mpl_bubblegum::state::leaf_schema::{self, Version};
use mpl_bubblegum::utils::get_asset_id;
use mpl_bubblegum_client::types::{Creator, MetadataArgs, TokenProgramVersion, TokenStandard};
use solana_program_test::{tokio, ProgramTestContext};
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::keccak;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;

// Test for multiple combinations?
const MAX_DEPTH: usize = 14;
const MAX_BUF_SIZE: usize = 64;

// Minting too many leaves takes quite a long time (in these tests at least).
const DEFAULT_NUM_MINTS: u64 = 10;

pub async fn context_tree_and_leaves(
    program_context: &mut ProgramTestContext,
) -> Result<(
    BubblegumTestContext,
    Tree<MAX_DEPTH, MAX_BUF_SIZE>,
    Vec<LeafArgs>,
)> {
    let context = BubblegumTestContext::new(program_context).await.unwrap();

    let (tree, leaves) = context
        .default_create_and_mint::<MAX_DEPTH, MAX_BUF_SIZE>(DEFAULT_NUM_MINTS)
        .await
        .unwrap();

    Ok((context, tree, leaves))
}

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn merkle_tree() {
    let context = &mut TestContext::new().await.unwrap();

    let new_owner = Keypair::new();

    let (mut bubblegum_context, mut tree, mut leaves) =
        context_tree_and_leaves(&mut context.program_context)
            .await
            .unwrap();

    println!(
        "tree: {:?} {:?} {:?}",
        tree.num_minted(),
        tree.authority(),
        tree.decode_root().await.unwrap()
    );
    println!("leaves: {:?}", leaves);

    tree.transfer(leaves.get_mut(0).unwrap(), &new_owner)
        .await
        .unwrap();

    let leaf = leaves.get(0).unwrap();

    let (data_hash, creator_hash) = compute_metadata_hashes(&leaf.metadata).unwrap();

    let leaf_hash = keccak::hashv(&[
        &[Version::V1.to_bytes()],
        (get_asset_id(&tree.tree_pubkey(), leaf.nonce)).as_ref(),
        new_owner.encodable_pubkey().as_ref(),
        leaf.delegate.encodable_pubkey().as_ref(),
        leaf.nonce.to_le_bytes().as_ref(),
        data_hash.as_ref(),
        creator_hash.as_ref(),
    ])
    .to_bytes();

    // let ix = spl_account_compression::instruction::VerifyLeaf {
    //     root: tree.decode_root().await.unwrap(),
    //     leaf: leaf_hash,
    //     index: leaf.index,
    // };

    // let accounts = spl_account_compression::accounts::VerifyLeaf {
    //     merkle_tree: tree.tree_pubkey(),
    // };

    let proof_path = tree.proof_of_leaf(leaf.index);
    let mut proof_path_metas: Vec<AccountMeta> = vec![];

    for proof in proof_path.iter() {
        proof_path_metas.push(AccountMeta::new_readonly(
            Pubkey::new_from_array(*proof),
            false,
        ));
    }

    let assert_ix = AssertAccountCompressionBuilder::new()
        .merkle_tree(tree.tree_pubkey())
        .root(Pubkey::new_from_array(tree.decode_root().await.unwrap()))
        .spl_account_compression(spl_account_compression::id())
        .leaf_index(leaf.index)
        .arg0(lighthouse_client::types::LogLevel::PlaintextLog)
        .leaf_hash(leaf_hash)
        .add_remaining_accounts(&proof_path_metas)
        .instruction();

    let tx = Transaction::new_signed_with_payer(
        &[assert_ix],
        Some(&context.payer().encodable_pubkey()),
        &[&context.payer()],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    {
        let ix = spl_account_compression::instruction::VerifyLeaf {
            root: tree.decode_root().await.unwrap(),
            leaf: leaf_hash,
            index: leaf.index,
        };

        let accounts = spl_account_compression::accounts::VerifyLeaf {
            merkle_tree: tree.tree_pubkey(),
        };

        let mut accounts: Vec<AccountMeta> = accounts.to_account_metas(None);

        for proof in proof_path.iter() {
            accounts.push(AccountMeta::new_readonly(
                Pubkey::new_from_array(*proof),
                false,
            ));
        }

        let tx = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: spl_account_compression::id(),
                accounts,
                data: ix.data(),
            }],
            Some(&context.payer().encodable_pubkey()),
            &[&context.payer()],
            context.get_blockhash().await,
        );

        process_transaction_assert_success(context, tx)
            .await
            .unwrap();
    }
}
