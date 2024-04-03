---
title: Assert Merkle Tree Account
metaTitle: Assert - Assert Merkle Tree Account
description:
---

## AssertAccountDelta Instruction

The **AssertMerkleTreeAccount** instruction is a wrapper for the `verify_leaf` instruction for the spl-account-compression program. The one advantage it has over the verify leaf instruction is that you can pass in a dummy account in as the root instead of needing to pass a [u8; 32] as the instruction root argument if you had called `verify_leaf` instruction directly.

{% callout %}
Using this over verify_leaf instruction saves 31 bytes (32 bytes reduced to 1 byte if you use an account that already exists in the transaction) but incurs a large CU cost for the CPI. The reason you can pass in an arbitrary root into the spl-account-compression program is it is ignored in the instruction since the current root will always be in the changelogs.
{% /callout %}

### Example: Assert on a leaf node in a merkle tree account.

The idea is to get fetch the proof path and also recreate the hash of the state you expect for the leaf node at the end of the transaction. This is useful for asserting that the leaf node matches what you expect.

{% dialect-switcher title="" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
AssertMerkleTreeAccountBuilder::new()
    .target_merkle_tree(tree_pubkey)
    .root(Pubkey::new_from_array(tree_root)) // This can be any account, preferably one that already exists in the transaction. See the note above.
    .spl_account_compression(spl_account_compression::id())
    .assertion(MerkleTreeAssertion::VerifyLeaf {
        leaf_index: leaf.index,
        leaf_hash: expectet_leaf_hash,  // This would be the leaf node with the state you expect at the end of the transaction.
    })
    .add_remaining_accounts(&proof_path_metas) // The proof path to verify the leaf node.
    .instruction()
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

<!--

Imagine a struct with the following layout:

```rust
pub struct TestAccount {
    pub account_discriminator: [u8; 8], // bytes 0 to 7
    pub balance: u64,                   // bytes 8 to 16
}
```

Say we wanted to assert on the _balance_ field in our TestAccount struct. If the struct uses a serialization schema that uses little-endian for integers (borsh, bytemuck, ...) we can deserialize the field and assert on that value at runtime!

{% dialect-switcher title="" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
AssertAccountDataBuilder::new()
    .target_account(test_account_key)
    .assertion(DataValueAssertion::U64 {
        value: 420, // The expected value.
        operator: IntegerOperator::GreaterThanOrEqual,
    })
    .offset(8) // The account data byte offset where 'balance' is stored.
    .instruction();
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Example: Equality assertion on pubkey in account data.

Imagine we expand the struct from before and add a pubkey field.

```rust
pub struct TestAccount {
    pub account_discriminator: [u8; 8], // bytes 0 to 7
    pub balance: u64,                   // bytes 8 to 16
    pub owner: Pubkey,                  // bytes 16 to 48
}
```

Say we wanted to assert on the "balance" field in our TestAccount struct. If the struct uses a serialization schema that uses little-endian for integers (borsh, bytemuck, ...) we can deserialize the field and assert on that value at runtime!

{% dialect-switcher title="" %}
{% dialect title="Rust" id="rust" %}
{% totem %}

```rust
AssertAccountDataBuilder::new()
    .target_account(test_account_key)
    .assertion(DataValueAssertion::Pubkey {
        value: owner_key,
        operator: EquatableOperator::Equal,
    })
    .offset(16) // The account data byte offset where 'owner' is stored.
    .instruction(),

```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %} -->
