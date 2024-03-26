---
title: Updating Assets
metaTitle: Token Metadata - Updating Assets
description: Learn how to update Assets on Token Metadata
---

The update authority of an asset can update its **Metadata** account using the **Update** instruction as long as the **Is Mutable** attribute is set to `true`. The **Update** instruction requires the **Update Authority** to sign the transaction and can update the following attributes of the **Metadata** account:

- **Data**: The object that defines the Name, Symbol, URI, Seller Fee Basis Points and the array of Creators of the asset. Note that the update authority can only add and/or remove unverified creators from the Creators array. The only exception is if the creator is the update authority, in which case the added or removed creators can be verified.
- **Primary Sale Happened**: A boolean that indicates whether the asset has been sold before.
- **Is Mutable**: A boolean that indicates whether the asset can be updated again. When changing this to `false`, any future updates will fail.
- **Collection**: This attribute enables us to set or clear the collection of the asset. Note that when setting a new collection, the `verified` boolean must be set to `false` and [verified using another instruction](/token-metadata/collections).
- **Collection Details**: This attribute enables us to set or clear the collection details of the asset.
- **Rule Set**: This attribute enables us to set or clear the rule set of the asset. This is only relevant for [Programmable Non-Fungibles](/token-metadata/pnfts).

Note that certain delegated authorities can also update the **Metadata** account of assets as discussed in the "[Delegated Authorities](/token-metadata/delegates)" page.

Here is how you can use our SDKs to update an asset on Token Metadata.

{% dialect-switcher title="Update Assets" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
import {
  updateV1,
  fetchMetadataFromSeeds,
} from '@metaplex-foundation/mpl-token-metadata'

const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })
await updateV1(umi, {
  mint,
  authority: updateAuthority,
  data: { ...initialMetadata, name: 'Updated Asset' },
}).sendAndConfirm(umi)
```

If you want to update more than just the **Data** attribute of the **Metadata** account, simply provide these attributes to the `updateV1` method.

```ts
import {
  updateV1,
  fetchMetadataFromSeeds,
} from '@metaplex-foundation/mpl-token-metadata'

const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })
await updateV1(umi, {
  mint,
  authority: updateAuthority,
  data: { ...initialMetadata, name: 'Updated Asset' },
  primarySaleHappened: true,
  isMutable: true,
  // ...
}).sendAndConfirm(umi)
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}
