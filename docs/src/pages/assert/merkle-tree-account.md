---
title: Assert Merkle Tree Account
metaTitle: Assert - Assert Merkle Tree Account
description:
---

## AssertAccountDelta Instruction

The **AssertMerkleTreeAccount** instruction is a wrapper for the `verify_leaf` instruction for the `spl-account-compression` program. The one advantage it has over the verify leaf instruction is that you can pass in a dummy account in as the root instead of needing to pass a `[u8; 32]` as the instruction root argument if you had called `verify_leaf` instruction directly.

{% callout %}
Using this over verify_leaf instruction saves 31 bytes (32 bytes reduced to 1 byte if you use an account that already exists in the transaction) but incurs a large CU cost for the CPI. The reason you can pass in an arbitrary root into the spl-account-compression program is it is ignored in the instruction since the current root will always be in the merkle tree changelogs.
{% /callout %}

### Example: Assert on a leaf node in a merkle tree account.

To verify leaf using Lighthouse, fetch the proof path and generate the hash of the state you expect the leaf node to have at the end of the transaction.

{% dialect-switcher title="Verify leaf through Lighthouse instruction" %}
{% dialect title="web3.js (Legacy)" id="js-legacy" %}
{% totem %}

```typescript
const proofPathMetas: AccountMeta[] = proofPath.map((hash) => ({
  pubkey: publicKey(hash),
  isWritable: false,
  isSigner: false,
}))

const tx = assertMerkleTreeAccount(umi, {
  targetMerkleTree,
  root,
  splAccountCompression,
  assertion: {
    __kind: 'VerifyLeaf',
    leafIndex: leafIndex,
    leafHash: expectedLeafHash,
  },
})
  .addRemainingAccounts(proofPathMetas)
  .build(umi)
```

{% /totem %}
{% /dialect %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
let ix = AssertMerkleTreeAccountBuilder::new()
    .target_merkle_tree(tree_pubkey)
    .root(Pubkey::new_from_array(tree_root)) // This can be any account, preferably one that already exists in the transaction. See the note above.
    .spl_account_compression(spl_account_compression::id())
    .assertion(MerkleTreeAssertion::VerifyLeaf {
        leaf_index: leaf.index,
        leaf_hash: expected_leaf_hash,  // This would be the leaf node with the state you expect at the end of the transaction.
    })
    .add_remaining_accounts(&proof_path_metas) // The proof path to verify the leaf node.
    .instruction()
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
