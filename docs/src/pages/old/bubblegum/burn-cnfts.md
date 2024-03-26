---
title: Burning Compressed NFTs
metaTitle: Bubblegum - Burning Compressed NFTs
description: Learn how to burn compressed NFTs on Bubblegum
---

The **Burn** instruction can be used to burn a Compressed NFT and, therefore, remove it from the Bubblegum Tree permanently. To authorize this operation, either the current owner or the delegate authority — if any — must sign the transaction. The instruction accepts the following parameter:

- **Leaf Owner** and **Leaf Delegate**: The current owner of the Compressed NFT and its delegate authority if any. One of these must sign the transaction.

Note that, since this instruction replaces the leaf on the Bubblegum Tree, additional parameters must be provided to verify the integrity of the Compressed NFT before it can be burnt. Since these parameters are common to all instructions that mutate leaves, they are documented [in the following FAQ](/bubblegum/faq#replace-leaf-instruction-arguments). Fortunately, we can use a helper method that will automatically fetch these parameters for us using the Metaplex DAS API.

{% dialect-switcher title="Burn a Compressed NFT" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import { getAssetWithProof, burn } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await burn(umi, {
  ...assetWithProof,
  leafOwner: currentLeafOwner,
}).sendAndConfirm(umi)
```

{% totem-accordion title="Using a delegate" %}

```ts
import { getAssetWithProof, burn } from '@metaplex-foundation/mpl-bubblegum'

const assetWithProof = await getAssetWithProof(umi, assetId)
await burn(umi, {
  ...assetWithProof,
  leafDelegate: currentLeafDelegate,
}).sendAndConfirm(umi)
```

{% /totem-accordion %}

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
