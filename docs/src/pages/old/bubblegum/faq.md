---
title: FAQ
metaTitle: Bubblegum - FAQ
description: Frequently asked questions about Bubblegum
---

## How do I find the arguments needed for operations such as transfer, delegate, burn, etc? {% #replace-leaf-instruction-arguments %}

Whenever we use an instruction that ends up replacing a leaf in the Bubblegum Tree — such as transfer, delegate, burn, etc. — the program requires a bunch of parameters that are used to ensure the current leaf is valid and can be updated. This is because the data of Compressed NFTs is not available inside on-chain accounts and therefore additional parameters such as the **Proof**, the **Leaf Index**, the **Nonce** and more are required for the program to fill the pieces.

All of that information can be retrieved from the **Metaplex DAS API** using both the `getAsset` and the `getAssetProof` RPC methods. However, the RPC responses from these methods and the parameters expected by the instructions are not exactly the same and parsing from one to the other is not trivial.

Fortunately, our SDKs provide a helper method that will do all the heavy lifting for us, as we can see in the code examples below. It accepts the Asset ID of the Compressed NFT and returns a bunch of parameters that can be directly injected into instructions that replace the leaf — such as burn, transfer, update, etc.

That being said, if you ever needed to do that parsing yourself, here is a quick breakdown of the parameters expected by the instructions and how to retrieve them from the Metaplex DAS API. Here we will assume the result of the `getAsset` and `getAssetProof` RPC methods are accessible via the `rpcAsset` and `rpcAssetProof` variables respectively.

- **Leaf Owner**: Accessible via `rpcAsset.ownership.owner`.
- **Leaf Delegate**: Accessible via `rpcAsset.ownership.delegate` and should default to `rpcAsset.ownership.owner` when null.
- **Merkle Tree**: Accessible via `rpcAsset.compression.tree` or `rpcAssetProof.tree_id`.
- **Root**: Accessible via `rpcAssetProof.root`.
- **Data Hash**: Accessible via `rpcAsset.compression.data_hash`.
- **Creator Hash**: Accessible via `rpcAsset.compression.creator_hash`.
- **Nonce**: Accessible via `rpcAsset.compression.leaf_id`.
- **Index**: Accessible via `rpcAssetProof.node_index - 2^max_depth` where `max_depth` is the maximum depth of the tree and can be inferred from the length of the `rpcAssetProof.proof` array.
- **Proof**: Accessible via `rpcAssetProof.proof`.
- **Metadata**: Currently needs to be reconstructed from various fields in the `rpcAsset` response.

{% dialect-switcher title="Get parameters for instructions that replace leaves" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

The Bubblegum Umi library provides a `getAssetWithProof` helper method that fits the description above. Here's an example of how to use it using the `transfer` instruction. Note that, in this case, we override the `leafOwner` parameter as it needs to be a Signer and `assetWithProof` gives us the owner as a Public Key.

```ts
import { getAssetWithProof, transfer } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await transfer(umi, {
  ...assetWithProof,
  leafOwner: leafOwnerA, // As a signer.
  newLeafOwner: leafOwnerB.publicKey,
}).sendAndConfirm(umi)
```

{% totem-accordion title="Get parameters without the helper function" %}

For completeness, here's how we could achieve the same result without using the provided helper function.

```ts
import { publicKeyBytes } from '@metaplex-foundation/umi'
import { transfer } from '@metaplex-foundation/mpl-bubblegum'

const rpcAsset = await umi.rpc.getAsset(assetId)
const rpcAssetProof = await umi.rpc.getAssetProof(assetId)

await transfer(umi, {
  leafOwner: leafOwnerA,
  newLeafOwner: leafOwnerB.publicKey,
  merkleTree: rpcAssetProof.tree_id,
  root: publicKeyBytes(rpcAssetProof.root),
  dataHash: publicKeyBytes(rpcAsset.compression.data_hash),
  creatorHash: publicKeyBytes(rpcAsset.compression.creator_hash),
  nonce: rpcAsset.compression.leaf_id,
  index: rpcAssetProof.node_index - 2 ** rpcAssetProof.proof.length,
  proof: rpcAssetProof.proof,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
