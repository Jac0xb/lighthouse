---
title: Verified Creators
metaTitle: Token Metadata - Verified Creators
description: Learn how to verify the creators of an Asset on Token Metadata
---

Similarly to [collections](/token-metadata/collections), the creators of an asset must be verified to ensure the authenticity of the asset. {% .lead %}

A creator whose `verified` flag is `false` could have been added by anyone and, therefore, cannot be trusted. On the other hand, a creator whose `verified` flag is `true` is guaranteed to have signed a transaction that verified them as a creator of that asset.

In the section below, we will learn how to verify and unverify the creators of an asset. Note that before verifying a creator, it must already be part of the **Creators** array of the asset's **Metadata** account. This can be done when [minting the asset](/token-metadata/mint) but also when [updating the asset](/token-metadata/update).

## Verify a Creator

The **Verify** instruction can be used to verify the creator of an asset. Notice that the same instruction can also be used to verify collections providing we pass different arguments to the instruction. Some of our SDKs split these instructions into multiple helpers like `verifyCreatorV1` and `verifyCollectionV1` to provide a better developer experience.

The main attributes required by the **Verify** instruction in the context of verifying a creator are the following:

- **Metadata**: The address of the asset's **Metadata** account.
- **Authority**: The creator we are trying to verify as a Signer.

Here's how we can use our SDKs to verify a creator on Token Metadata.

{% dialect-switcher title="Verify a Creator" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { verifyCreatorV1 } from '@metaplex-foundation/mpl-token-metadata'

await verifyCreatorV1(umi, {
  metadata,
  authority: creator,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

## Unverify a Creator

Reciprocally, the **Unverify** instruction can be used to turn the `verified` flag of a creator to `false`. It accepts the same attributes as the **Verify** instruction and can be used in the same way.

{% dialect-switcher title="Unverify a Creator" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { unverifyCreatorV1 } from '@metaplex-foundation/mpl-token-metadata'

await unverifyCreatorV1(umi, {
  metadata,
  authority: creator,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}
