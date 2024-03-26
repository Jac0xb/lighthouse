---
title: Updating Compressed NFTs
metaTitle: Bubblegum - Updating Compressed NFTs
description: Learn how to update compressed NFTs on Bubblegum
---

The **Update** instruction can be used to modify the metadata of a Compressed NFT. The Merkle root is updated to reflect the propagated hash of the data, and RPC providers who conform to the [Metaplex DAS API](https://github.com/metaplex-foundation/digital-asset-standard-api) will update their index of the cNFTs.

The metadata can be updated by one of two authorities, depending on if the compressed NFT is a verified item in a collection.

## As the Tree Owner

If there _is no_ verified collection set, the **Tree Owner** is the authority that is permissioned to call **Update**

{% dialect-switcher title="Update a Compressed NFT as the Tree Owner" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  updateMetadata,
  UpdateArgsArgs,
  getCurrentRoot,
} from '@metaplex-foundation/mpl-bubblegum'

// Then we can use it to update metadata for the NFT.
const updateArgs: UpdateArgsArgs = {
  name: some('New name'),
  uri: some('https://updated-example.com/my-nft.json'),
}

await updateMetadata(umi, {
  leafOwner,
  merkleTree,
  root: getCurrentRoot(merkleTreeAccount.tree),
  nonce: leafIndex,
  index: leafIndex,
  currentMetadata: metadata,
  proof: [],
  updateArgs,
  }).sendAndConfirm(umi);
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

This can also be done using the `getAssetWithProof` helper method:

{% dialect-switcher title="Update a Compressed NFT as the Tree Owner using Helper Method" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  updateMetadata,
  UpdateArgsArgs,
} from '@metaplex-foundation/mpl-bubblegum'

// Use the helper to fetch the proof.
const assetWithProof = await getAssetWithProof(umi, assetId)

// Then we can use it to update metadata for the NFT.
const updateArgs: UpdateArgsArgs = {
  name: some('New name'),
  uri: some('https://updated-example.com/my-nft.json'),
}

await updateMetadata(umi, {
  ...assetWithProof,
  leafOwner,
  currentMetadata: metadata,
  updateArgs,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## As the Collection Update Authority

If the verified collection _is_ set, then the **Collection Update Authority** is the only authority that is permissioned to call **Update**

{% dialect-switcher title="Update a Compressed NFT as the Collection Update Authority" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  updateMetadata,
  UpdateArgsArgs,
  getCurrentRoot,
} from '@metaplex-foundation/mpl-bubblegum'
import { findMetadataPda } from '@metaplex-foundation/mpl-token-metadata'

// Then we can use it to update metadata for the NFT.
const updateArgs: UpdateArgsArgs = {
  name: some('New name'),
  uri: some('https://updated-example.com/my-nft.json'),
}
await updateMetadata(umi, {
  leafOwner,
  merkleTree,
  root: getCurrentRoot(merkleTreeAccount.tree),
  nonce: leafIndex,
  index: leafIndex,
  currentMetadata: metadata,
  proof: [],
  updateArgs,
  authority: collectionAuthority,
  collectionMint: collectionMint.publicKey,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

This can also be done using the `getAssetWithProof` helper method:

{% dialect-switcher title="Update a Compressed NFT as the Collection Update Authority using Helper Method" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  updateMetadata,
  UpdateArgsArgs,
} from '@metaplex-foundation/mpl-bubblegum'
import { findMetadataPda } from '@metaplex-foundation/mpl-token-metadata'

// Use the helper to fetch the proof.
const assetWithProof = await getAssetWithProof(umi, assetId)

// Then we can use it to update metadata for the NFT.
const updateArgs: UpdateArgsArgs = {
  name: some('New name'),
  uri: some('https://updated-example.com/my-nft.json'),
}
await updateMetadata(umi, {
  ...assetWithProof,
  leafOwner,
  currentMetadata: metadata,
  updateArgs,
  authority: collectionAuthority,
  collectionMint: collectionMint.publicKey,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
