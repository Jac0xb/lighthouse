---
title: Transferring Compressed NFTs
metaTitle: Bubblegum - Transferring Compressed NFTs
description: Learn how to transfer compressed NFTs on Bubblegum
---

The **Transfer** instruction can be used to transfer a Compressed NFT from one owner to another. To authorize the transfer, either the current owner or the delegate authority — if any — must sign the transaction. The instruction accepts the following parameters:

- **Leaf Owner** and **Leaf Delegate**: The current owner of the Compressed NFT and its delegate authority if any. One of these must sign the transaction.
- **New Leaf Owner**: The address of the Compressed NFT's new owner.

Note that this instruction updates the Compressed NFT and therefore replaces the leaf on the Bubblegum Tree. This means additional parameters must be provided to verify the integrity of the Compressed NFT. Since these parameters are common to all instructions that mutate leaves, they are documented [in the following FAQ](/bubblegum/faq#replace-leaf-instruction-arguments). Fortunately, we can use a helper method that will automatically fetch these parameters for us using the Metaplex DAS API.

{% dialect-switcher title="Transfer a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { getAssetWithProof, transfer } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await transfer(umi, {
  ...assetWithProof,
  leafOwner: currentLeafOwner,
  newLeafOwner: newLeafOwner.publicKey,
}).sendAndConfirm(umi)
```

{% totem-accordion title="Using a delegate" %}

```ts
import { getAssetWithProof, transfer } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await transfer(umi, {
  ...assetWithProof,
  leafDelegate: currentLeafDelegate,
  newLeafOwner: newLeafOwner.publicKey,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
