use crate::utils::bubblegum::context::BubblegumTestContext;
use crate::utils::bubblegum::{DirtyClone, LeafArgs, Tree};
use crate::utils::context::TestContext;
use crate::utils::{
    process_transaction_assert_failure, process_transaction_assert_success, to_transaction_error,
    to_transaction_error_u8, Result,
};
use lighthouse_client::errors::LighthouseError;
use lighthouse_client::instructions::AssertMerkleTreeAccountBuilder;
use lighthouse_client::types::MerkleTreeAssertion;
use solana_program_test::{tokio, ProgramTestContext};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::EncodableKeypair;
use solana_sdk::transaction::Transaction;

const MAX_DEPTH: usize = 14;
const MAX_BUF_SIZE: usize = 64;
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
async fn simple() {
    let context = &mut TestContext::new().await.unwrap();

    let new_owner = Keypair::new();

    let (_, mut tree, mut leaves) = context_tree_and_leaves(&mut context.program_context)
        .await
        .unwrap();

    let leaf = leaves.first_mut().unwrap();

    let tree_pubkey = tree.tree_pubkey();
    let tree_root = tree.decode_root().await.unwrap();

    let proof_path = tree.proof_of_leaf(leaf.index);
    let mut proof_path_metas: Vec<AccountMeta> = vec![];

    for proof in proof_path.iter() {
        proof_path_metas.push(AccountMeta::new_readonly(
            Pubkey::new_from_array(*proof),
            false,
        ));
    }

    let mut modified_leaf_node = leaf.clone();
    modified_leaf_node.owner = new_owner.dirty_clone();
    modified_leaf_node.delegate = new_owner.dirty_clone();
    let new_leaf_hash = tree.leaf_node(&modified_leaf_node).unwrap();

    tree.transfer(
        leaf,
        &new_owner,
        &[AssertMerkleTreeAccountBuilder::new()
            .target_merkle_tree(tree_pubkey)
            .root(Pubkey::new_from_array(tree_root))
            .spl_account_compression(spl_account_compression::id())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(MerkleTreeAssertion::VerifyLeaf {
                leaf_index: leaf.index,
                leaf_hash: new_leaf_hash,
            })
            .add_remaining_accounts(&proof_path_metas)
            .instruction()],
        &[],
    )
    .await
    .unwrap();
}

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn simple_no_modify() {
    let context = &mut TestContext::new().await.unwrap();

    let (_, mut tree, mut leaves) = context_tree_and_leaves(&mut context.program_context)
        .await
        .unwrap();

    let leaf = leaves.first_mut().unwrap();

    let tree_pubkey = tree.tree_pubkey();
    let tree_root = tree.decode_root().await.unwrap();

    let proof_path = tree.proof_of_leaf(leaf.index);
    let mut proof_path_metas: Vec<AccountMeta> = vec![];

    for proof in proof_path.iter() {
        proof_path_metas.push(AccountMeta::new_readonly(
            Pubkey::new_from_array(*proof),
            false,
        ));
    }

    let leaf_hash = tree.leaf_node(leaf).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[AssertMerkleTreeAccountBuilder::new()
            .target_merkle_tree(tree_pubkey)
            .root(Pubkey::new_from_array(tree_root))
            .spl_account_compression(spl_account_compression::id())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(MerkleTreeAssertion::VerifyLeaf {
                leaf_index: leaf.index,
                leaf_hash,
            })
            .add_remaining_accounts(&proof_path_metas)
            .instruction()],
        Some(&context.program_context.payer.encodable_pubkey()),
        &[&context.program_context.payer],
        context.program_context.last_blockhash,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Bad leaf

    let tx = Transaction::new_signed_with_payer(
        &[AssertMerkleTreeAccountBuilder::new()
            .target_merkle_tree(tree_pubkey)
            .root(Pubkey::new_from_array(tree_root))
            .spl_account_compression(spl_account_compression::id())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(MerkleTreeAssertion::VerifyLeaf {
                leaf_index: leaf.index,
                leaf_hash: [69; 32],
            })
            .add_remaining_accounts(&proof_path_metas)
            .instruction()],
        Some(&context.program_context.payer.encodable_pubkey()),
        &[&context.program_context.payer],
        context.program_context.last_blockhash,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Bad proof path

    let bad_proof_path_metas = proof_path_metas
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<AccountMeta>>();

    let tx = Transaction::new_signed_with_payer(
        &[AssertMerkleTreeAccountBuilder::new()
            .target_merkle_tree(tree_pubkey)
            .root(Pubkey::new_from_array(tree_root))
            .spl_account_compression(spl_account_compression::id())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(MerkleTreeAssertion::VerifyLeaf {
                leaf_index: leaf.index,
                leaf_hash,
            })
            .add_remaining_accounts(&bad_proof_path_metas)
            .instruction()],
        Some(&context.program_context.payer.encodable_pubkey()),
        &[&context.program_context.payer],
        context.program_context.last_blockhash,
    );

    process_transaction_assert_failure(
        context,
        tx,
        to_transaction_error(0, LighthouseError::AssertionFailed),
        None,
    )
    .await
    .unwrap();

    // Bad root (actually works fine because of how proof caching works)

    let tx = Transaction::new_signed_with_payer(
        &[AssertMerkleTreeAccountBuilder::new()
            .target_merkle_tree(tree_pubkey)
            .root(lighthouse_client::ID)
            .spl_account_compression(spl_account_compression::id())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(MerkleTreeAssertion::VerifyLeaf {
                leaf_index: leaf.index,
                leaf_hash,
            })
            .add_remaining_accounts(&proof_path_metas)
            .instruction()],
        Some(&context.program_context.payer.encodable_pubkey()),
        &[&context.program_context.payer],
        context.program_context.last_blockhash,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();

    // Bad tree

    let tx = Transaction::new_signed_with_payer(
        &[AssertMerkleTreeAccountBuilder::new()
            .target_merkle_tree(Pubkey::new_from_array([69; 32]))
            .root(Pubkey::new_from_array(tree_root))
            .spl_account_compression(spl_account_compression::id())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(MerkleTreeAssertion::VerifyLeaf {
                leaf_index: leaf.index,
                leaf_hash,
            })
            .add_remaining_accounts(&proof_path_metas)
            .instruction()],
        Some(&context.program_context.payer.encodable_pubkey()),
        &[&context.program_context.payer],
        context.program_context.last_blockhash,
    );

    process_transaction_assert_failure(context, tx, to_transaction_error_u8(0, 6006), None)
        .await
        .unwrap();
}

#[tokio::test]
async fn fail() {
    let context = &mut TestContext::new().await.unwrap();

    let new_owner = Keypair::new();

    let (_, mut tree, mut leaves) = context_tree_and_leaves(&mut context.program_context)
        .await
        .unwrap();

    let leaf = leaves.first_mut().unwrap();

    let tree_pubkey = tree.tree_pubkey();
    let tree_root = tree.decode_root().await.unwrap();

    let proof_path = tree.proof_of_leaf(leaf.index);
    let mut proof_path_metas: Vec<AccountMeta> = vec![];

    for proof in proof_path.iter() {
        proof_path_metas.push(AccountMeta::new_readonly(
            Pubkey::new_from_array(*proof),
            false,
        ));
    }

    // Unmodified leaf hash should fail after transfer

    let leaf_hash = tree.leaf_node(leaf).unwrap();

    let result = tree
        .transfer(
            leaf,
            &new_owner,
            &[AssertMerkleTreeAccountBuilder::new()
                .target_merkle_tree(tree_pubkey)
                .root(Pubkey::new_from_array(tree_root))
                .spl_account_compression(spl_account_compression::id())
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(MerkleTreeAssertion::VerifyLeaf {
                    leaf_index: leaf.index,
                    leaf_hash,
                })
                .add_remaining_accounts(&proof_path_metas)
                .instruction()],
            &[],
        )
        .await;

    assert!(result.is_err());

    // Fail on bad proof path

    let mut modified_leaf_node = leaf.clone();
    modified_leaf_node.owner = new_owner.dirty_clone();
    modified_leaf_node.delegate = new_owner.dirty_clone();
    let new_leaf_hash = tree.leaf_node(&modified_leaf_node).unwrap();

    let bad_proof_path_metas = proof_path_metas
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<AccountMeta>>();

    let result = tree
        .transfer(
            leaf,
            &new_owner,
            &[AssertMerkleTreeAccountBuilder::new()
                .target_merkle_tree(tree_pubkey)
                .root(Pubkey::new_from_array(tree_root))
                .spl_account_compression(spl_account_compression::id())
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(MerkleTreeAssertion::VerifyLeaf {
                    leaf_index: leaf.index,
                    leaf_hash: new_leaf_hash,
                })
                .add_remaining_accounts(&bad_proof_path_metas)
                .instruction()],
            &[],
        )
        .await;

    assert!(result.is_err());

    // Fail on tree not owned by program

    let result = tree
        .transfer(
            leaf,
            &new_owner,
            &[AssertMerkleTreeAccountBuilder::new()
                .target_merkle_tree(Pubkey::new_from_array([0; 32]))
                .root(Pubkey::new_from_array(tree_root))
                .spl_account_compression(spl_account_compression::id())
                .log_level(lighthouse_client::types::LogLevel::Silent)
                .assertion(MerkleTreeAssertion::VerifyLeaf {
                    leaf_index: leaf.index,
                    leaf_hash: new_leaf_hash,
                })
                .add_remaining_accounts(&proof_path_metas)
                .instruction()],
            &[],
        )
        .await;

    assert!(result.is_err());

    tree.transfer(
        leaf,
        &new_owner,
        &[AssertMerkleTreeAccountBuilder::new()
            .target_merkle_tree(tree_pubkey)
            .root(Pubkey::new_from_array(tree_root))
            .spl_account_compression(spl_account_compression::id())
            .log_level(lighthouse_client::types::LogLevel::Silent)
            .assertion(MerkleTreeAssertion::VerifyLeaf {
                leaf_index: leaf.index,
                leaf_hash: new_leaf_hash,
            })
            .add_remaining_accounts(&proof_path_metas)
            .instruction()],
        &[],
    )
    .await
    .unwrap();
}
