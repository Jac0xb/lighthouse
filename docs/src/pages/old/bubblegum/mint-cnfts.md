---
title: Minting Compressed NFTs
metaTitle: Bubblegum - Minting Compressed NFTs
description: Learn how to mint compressed NFTs on Bubblegum
---

In [the previous page](/bubblegum/create-trees), we saw that we need a Bubblegum Tree to mint Compressed NFTs and we saw how to create one. Now, let's see how to mint compressed NFTs from a given Bubblegum Tree. {% .lead %}

The Bubblegum program offers two minting instructions. One that mints NFTs without associating them with a collection and one that mints NFTs to a given collection. Let's start by looking at the former since the latter simply requires a few more parameters.

## Minting without a Collection

The Bubblegum program provides a **Mint V1** instruction that enables us to mint Compressed NFTs from a Bubblegum Tree. If the Bubblegum Tree is public, anyone will be able to use this instruction. Otherwise, only the Tree Creator or the Tree Delegate will be able to do so.

The main parameters of the Mint V1 instruction are:

- **Merkle Tree**: The Merkle Tree address from which the Compressed NFT will be minted.
- **Tree Creator Or Delegate**: The authority allowed to mint from the Bubblegum Tree — this can either be the creator or the delegate of the tree. This authority must sign the transaction. In the case of a public tree, this parameter can be any authority but must still be a signer.
- **Leaf Owner**: The owner of the Compressed NFT that will be minted.
- **Leaf Delegate**: An delegate authority allowed to manage the minted cNFT, if any. Otherwise, it is set to the Leaf Owner.
- **Metadata**: The metadata of the Compressed NFT that will be minted. It contains information such as the **Name** of the NFT, its **URI**, its **Collection**, its **Creators**, etc.
  - Note that is it possible to provide a **Collection** object within the metadata but its **Verified** field will have to be set to `false` since the Collection Authority is not requested in this instruction and therefore cannot sign the transaction.
  - Also note that creators can verify themselves on the cNFT when minting. To make this work, we need to set the **Verified** field of the **Creator** object to `true` and add the creator as a Signer in the remaining accounts. This can happen for multiple creators as long as they all sign the transaction and are added as remaining accounts.

{% dialect-switcher title="Mint a Compressed NFT without a Collection" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { none } from '@metaplex-foundation/umi'
import { mintV1 } from '@metaplex-foundation/mpl-bubblegum'

await mintV1(umi, {
  leafOwner,
  merkleTree,
  metadata: {
    name: 'My Compressed NFT',
    uri: 'https://example.com/my-cnft.json',
    sellerFeeBasisPoints: 500, // 5%
    collection: none(),
    creators: [
      { address: umi.identity.publicKey, verified: false, share: 100 },
    ],
  },
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

### Get leaf schema from mint transaction {% #get-leaf-schema-from-mint-transaction %}

You can retrieve the leaf and determine the asset ID from the `mintV1` transaction using the `parseLeafFromMintV1Transaction` helper.

{% dialect-switcher title="Get leaf schema from mint transaction" %}
{% dialect title="JavaScript" id="js" %}

```ts
import {
    findLeafAssetIdPda,
    parseLeafFromMintV1Transaction
} from "@metaplex-foundation/mpl-bubblegum";

const { signature } = await mintV1(umi, { leafOwner, merkleTree, metadata }).sendAndConfirm(umi, { confirm: { commitment: 'confirmed' } });
const leaf: LeafSchema = await parseLeafFromMintV1Transaction(umi, signature);
const assetId = findLeafAssetIdPda(umi, { merkleTree, leafIndex: leaf.nonce });
```

{% /dialect %}
{% /dialect-switcher %}

## Minting to a Collection

Whilst it is possible to set and verify a Collection for a Compressed NFT _after_ it was minted, the Bubblegum program provides a convenient instruction to mint a Compressed NFT directly to a given Collection. This instruction is called **Mint To Collection V1** and it uses the same parameters as the **Mint V1** instruction, with the addition of the following parameters:

- **Collection Mint**: The Mint address of the Collection NFT to which the Compressed NFT will be part.
- **Collection Authority**: The authority allowed to manage the given Collection NFT. This can either be the update authority of the Collection NFT or a delegated collection authority. This authority must sign the transaction regardless of whether the Bubblegum Tree is public or not.
- **Collection Authority Record Pda**: When using a delegated collection authority, the Delegate Record PDA must be provided to ensure the authority is allowed to manage the Collection NFT. This can either be using the new "Metadata Delegate" PDA or the legacy "Collection Authority Record" PDA.

Additionally, note that the **Metadata** parameter must contain a **Collection** object such that:

- Its **Address** field matches the **Collection Mint** parameter.
- Its **Verified** field can be passed in as either `true` or `false`. If it is passed in as `false`, it will be set to `true` during the transaction and the cNFT will be minted with **Verified** set to `true`.

Also note that, just like in the **Mint V1** instruction, creators can verify themselves by signing the transaction and adding themselves as remaining accounts.

{% dialect-switcher title="Mint a Compressed NFT to a Collection" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { none } from '@metaplex-foundation/umi'
import { mintToCollectionV1 } from '@metaplex-foundation/mpl-bubblegum'

await mintToCollectionV1(umi, {
  leafOwner,
  merkleTree,
  collectionMint,
  metadata: {
    name: 'My Compressed NFT',
    uri: 'https://example.com/my-cnft.json',
    sellerFeeBasisPoints: 500, // 5%
    collection: { key: collectionMint, verified: false },
    creators: [
      { address: umi.identity.publicKey, verified: false, share: 100 },
    ],
  },
}).sendAndConfirm(umi)
```

By default, the Collection Authority is set to the Umi identity but this can be customized as shown in the example below.

```ts
const customCollectionAuthority = generateSigner(umi)
await mintToCollectionV1(umi, {
  // ...
  collectionAuthority: customCollectionAuthority,
})
```

{% totem-accordion title="Create a Collection NFT" %}

If you do not have a Collection NFT yet, you can create one using the `@metaplex-foundation/mpl-token-metadata` library.

```shell
npm install @metaplex-foundation/mpl-token-metadata
```

And create a Collection NFT like so:

```ts
import { generateSigner, percentAmount } from '@metaplex-foundation/umi'
import { createNft } from '@metaplex-foundation/mpl-token-metadata'

const collectionMint = generateSigner(umi)
await createNft(umi, {
  mint: collectionMint,
  name: 'My Collection',
  uri: 'https://example.com/my-collection.json',
  sellerFeeBasisPoints: percentAmount(5.5), // 5.5%
  isCollection: true,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

### Get leaf schema from mint to collection transaction {% #get-leaf-schema-from-mint-to-collection-transaction %}

Again you can retrieve the leaf and determine the asset ID from the `mintToCollectionV1` transaction using the `parseLeafFromMintToCollectionV1Transaction` helper.

{% dialect-switcher title="Get leaf schema from mint to collection transaction" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { parseLeafFromMintToCollectionV1Transaction } from '../src';

const { signature } = await mintToCollectionV1(umi, {
  leafOwner,
  merkleTree,
  metadata,
  collectionMint: collectionMint.publicKey,
}).sendAndConfirm(umi);

const leaf: LeafSchema = await parseLeafFromMintToCollectionV1Transaction(umi, signature);
const assetId = findLeafAssetIdPda(umi, { merkleTree, leafIndex: leaf.nonce });
```

{% /dialect %}
{% /dialect-switcher %}
