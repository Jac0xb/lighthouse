---
title: Burning Assets
metaTitle: Token Metadata - Burning Assets
description: Learn how to burn Assets on Token Metadata
---

The owner of an asset can burn it using the **Burn** instruction of the Token Metadata program. This will close all possible accounts associated with the asset and return the various rent-exempt fees to the owner. This instruction accepts the following attributes:

- **Authority**: The signer that authorizes the burn. Typically, this is the owner of the asset but note that certain delegated authorities can also burn assets on behalf of the owner as discussed in the "[Delegated Authorities](/token-metadata/delegates)" page.
- **Token Owner**: The public key of the current owner of the asset.
- **Token Standard**: The standard of the asset being burnt. This instruction works for all Token Standards in order to provide a unified interface for burning assets. That being said, it is worth noting that non-programmable assets can be burnt using the **Burn** instruction of the SPL Token program directly.

The exact accounts closed by the **Burn** instruction depend on the Token Standard of the asset being burnt. Here's a table that summarizes the accounts for each Token Standard:

| Token Standard                 | Mint | Token                      | Metadata | Edition | Token Record | Edition Marker                    |
| ------------------------------ | ---- | -------------------------- | -------- | ------- | ------------ | --------------------------------- |
| `NonFungible`                  | ❌   | ✅                         | ✅       | ✅      | ❌           | ❌                                |
| `NonFungibleEdition`           | ❌   | ✅                         | ✅       | ✅      | ❌           | ✅ if all prints for it are burnt |
| `Fungible` and `FungibleAsset` | ❌   | ✅ if all tokens are burnt | ❌       | ❌      | ❌           | ❌                                |
| `ProgrammableNonFungible`      | ❌   | ✅                         | ✅       | ✅      | ✅           | ❌                                |

Note that the Mint account is never closed because the SPL Token program does not allow it.

Here is how you can use our SDKs to burn an asset on Token Metadata.

{% dialect-switcher title="Burning Assets" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { burnV1 } from '@metaplex-foundation/mpl-token-metadata'

await burnV1(umi, {
  mint,
  authority: owner,
  tokenOwner: owner.publicKey,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}
