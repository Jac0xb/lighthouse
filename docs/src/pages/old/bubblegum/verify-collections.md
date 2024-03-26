---
title: Verifying Collections
metaTitle: Bubblegum - Verifying Collections
description: Learn how to set, verify and unverify collections on Bubblegum
---

Whenever a collection is set on a Compressed NFT, the update authority of the collection — or any approved collection delegate — may verify and/or unverify that collection on the cNFT. {% .lead %}

Technically, this will toggle a **Verified** boolean on the **Collection** object of the cNFT, letting anyone know that an authority of the collection approved this Compressed NFT as being part of the collection.

If you are not familiar with the concept of collections with regard to NFTs, they are special non-compressed NFTs that can be used to group other NFTs together. The data of the **Collection NFT** is therefore used to describe the name and the branding of the entire collection. You can [read more about Metaplex Certified Collections here](https://docs.metaplex.com/programs/token-metadata/certified-collections).

Note that is possible to mint a Compressed NFT directly into a collection by using the **Mint to Collection V1** instruction [documented here](/bubblegum/mint-cnfts#minting-to-a-collection). That being said, if you have already minted a cNFT without a collection, let's see how we can verify, unverify but also set the collection on that cNFT.

## Verify a Collection

The **Verify Collection** instruction of the Bubblegum program can be used to set the **Verified** boolean of a Compressed NFT to `true`. In order for this to work, the **Collection** object must have already been set on the cNFT — for instance, when it was minted.

The instruction accepts the following parameters:

- **Collection Mint**: The mint account of the Collection NFT.
- **Collection Authority**: The update authority of the Collection NFT — or an approved collection delegate — as a Signer. In case the collection authority is a delegate authority, note that the program supports both the new unified **Metadata Delegate** system and the legacy **Collection Authority Records** accounts. Simply pass the approriate PDA to the **Collection Authority Record Pda** parameter.

Additionally, more parameters must be provided to verify the integrity of the Compressed NFT as this instruction will end up replacing the leaf on the Bubblegum Tree. Since these parameters are common to all instructions that mutate leaves, they are documented [in the following FAQ](/bubblegum/faq#replace-leaf-instruction-arguments). Fortunately, we can use a helper method that will automatically fetch these parameters for us using the Metaplex DAS API.

{% dialect-switcher title="Verify the Collection of a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  verifyCollection,
} from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await verifyCollection(umi, {
  ...assetWithProof,
  collectionMint,
  collectionAuthority,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Set and Verify a Collection

If the **Collection** object has not been set on the Compressed NFT yet, the **Set and Verify Collection** instruction can be used to set it and verify it at the same time. This instruction accepts the same parameters as the **Verify Collection** instruction but also requires the **Tree Creator Or Delegate** attribute to be passed as a Signer if it is different than the collection authority.

{% dialect-switcher title="Set and Verify the Collection of a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  setAndVerifyCollection,
} from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await setAndVerifyCollection(umi, {
  ...assetWithProof,
  treeCreatorOrDelegate,
  collectionMint,
  collectionAuthority,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Unverify a Collection

The update authority of a collection can also unverify the collection of a Compressed NFT by using the **Unverify Collection** instruction. In order to send this instruction, the **Collection** object of the cNFT is expected to already be set and verified. The attributes required by the **Unverify Collection** instruction are the same as the ones required by the **Verify Collection** instruction.

{% dialect-switcher title="Unverify the Collection of a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  unverifyCollection,
} from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await unverifyCollection(umi, {
  ...assetWithProof,
  collectionMint,
  collectionAuthority,
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
