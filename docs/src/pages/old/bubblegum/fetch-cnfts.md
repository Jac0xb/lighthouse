---
title: Fetching Compressed NFTs
metaTitle: Bubblegum - Fetching Compressed NFTs
description: Learn how to fetch compressed NFTs on Bubblegum
---

As mentioned in the [Overview](/bubblegum#read-api) page, Compressed NFTs are not stored inside on-chain accounts like regular NFTs but, instead, are logged in the transactions that created and updated them. {% .lead %}

As such, a special indexer was created to facilitate the retrieval of Compressed NFTs. This indexed data is made available through an extension of the Solana RPC methods which we call the **Metaplex DAS API**. In fact, the DAS API allows us to fetch any **Digital Asset**. This can be a Compressed NFT, a regular NFT, or even a Fungible Asset.

Since not all RPCs support the DAS API, you will need to choose your RPC provider carefully if you are planning to work with Compressed NFTs. Note that we maintain a list of all RPCs that support the Metaplex DAS API [in a dedicated page](/bubblegum/rpcs).

On this page, we will learn how to fetch Compressed NFTs using the Metaplex DAS API.

## Installing the Metaplex DAS API SDK

Once you have chosen an RPC provider that supports the Metaplex DAS API, you may simply send special RPC methods to fetch Compressed NFTs. However, our SDKs provide a more convenient way to get started with the DAS API by offering helper methods. Follow the instructions below to get started with the Metaplex DAS API using our SDK.

{% totem %}

{% dialect-switcher title="Get started with the Metaplex DAS API" %}
{% dialect title="JavaScript" id="js" %}

{% totem-prose %}
When using Umi, the Metaplex DAS API plugin is automatically installed within the `mplBubblegum` plugin. So you are already be good to go!

If you wanted to use the DAS API plugin _without_ importing the whole `mplBubblegum` plugin, you could do so by installing the Metaplex DAS API plugin directly:

```sh
npm install @metaplex-foundation/digital-asset-standard-api
```

After that,  register the library with your Umi instance:

```ts
import { dasApi } from '@metaplex-foundation/digital-asset-standard-api';

umi.use(dasApi());
```
{% /totem-prose %}
{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

{% totem-prose %}

You can find more information about the methods available on the Metaplex DAS API on its [repository](https://github.com/metaplex-foundation/digital-asset-standard-api).

{% /totem-prose %}
{% /totem %}

## Asset IDs {% #asset-ids %}

In order to fetch an NFT, compressed or not, we need to have access to a unique ID that identifies the NFT. We call this unique identifier the **Asset ID**.

- For regular NFTs, we use the **mint address of the NFT** for that purpose since all other accounts simply derive from that address.
- For compressed NFTs, we use a special **PDA** (Program Derived Address) that is derived from the **address of the Merkle Tree** and the **leaf index** of the Compressed NFT in the Merkle tree. We call this special PDA a **Leaf Asset ID**.

You typically shouldn't need to derive the **Leaf Asset ID** yourself since the DAS API methods will provide it for you when fetching Compressed NFTs in bulk — e.g. fetching all NFTs owned by a given address. However, if you had access to the address of the Merkle Tree and the leaf index of the cNFT, here's how you could use our SDKs to derive the Leaf Asset ID.

{% dialect-switcher title="Find the Leaf Asset ID PDA" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { findLeafAssetIdPda } from '@metaplex-foundation/mpl-bubblegum'

const [assetId, bump] = await findLeafAssetIdPda(umi, {
  merkleTree,
  leafIndex,
})
```

{% /dialect %}
{% /dialect-switcher %}

## Fetching a Compressed NFT

Fetching a Compressed NFT is as simple as calling the `getAsset` method of the DAS API. This method will return an **Rpc Asset** object that contains the following information:

- **Id**: The Asset ID as discussed above.
- **Interface**: A special value that defines the type of asset we are dealing with. E.g. `V1_NFT` OR `ProgrammableNFT`.
- **Ownership**: An object telling us who owns the asset. This includes any delegate that may have been set and whether or not the asset is marked as frozen.
- **Mutable**: A boolean indicating whether the data of the asset is updatable or not.
- **Authorities**: An array of authorities, each including a scope array indicating what operations the authority is allowed to perform on the asset.
- **Content**: An object containing the data of the asset. Namely, it includes its URI and a parsed `metadata` object.
- **Royalty**: An object that defines the royalty model defined by the asset. Currently, there is only one royalty model supported which sends a percentage of the proceeds to the creator(s) of the asset.
- **Supply**: When dealing with printable assets, this object provides the current and max supply of printed editions.
- **Creators**: The list of creators of the asset. Each includes a `verified` boolean indicating whether the creator has been verified or not and a `share` number indicating the percentage of royalties that should be sent to the creator.
- **Grouping**: An array of key/value grouping mechanisms that can help index and retrieve assets in bulk. Currently, only one grouping mechanism is supported — `collection` — which allows us to group assets by collection.
- **Compression**: When dealing with Compressed NFTs, this object gives us various information about the leaf of the Bubblegum Tree. For instance, it provides the full hash of the leaf, but also partial hashes such as the **Creator Hash** and **Data Hash** which are used to verify the authenticity of the asset. It also gives us the Merkle Tree address, its root, sequence, etc.

Here is how one can fetch an asset from a given Asset ID using our SDKs.

{% dialect-switcher title="Fetch a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}

```ts
const rpcAsset = await umi.rpc.getAsset(assetId)
```

{% /dialect %}
{% /dialect-switcher %}

## Fetching the Proof of a Compressed NFT

Whilst the `getAsset` RPC method returns a whole lot of information about the asset, it does not return the **Proof** of the asset. As mentioned, in the [Overview](/bubblegum#merkle-trees-leaves-and-proofs) page, the Proof of a Compressed NFT is a list of hashes that allow us to verify the authenticity of the asset. Without it, anyone could pretend that they have a Compressed NFT in a tree with any given data.

As such, many operations on Compressed NFTs — e.g. burning, transferring, updating, etc. — require the Proof of the asset before allowing us to perform them. Computing the Proof of an asset is possible but requires someone to know the hash of all Compressed NFTs that exist within a given tree. This is why the DAS API also keeps track of the Proof of all Compressed NFTs.

In order to access the Proof of a Compressed NFT, we may use the `getAssetProof` RPC method. This method will return an **Rpc Asset Proof** object containing the following information:

- **Proof**: The proof of the Compressed NFT as promised.
- **Root**: The root of the Merkle Tree that the asset belongs to. When verifying the asset using the provided Proof, we should end up with this root as the final hash.
- **Node Index**: The index of the asset in the Merkle Tree if we counted every single node in the tree from left to right, top to bottom. A more useful index called the **Leaf Index** can be inferred from this value by the following formula: `leaf_index = node_index - 2^max_depth` where `max_depth` is the maximum depth of the Merkle Tree. The **Leaf Index** is the index of the asset in the Merkle Tree if we counted only the leaves of the tree — i.e. the lowest row — from left to right. This index is requested by many instructions and is used to derive the **Leaf Asset ID** of the asset.
- **Leaf**: The full hash of the Compressed NFT.
- **Tree ID**: The address of the Merkle Tree that the asset belongs to.

As you can see some of the information here is redundant from the `getAsset` RPC call but is provided here for convenience. However, the **Proof** and the **Node Index** of the asset can only be fetched through this method.

Here is how we can fetch the Proof of an asset using our SDKs.

{% dialect-switcher title="Fetch the proof of a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}

```ts
const rpcAssetProof = await umi.rpc.getAssetProof(assetId)
```

{% /dialect %}
{% /dialect-switcher %}

## Fetching Multiple Compressed NFTs

The DAS API also allows us to fetch multiple assets at once using the `getAssetsByOwner` and `getAssetsByGroup` RPC methods. These methods will return a paginated **Rpc Asset List** object containing the following information:

- **Items**: An array of **Rpc Asset** as described above.
- **Total**: The total number of assets available based on the provided criteria.
- **Limit**: The maximum number of assets we are retrieving on a page.
- **Page**: When using numbered pagination, it tells us which page we are currently on.
- **Before** and **After**: When using cursor pagination, it tells us after which and/or before which asset we are currently browsing assets. These cursors can be used to navigate to the previous and next pages.
- **Errors**: A potential list of errors returned by the RPC.

Here is how we can use both of these RPC methods using our SDKs.

### By Owner

{% dialect-switcher title="Fetch Compressed NFTs by owner" %}
{% dialect title="JavaScript" id="js" %}

```ts
const rpcAssetList = await umi.rpc.getAssetsByOwner({ owner })
```

{% /dialect %}
{% /dialect-switcher %}

### By Collection

{% dialect-switcher title="Fetch Compressed NFTs by collection" %}
{% dialect title="JavaScript" id="js" %}

```ts
const rpcAssetList = await umi.rpc.getAssetsByGroup({
  groupKey: 'collection',
  groupValue: collectionMint,
})
```

{% /dialect %}
{% /dialect-switcher %}
