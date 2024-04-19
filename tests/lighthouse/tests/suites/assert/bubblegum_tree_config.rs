use crate::utils::bubblegum::context::BubblegumTestContext;
use crate::utils::bubblegum::{DirtyClone, LeafArgs, Tree};
use crate::utils::context::TestContext;
use crate::utils::{
    create_user, process_transaction_assert_success, set_account_from_refs, Result,
};
use borsh::BorshSerialize;
use lighthaus_sdk::instructions::{
    AssertBubblegumTreeConfigAccountBuilder, AssertMerkleTreeAccountBuilder,
};
use lighthaus_sdk::types::{
    BubblegumTreeConfigAssertion, EquatableOperator, IntegerOperator, MerkleTreeAssertion,
};
use mpl_bubblegum_sdk::accounts::TreeConfig;
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

    let new_owner = create_user(context).await.unwrap();

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

    let tree_config = tree.read_tree_config().await.unwrap();
    let tree_config_key = tree.authority();

    println!("tree config {:?}", tree_config);

    tree.transfer(
        leaf,
        &new_owner,
        &[
            AssertMerkleTreeAccountBuilder::new()
                .target_merkle_tree(tree_pubkey)
                .root(Pubkey::new_from_array(tree_root))
                .spl_account_compression(spl_account_compression::id())
                .log_level(lighthaus_sdk::types::LogLevel::Silent)
                .assertion(MerkleTreeAssertion::VerifyLeaf {
                    leaf_index: leaf.index,
                    leaf_hash: new_leaf_hash,
                })
                .add_remaining_accounts(&proof_path_metas)
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::IsDecompressible {
                    value: 1,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::IsPublic {
                    value: false,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::NumMinted {
                    value: 10,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::TotalMintCapacity {
                    value: tree_config.total_mint_capacity,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::TreeDelegate {
                    value: tree_config.tree_delegate,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::TreeCreator {
                    value: tree_config.tree_creator,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        ],
        &[],
    )
    .await
    .unwrap();

    let new_delegate = create_user(context).await.unwrap();
    tree.set_tree_delegate_tx(&new_delegate)
        .execute(
            &[
                AssertBubblegumTreeConfigAccountBuilder::new()
                    .target_account(tree_config_key)
                    .assertion(BubblegumTreeConfigAssertion::TreeDelegate {
                        value: new_delegate.encodable_pubkey(),
                        operator: EquatableOperator::Equal,
                    })
                    .instruction(),
                AssertBubblegumTreeConfigAccountBuilder::new()
                    .target_account(tree_config_key)
                    .assertion(BubblegumTreeConfigAssertion::TreeCreator {
                        value: tree_config.tree_creator,
                        operator: EquatableOperator::Equal,
                    })
                    .instruction(),
            ],
            &[],
        )
        .await
        .unwrap();

    let new_config = TreeConfig {
        discriminator: [0; 8],
        tree_creator: Keypair::new().encodable_pubkey(),
        tree_delegate: Keypair::new().encodable_pubkey(),
        total_mint_capacity: tree_config.total_mint_capacity + 1,
        num_minted: tree_config.total_mint_capacity,
        is_public: true,
        is_decompressible: mpl_bubblegum_sdk::types::DecompressibleState::Enabled,
    };

    let mut config_account_data = context.get_account(tree_config_key).await.unwrap().data;

    config_account_data[8..90].copy_from_slice(&new_config.try_to_vec().unwrap()[8..]);

    set_account_from_refs(
        context,
        &tree_config_key,
        &config_account_data,
        &mpl_bubblegum_sdk::ID,
    )
    .await;

    let tx = Transaction::new_signed_with_payer(
        &[
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::IsDecompressible {
                    value: new_config.is_decompressible as u8,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::IsPublic {
                    value: new_config.is_public,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::NumMinted {
                    value: new_config.num_minted,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::TotalMintCapacity {
                    value: new_config.total_mint_capacity,
                    operator: IntegerOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::TreeDelegate {
                    value: new_config.tree_delegate,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
            AssertBubblegumTreeConfigAccountBuilder::new()
                .target_account(tree_config_key)
                .assertion(BubblegumTreeConfigAssertion::TreeCreator {
                    value: new_config.tree_creator,
                    operator: EquatableOperator::Equal,
                })
                .instruction(),
        ],
        Some(&new_owner.encodable_pubkey()),
        &[&new_owner],
        context.get_blockhash().await,
    );

    process_transaction_assert_success(context, tx)
        .await
        .unwrap();
}
