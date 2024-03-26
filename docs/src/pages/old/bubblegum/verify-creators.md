---
title: Verifying Creators
metaTitle: Bubblegum - Verifying Creators
description: Learn how to verify and unverify creators on Bubblegum
---

If a Compressed NFT has a list of creators set in its metadata, these creators can use special instructions to verify and unverify themselves on the cNFT. {% .lead %}

These instructions will toggle a **Verified** boolean on the appropriate item of the cNFT's **Creators** array. That boolean is important as it allows apps such as wallets and marketplaces to know which creators are genuine and which ones are not.

It is worth noting that creators can verify themselves directly when [minting the Compressed NFT](/bubblegum/mint-cnfts) by signing the mint transaction. That being said, let's now see how a creator can verify or unverify themselves on an existing Compressed NFT.

## Verify a Creator

The Bubblegum program offers a **Verify Creator** instruction that must be signed by the creator we are trying to verify.

Additionally, more parameters must be provided to verify the integrity of the Compressed NFT as this instruction will end up replacing the leaf on the Bubblegum Tree. Since these parameters are common to all instructions that mutate leaves, they are documented [in the following FAQ](/bubblegum/faq#replace-leaf-instruction-arguments). Fortunately, we can use a helper method that will automatically fetch these parameters for us using the Metaplex DAS API.

{% dialect-switcher title="Verify the Creator of a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  verifyCreator,
} from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await verifyCreator(umi, { ...assetWithProof, creator }).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Unverify a Creator

Similarly to the **Verify Creator** instruction, the **Unverify Creator** instruction must be signed by the creator and will unverify them on the Compressed NFT.

{% dialect-switcher title="Unverify the Creator of a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  getAssetWithProof,
  unverifyCreator,
} from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await unverifyCreator(umi, { ...assetWithProof, creator }).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
