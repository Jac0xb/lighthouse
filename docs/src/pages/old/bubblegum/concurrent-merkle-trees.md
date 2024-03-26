---
title: Concurrent Merkle Trees
metaTitle: Bubblegum - Concurrent Merkle Trees
description: Learn more about Concurrent Merkle Trees and how they are used on Bubblegum
---

## Introduction

A Merkle Tree is a tree data structure in which each leaf node is labeled with a hash representing some data.  Adjacent leaves are hashed together, and the resulting hash becomes the label for the node that is the parent of those leaves.  Nodes at the same level are hashed together again, and the resulting hash becomes the label for the node that is the parent of those nodes.  This process continues until a single hash is created for the root node.  This single hash cryptographically represents the data integrity of the entire tree, and is called the Merkle root.

Most Merkle trees are binary trees, but they do not have to be.  The Merkle tree used for Bubblegum compressed NFTs (cNFTs) is a binary tree as shown in our diagram.

{% diagram %}

{% node %}
{% node #root label="Merkle Root" /%}
{% node label="Hash(Node 1, Node 2)" /%}
{% /node %}

{% node parent="root" y=100 x=-220 %}
{% node #i-node-1 label="Node 1" /%}
{% node label="Hash(Node 3, Node 4)" /%}
{% /node %}

{% node parent="root" y=100 x=220 %}
{% node #i-node-2 label="Node 2" /%}
{% node label="Hash(Node 5, Node 6)" /%}
{% /node %}

{% node parent="i-node-1" y=100 x=-110 %}
{% node #i-node-3 label="Node 3" /%}
{% node label="Hash(Leaf 1, Leaf 2)" /%}
{% /node %}

{% node parent="i-node-1" y=100 x=110 %}
{% node #i-node-4 label="Node 4" /%}
{% node label="Hash(Leaf 3, Leaf 4)" /%}
{% /node %}

{% node parent="i-node-2" y=100 x=-110 %}
{% node #i-node-5 label="Node 5" /%}
{% node label="Hash(Leaf 5, Leaf 6)" /%}
{% /node %}

{% node parent="i-node-2" y=100 x=110 %}
{% node #i-node-6 label="Node 6" /%}
{% node label="Hash(Leaf 7, Leaf 8)" /%}
{% /node %}

{% node parent="i-node-3" y="100" x="-40" %}
{% node #leaf-1 label="Leaf 1" /%}
{% node label="Hash(cNFT 1)" /%}
{% /node %}

{% node parent="i-node-3" y="100" x="70" %}
{% node #leaf-2 label="Leaf 2" /%}
{% node label="Hash(cNFT 2)" /%}
{% /node %}

{% node parent="i-node-4" y="100" x="-40" %}
{% node #leaf-3 label="Leaf 3" /%}
{% node label="Hash(cNFT 3)" /%}
{% /node %}

{% node parent="i-node-4" y="100" x="70" %}
{% node #leaf-4 label="Leaf 4" /%}
{% node label="Hash(cNFT 4)" /%}
{% /node %}

{% node parent="i-node-5" y="100" x="-40" %}
{% node #leaf-5 label="Leaf 5" /%}
{% node label="Hash(cNFT 5)" /%}
{% /node %}

{% node parent="i-node-5" y="100" x="70" %}
{% node #leaf-6 label="Leaf 6" /%}
{% node label="Hash(cNFT 6)" /%}
{% /node %}

{% node parent="i-node-6" y="100" x="-40" %}
{% node #leaf-7 label="Leaf 7" /%}
{% node label="Hash(cNFT 7)" /%}
{% /node %}

{% node parent="i-node-6" y="100" x="70" %}
{% node #leaf-8 label="Leaf 8" /%}
{% node label="Hash(cNFT 8)" /%}
{% /node %}

{% edge from="i-node-1" to="root" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-2" to="root" fromPosition="top" toPosition="bottom" /%}

{% edge from="i-node-3" to="i-node-1" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-4" to="i-node-1" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-6" to="i-node-2" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-5" to="i-node-2" fromPosition="top" toPosition="bottom" /%}

{% edge from="leaf-1" to="i-node-3" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-2" to="i-node-3" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-4" to="i-node-4" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-3" to="i-node-4" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-5" to="i-node-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-6" to="i-node-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-7" to="i-node-6" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-8" to="i-node-6" fromPosition="top" toPosition="bottom" /%}

{% /diagram %}

When we talk about storing the state of data on the blockchain, if we store this Merkle root, we can effectively store a single value that represents the data integrity of everything that was previously hashed in order to create the root.  If any leaf value were to change on the tree, the existing Merkle root would become invalid and need to be recomputed.

For Bubblegum compressed NFTs, the leaf node hashes are the hash of a [leaf schema](https://github.com/metaplex-foundation/mpl-bubblegum/blob/main/programs/bubblegum/program/src/state/leaf_schema.rs#L40).  The leaf schema contains a leaf ID, owner/delegate information, a [`creator_hash`](https://github.com/metaplex-foundation/mpl-bubblegum/blob/main/programs/bubblegum/program/src/lib.rs#L433) representing the cNFT's [creators](https://github.com/metaplex-foundation/mpl-bubblegum/blob/main/programs/bubblegum/program/src/state/metaplex_adapter.rs#L103), and a [`data_hash`](https://github.com/metaplex-foundation/mpl-bubblegum/blob/main/programs/bubblegum/program/src/lib.rs#L450) representing the compressed NFT's [metadata](https://github.com/metaplex-foundation/mpl-bubblegum/blob/main/programs/bubblegum/program/src/state/metaplex_adapter.rs#L81) in general (it again includes the creator array).  So all the information we need to cryptographically verify a single compressed NFT is stored in the hashed leaf schema.

## Leaf Path

As we learned in the previous section, in a Merkle tree only the leaf nodes represent end-user data.  The inner nodes leading up to the hash are all just intermediate values in service to the Merkle root.  When we refer to a leaf node's **Path**, we mean the leaf node hash itself and the inner nodes directly leading to the Merkle root.  For example, the Path for leaf 2 is highlighted in the diagram below.

{% diagram %}

{% node %}
{% node #root label="Merkle Root" theme="blue" /%}
{% node label="Hash(Node 1, Node 2)" theme="blue" /%}
{% /node %}

{% node parent="root" y=100 x=-220 %}
{% node #i-node-1 label="Node 1" theme="blue" /%}
{% node label="Hash(Node 3, Node 4)" theme="blue" /%}
{% /node %}

{% node parent="root" y=100 x=220 %}
{% node #i-node-2 label="Node 2" /%}
{% node label="Hash(Node 5, Node 6)" /%}
{% /node %}

{% node parent="i-node-1" y=100 x=-110 %}
{% node #i-node-3 label="Node 3" theme="blue" /%}
{% node label="Hash(Leaf 1, Leaf 2)" theme="blue" /%}
{% /node %}

{% node parent="i-node-1" y=100 x=110 %}
{% node #i-node-4 label="Node 4" /%}
{% node label="Hash(Leaf 3, Leaf 4)" /%}
{% /node %}

{% node parent="i-node-2" y=100 x=-110 %}
{% node #i-node-5 label="Node 5" /%}
{% node label="Hash(Leaf 5, Leaf 6)" /%}
{% /node %}

{% node parent="i-node-2" y=100 x=110 %}
{% node #i-node-6 label="Node 6" /%}
{% node label="Hash(Leaf 7, Leaf 8)" /%}
{% /node %}

{% node parent="i-node-3" y="100" x="-40" %}
{% node #leaf-1 label="Leaf 1" /%}
{% node label="Hash(cNFT 1)" /%}
{% /node %}

{% node parent="i-node-3" y="100" x="70" %}
{% node #leaf-2 label="Leaf 2" theme="blue" /%}
{% node label="Hash(cNFT 2)" theme="blue" /%}
{% /node %}

{% node parent="i-node-4" y="100" x="-40" %}
{% node #leaf-3 label="Leaf 3" /%}
{% node label="Hash(cNFT 3)" /%}
{% /node %}

{% node parent="i-node-4" y="100" x="70" %}
{% node #leaf-4 label="Leaf 4" /%}
{% node label="Hash(cNFT 4)" /%}
{% /node %}

{% node parent="i-node-5" y="100" x="-40" %}
{% node #leaf-5 label="Leaf 5" /%}
{% node label="Hash(cNFT 5)" /%}
{% /node %}

{% node parent="i-node-5" y="100" x="70" %}
{% node #leaf-6 label="Leaf 6" /%}
{% node label="Hash(cNFT 6)" /%}
{% /node %}

{% node parent="i-node-6" y="100" x="-40" %}
{% node #leaf-7 label="Leaf 7" /%}
{% node label="Hash(cNFT 7)" /%}
{% /node %}

{% node parent="i-node-6" y="100" x="70" %}
{% node #leaf-8 label="Leaf 8" /%}
{% node label="Hash(cNFT 8)" /%}
{% /node %}

{% edge from="i-node-1" to="root" fromPosition="top" toPosition="bottom" theme="blue" animated=true /%}
{% edge from="i-node-2" to="root" fromPosition="top" toPosition="bottom" /%}

{% edge from="i-node-3" to="i-node-1" fromPosition="top" toPosition="bottom" theme="blue" animated=true /%}
{% edge from="i-node-4" to="i-node-1" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-6" to="i-node-2" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-5" to="i-node-2" fromPosition="top" toPosition="bottom" /%}

{% edge from="leaf-1" to="i-node-3" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-2" to="i-node-3" fromPosition="top" toPosition="bottom" theme="blue" animated=true /%}
{% edge from="leaf-4" to="i-node-4" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-3" to="i-node-4" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-5" to="i-node-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-6" to="i-node-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-7" to="i-node-6" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-8" to="i-node-6" fromPosition="top" toPosition="bottom" /%}

{% /diagram %}

## Leaf Proof

 If we want to prove whether a compressed NFT exists in a Merkle tree, we don't need to rehash all the leaf nodes.  As you can see in the diagram below we only need to have certain values to hash together until we calculate our Merkle root.  These values are known as the **Proof** for the leaf.  Specifically, the Proof for a leaf node is the adjacent leaf node's hash, and the adjacent inner node hashes that can be used to calculate the Merkle root.  The Proof for leaf 2 is highlighted in the diagram below.

{% diagram %}

{% node %}
{% node #root label="Merkle Root" /%}
{% node label="Hash(Node 1, Node 2)" /%}
{% /node %}

{% node parent="root" y=100 x=-220 %}
{% node #i-node-1 label="Node 1" /%}
{% node label="Hash(Node 3, Node 4)" /%}
{% /node %}

{% node parent="root" y=100 x=220 %}
{% node #i-node-2 label="Node 2" theme="mint" /%}
{% node label="Hash(Node 5, Node 6)" theme="mint" /%}
{% /node %}

{% node parent="i-node-1" y=100 x=-110 %}
{% node #i-node-3 label="Node 3" /%}
{% node label="Hash(Leaf 1, Leaf 2)" /%}
{% /node %}

{% node parent="i-node-1" y=100 x=110 %}
{% node #i-node-4 label="Node 4" theme="mint" /%}
{% node label="Hash(Leaf 3, Leaf 4)" theme="mint" /%}
{% /node %}

{% node parent="i-node-2" y=100 x=-110 %}
{% node #i-node-5 label="Node 5" /%}
{% node label="Hash(Leaf 5, Leaf 6)" /%}
{% /node %}

{% node parent="i-node-2" y=100 x=110 %}
{% node #i-node-6 label="Node 6" /%}
{% node label="Hash(Leaf 7, Leaf 8)" /%}
{% /node %}

{% node parent="i-node-3" y="100" x="-40" %}
{% node #leaf-1 label="Leaf 1" theme="mint" /%}
{% node label="Hash(cNFT 1)" theme="mint" /%}
{% /node %}

{% node parent="i-node-3" y="100" x="70" %}
{% node #leaf-2 label="Leaf 2" theme="blue" /%}
{% node label="Hash(cNFT 2)" theme="blue" /%}
{% /node %}

{% node parent="i-node-4" y="100" x="-40" %}
{% node #leaf-3 label="Leaf 3" /%}
{% node label="Hash(cNFT 3)" /%}
{% /node %}

{% node parent="i-node-4" y="100" x="70" %}
{% node #leaf-4 label="Leaf 4" /%}
{% node label="Hash(cNFT 4)" /%}
{% /node %}

{% node parent="i-node-5" y="100" x="-40" %}
{% node #leaf-5 label="Leaf 5" /%}
{% node label="Hash(cNFT 5)" /%}
{% /node %}

{% node parent="i-node-5" y="100" x="70" %}
{% node #leaf-6 label="Leaf 6" /%}
{% node label="Hash(cNFT 6)" /%}
{% /node %}

{% node parent="i-node-6" y="100" x="-40" %}
{% node #leaf-7 label="Leaf 7" /%}
{% node label="Hash(cNFT 7)" /%}
{% /node %}

{% node parent="i-node-6" y="100" x="70" %}
{% node #leaf-8 label="Leaf 8" /%}
{% node label="Hash(cNFT 8)" /%}
{% /node %}

{% edge from="i-node-1" to="root" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-2" to="root" fromPosition="top" toPosition="bottom" /%}

{% edge from="i-node-3" to="i-node-1" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-4" to="i-node-1" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-6" to="i-node-2" fromPosition="top" toPosition="bottom" /%}
{% edge from="i-node-5" to="i-node-2" fromPosition="top" toPosition="bottom" /%}

{% edge from="leaf-1" to="i-node-3" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-2" to="i-node-3" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-4" to="i-node-4" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-3" to="i-node-4" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-5" to="i-node-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-6" to="i-node-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-7" to="i-node-6" fromPosition="top" toPosition="bottom" /%}
{% edge from="leaf-8" to="i-node-6" fromPosition="top" toPosition="bottom" /%}

{% /diagram %}

## Leaf Validation

The process for using the leaf node and its Proof to calculate the Merkle root is as follows:
1. Start with our raw leaf schema, hash it.
2. Hash the value from step 1 with the sibling leaf node's hash to create the next value up of the leaf's Path.
3. Hash the path value from step 2 with the next sibling inner node, which is the next value of the Proof.
4. Continue this process of hashing values with sibling inner node values, up the tree until we calculate the Merkle root.

If the Merkle root we calculate matches the Merkle root we were given for that tree, then we know that our exact leaf node exists in the Merkle tree.  Also any time a leaf node is updated (i.e. when the cNFT is transferred to a new owner), a new Merkle root must be calculated and updated on-chain.

## Concurrency

The on-chain Merkle tree used for cNFTs must be able to handle multiple writes occurring in the same block.  This is because there can be multiple transactions to mint new cNFTs to the tree, transfer cNFTs, delegate cNFTs, burn cNFTs, etc.  The problem is that the first write to the on-chain tree invalidates the proofs sent for other writes within the same block.

The solution for this is that the Merkle tree used by [spl-account-compression](https://spl.solana.com/account-compression) doesn't only store one Merkle root, but also stores a [`ChangeLog`](https://github.com/solana-labs/solana-program-library/blob/master/libraries/concurrent-merkle-tree/src/changelog.rs#L9) of previous roots and the paths for previously modified leaves.  Even if the root and proof sent by the new transaction have been invalidated by a previous update, the program will fast-forward the proof.  Note the number of `ChangeLog`s available is set by the [Max Buffer Size](/bubblegum/create-trees#creating-a-bubblegum-tree) used when creating the tree.

Also note that the rightmost proof for the Merkle tree is stored on-chain.  This allows for appends to the tree to occur without needing a proof to be sent.  This is exactly how Bubblegum is able to mint new cNFTs without needing a proof.
