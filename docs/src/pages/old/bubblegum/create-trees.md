---
title: Creating Bubblegum Trees
metaTitle: Bubblegum - Creating Bubblegum Trees
description: Learn how to create and fetch new Merkle Trees that can hold compressed NFTs
---

## Introduction

Whilst the data of Compressed NFTs is stored inside transactions and not on-chain accounts, we still need some on-chain accounts to keep track of the Merkle Tree and its configuration. As such, before we can start minting Compressed NFTs, we need to create two accounts:

- A **Merkle Tree account**. This account holds a generic Merkle Tree that can be used to verify the authenticity of any type of data. It is owned by the [Account Compression Program](https://spl.solana.com/account-compression) created and maintained by Solana. In our case, we will use it to verify the authenticity of Compressed NFTs.
- A **Tree Config account**. This second account is a PDA derived from the address of the Merkle Tree account. It allows us to store additional configurations for the Merkle Tree that are specific to Compressed NFTs — e.g. the tree creator, the number of minted cNFTs, etc.

With these two accounts, we have everything we need to start minting Compressed NFTs. Note that, we will refer to Merkle Tree accounts with associated Tree Config accounts as **Bubblegum Trees**.

{% diagram height="h-64 md:h-[200px]" %}

{% node %}
{% node #merkle-tree label="Merkle Tree Account" theme="blue" /%}
{% node label="Owner: Account Compression Program" theme="dimmed" /%}
{% /node %}

{% node #tree-config-pda parent="merkle-tree" x="300" label="PDA" theme="crimson" /%}

{% node parent="tree-config-pda" y="60" %}
{% node #tree-config label="Tree Config Account" theme="crimson" /%}
{% node label="Owner: Bubblegum Program" theme="dimmed" /%}
{% /node %}

{% edge from="merkle-tree" to="tree-config-pda" /%}
{% edge from="tree-config-pda" to="tree-config" /%}

{% /diagram %}

## Creating a Bubblegum Tree

Let's now see how one can create both of these accounts to create a Bubblegum Tree. Fortunately, our libraries make this process easy by providing a **Create Tree** operation that takes care of everything for us. This operation accepts a variety of parameters — most of them optional — that allow us to customize the Bubblegum Tree to our needs. The most important ones are:

- **Merkle Tree**: A newly generated signer that will be used to create the Merkle Tree account. The Merkle Tree account will then be accessible at this address.
- **Tree Creator**: The address of the account that will be able to manage the Bubblegum Tree and mint Compressed NFTs.
- **Max Depth** and **Max Buffer Size**: The **Max Depth** parameter is used to compute the maximum number of leaves — and therefore Compressed NFTs — that the Merkle Tree can hold. This maximum is calculated by `2^maxDepth`. The **Max Buffer Size** parameter indicates the minimum concurrency limit of the Merkle Tree. In other words, it defines how many changes can happen in the tree in parallel. These two parameters cannot be chosen arbitrarily and have to be selected from a pre-defined set of values as displayed in the table below.

  {% totem %}
  {% totem-accordion title="Max Depth / Max Buffer Size Table" %}

  | Max Depth | Max Buffer Size | Max Number of cNFTs |
  | --------- | --------------- | ------------------- |
  | 3         | 8               | 8                   |
  | 5         | 8               | 32                  |
  | 14        | 64              | 16,384              |
  | 14        | 256             | 16,384              |
  | 14        | 1,024           | 16,384              |
  | 14        | 2,048           | 16,384              |
  | 15        | 64              | 32,768              |
  | 16        | 64              | 65,536              |
  | 17        | 64              | 131,072             |
  | 18        | 64              | 262,144             |
  | 19        | 64              | 524,288             |
  | 20        | 64              | 1,048,576           |
  | 20        | 256             | 1,048,576           |
  | 20        | 1,024           | 1,048,576           |
  | 20        | 2,048           | 1,048,576           |
  | 24        | 64              | 16,777,216          |
  | 24        | 256             | 16,777,216          |
  | 24        | 512             | 16,777,216          |
  | 24        | 1,024           | 16,777,216          |
  | 24        | 2,048           | 16,777,216          |
  | 26        | 512             | 67,108,864          |
  | 26        | 1,024           | 67,108,864          |
  | 26        | 2,048           | 67,108,864          |
  | 30        | 512             | 1,073,741,824       |
  | 30        | 1,024           | 1,073,741,824       |
  | 30        | 2,048           | 1,073,741,824       |

  {% /totem-accordion %}
  {% /totem %}

- **Public**: Whether or not the Bubblegum Tree should be public. If it is public, anyone will be able to mint Compressed NFTs from it. Otherwise, only the Tree Creator or the Tree Delegate (as discussed in [Delegating cNFTs](/bubblegum/delegate-cnfts)) will be able to mint Compressed NFTs.

Here is how one can create a Bubblegum Tree using our libraries:

{% dialect-switcher title="Create a Bubblegum Tree" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import { createTree } from '@metaplex-foundation/mpl-bubblegum'

const merkleTree = generateSigner(umi)
const builder = await createTree(umi, {
  merkleTree,
  maxDepth: 14,
  maxBufferSize: 64,
})
await builder.sendAndConfirm(umi)
```

By default, the Tree Creator is set to the Umi identity and the Public parameter is set to `false`. However, these parameters can be customized as shown in the example below.

```ts
const customTreeCreator = generateSigner(umi)
const builder = await createTree(umi, {
  // ...
  treeCreator: customTreeCreator,
  public: true,
})
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Fetching a Bubblegum Tree

Since a **Bubblegum Tree** is composed of two on-chain accounts, let's see how to fetch either of them.

### Fetching a Merkle Tree

The Merkle Tree account contains various information about the tree such as:

- The **Tree Header** which stores the **Max Depth**, the **Max Buffer Size**, the **Authority** of the tree and the **Creation Slot** of when the tree was created.
- The **Tree** itself which stores low-level information about the tree such as its **Change Logs** (or roots), its **Sequence Number**, etc. We talk more about Concurrent Merkle Trees in a [dedicated page](/bubblegum/concurrent-merkle-trees) of this documentation.
- The **Canopy** as discussed in the [Merkle Tree Canopy](/bubblegum/merkle-tree-canopy) page.

Here is how one can fetch all of that data using our libraries:

{% dialect-switcher title="Fetch a Merkle Tree" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchMerkleTree } from '@metaplex-foundation/mpl-bubblegum'

const merkleTreeAccount = await fetchMerkleTree(umi, merkleTree)
```

{% /dialect %}
{% /dialect-switcher %}

### Fetching a Tree Config

The Tree Config account contains data specific to Compressed NFTs. It stores:

- The **Tree Creator** of the Bubblegum Tree.
- The **Tree Delegate** of the Bubblegum Tree, if any. Otherwise, it is set to the **Tree Creator**.
- The **Total Capacity** of the Bubblegum Tree which is the maximum number of cNFTs that can be minted from the tree.
- The **Number Minted** which keeps track of the number of cNFTs minted into the tree. This value is important as it is used as a **Nonce** ("number used once") value for operations to ensure the Merkle tree leaves are unique. Thus, this nonce acts as a tree-scoped unique identifier of the asset.
- The **Is Public** parameter which indicates whether or not anyone can mint cNFTs from the tree.

Here is how one can fetch all of that data using our libraries:

{% dialect-switcher title="Fetch a Tree Config" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { fetchTreeConfigFromSeeds } from '@metaplex-foundation/mpl-bubblegum'

const treeConfig = await fetchTreeConfigFromSeeds(umi, { merkleTree })
```

{% /dialect %}
{% /dialect-switcher %}
