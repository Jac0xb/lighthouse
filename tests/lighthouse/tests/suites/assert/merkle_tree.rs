use crate::utils::bubblegum::context::BubblegumTestContext;
use crate::utils::bubblegum::{compute_metadata_hashes, DirtyClone, LeafArgs, Tree};
use crate::utils::context::TestContext;
use crate::utils::utils::process_transaction_assert_success;
use crate::utils::Result;
use anchor_lang::{InstructionData, ToAccountMetas};
use lighthouse_client::instructions::AssertMerkleTreeAccountBuilder;
use mpl_bubblegum::state::leaf_schema::Version;
use mpl_bubblegum::utils::get_asset_id;
use solana_program_test::{tokio, ProgramTestContext};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::keccak;
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
async fn merkle_tree() {
    let context = &mut TestContext::new().await.unwrap();

    let new_owner = Keypair::new();

    let (_, mut tree, mut leaves) = context_tree_and_leaves(&mut context.program_context)
        .await
        .unwrap();

    let leaf = leaves.first_mut().unwrap();
    // let (data_hash, creator_hash) = compute_metadata_hashes(&leaf.metadata).unwrap();

    let tree_pubkey = tree.tree_pubkey();
    let tree_root = tree.decode_root().await.unwrap();

    // let leaf_hash = keccak::hashv(&[
    //     &[Version::V1.to_bytes()],
    //     (get_asset_id(&tree_pubkey, leaf.nonce)).as_ref(),
    //     new_owner.encodable_pubkey().as_ref(),
    //     leaf.delegate.encodable_pubkey().as_ref(),
    //     leaf.nonce.to_le_bytes().as_ref(),
    //     data_hash.as_ref(),
    //     creator_hash.as_ref(),
    // ])
    // .to_bytes();

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
            .merkle_tree(tree_pubkey)
            .root(Pubkey::new_from_array(tree_root))
            .spl_account_compression(spl_account_compression::id())
            .leaf_index(leaf.index)
            .arg0(lighthouse_client::types::LogLevel::PlaintextLog)
            .leaf_hash(new_leaf_hash)
            .add_remaining_accounts(&proof_path_metas)
            .instruction()],
        &[],
    )
    .await
    .unwrap();
}
