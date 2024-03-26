---
title: Printing Editions
metaTitle: Token Metadata - Printing Editions
description: Learn how to print NFT editions on Token Metadata
---

Every NFT has the potential to be printed as multiple editions when its **Master Edition** account is configured accordingly. On this page, we'll learn how to create a printable NFT and how to print editions from it.

## Printable NFTs

The owner of a printable NFT can print as many editions as they want from it as long as its maximum supply has not been reached.

Every Non-Fungible asset — i.e. `NonFungible` and `ProgrammableNonFungible` token standards — can be a printable NFT when created. This is done by configuring the **Max Supply** attribute of the asset's Master Edition account. This attribute is optional and can have one of the following values:

- `None`: The NFT does not have a fixed supply. In other words, **the NFT is printable and has an unlimited supply**.
- `Some(x)`: The NFT has a fixed supply of `x` editions.
  - When `x = 0`, this means **the NFT is not printable**.
  - When `x > 0`, this means **the NFT is printable and has a fixed supply of `x` editions**.

Whenever a new printed edition is created from a printable NFT, a few things happen:

- A brand new edition NFT is created and its data matches the original NFT. The only difference is that the printed edition uses a different token standard than the original NFT.
  - For a `NonFungible` asset, the printed editions use the `NonFungibleEdition` token standard.
  - For a `ProgrammableNonFungible` asset, the printed editions use the `ProgrammableNonFungibleEdition` token standard.
- Instead of using a **Master Edition** account, the new edition NFT uses an **Edition** account which keeps track of its edition number and its parent NFT by storing the address of its parent's **Master Edition** account.
- The **Supply** attribute of the Master edition account is incremented by 1. When the **Supply** attribute reaches the **Max Supply** attribute, the NFT is no longer printable.

{% diagram height="h-64 md:h-[500px]" %}
{% node %}
{% node #wallet label="Wallet Account" theme="indigo" /%}
{% node label="Owner: System Program" theme="dimmed" /%}
{% /node %}

{% node x="200" parent="wallet" %}
{% node #token label="Token Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node label="Amount = 1" /%}
{% /node %}

{% node x="200" parent="token" %}
{% node #mint label="Mint Account" theme="blue" /%}
{% node label="Owner: Token Program" theme="dimmed" /%}
{% node #mint-authority label="Mint Authority = Edition" /%}
{% node label="Supply = 1" /%}
{% node label="Decimals = 0" /%}
{% node #freeze-authority label="Freeze Authority = Edition" /%}
{% /node %}

{% node #metadata-pda parent="mint" x="-10" y="-80" label="PDA" theme="crimson" /%}

{% node parent="metadata-pda" x="-280" %}
{% node #metadata label="Metadata Account" theme="crimson" /%}
{% node label="Owner: Token Metadata Program" theme="dimmed" /%}
{% /node %}

{% node #master-edition-pda parent="mint" x="-10" y="-220" label="PDA" theme="crimson" /%}

{% node parent="master-edition-pda" x="-280" %}
{% node #master-edition label="Master Edition Account" theme="crimson" /%}
{% node label="Owner: Token edition Program" theme="dimmed" /%}
{% node label="Key = MasterEditionV2" /%}
{% node label="Supply" /%}
{% node label="Max Supply" theme="orange" z=1 /%}
{% /node %}

{% node parent="master-edition" y="-140" %}
{% node #edition label="Edition Account" theme="crimson" /%}
{% node label="Owner: Token edition Program" theme="dimmed" /%}
{% node label="Key = EditionV1" /%}
{% node #edition-parent label="Parent" /%}
{% node label="Edition" /%}
{% /node %}

{% edge from="wallet" to="token" /%}
{% edge from="mint" to="token" /%}
{% edge from="mint" to="metadata-pda" /%}
{% edge from="mint" to="master-edition-pda" /%}
{% edge from="metadata-pda" to="metadata" path="straight" /%}
{% edge from="master-edition-pda" to="master-edition" path="straight" /%}
{% edge from="master-edition-pda" to="edition" fromPosition="left" label="OR" /%}
{% edge from="mint-authority" to="master-edition-pda" dashed=true arrow="none" fromPosition="right" toPosition="right" /%}
{% edge from="freeze-authority" to="master-edition-pda" dashed=true arrow="none" fromPosition="right" toPosition="right" /%}
{% edge from="edition-parent" to="master-edition" dashed=true arrow="none" fromPosition="left" toPosition="left" /%}
{% /diagram %}

## Setting up a Master Edition NFT

To create a printable NFT, we need to configure the **Print Supply** attribute of [the **Create** instruction](/token-metadata/mint#creating-accounts) of the Token Metadata program. This will configure the **Max Supply** attribute of the **Master Edition** account as seen in the previous section. This attribute can be:

- `Zero`: The NFT is not printable.
- `Limited(x)`: The NFT is printable and has a fixed supply of `x` editions.
- `Unlimited`: The NFT is printable and has an unlimited supply.

Here is how you can use our SDKs to create a printable NFT.

{% dialect-switcher title="Create the Master Edition NFT" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { percentAmount, generateSigner } from '@metaplex-foundation/umi'
import { createNft, printSupply } from '@metaplex-foundation/mpl-token-metadata'

const mint = generateSigner(umi)
await createNft(umi, {
  mint,
  name: 'My Master Edition NFT',
  uri: 'https://example.com/my-nft.json',
  sellerFeeBasisPoints: percentAmount(5.5),
  printSupply: printSupply('Limited', [100]), // Or printSupply('Unlimited')
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}

## Printing Editions from the Master Edition NFT

Once we have a printable NFT that has not reached its **Max Supply**, we can print new editions from it. This is done by calling the **Print** instruction of the Token Metadata program. This instruction accepts the following attributes:

- **Master Edition Mint**: The address of the printable NFT's Mint account.
- **Edition Mint**: The address of the new edition NFT's Mint account. This is typically a newly generated Signer since the account will be created by the instruction if it does not exist.
- **Master Token Account Owner**: The owner of the printable NFT as a Signer. Only the owner of a printable NFT can print new editions from it.
- **Edition Token Account Owner**: The address of the new edition NFT's owner
- **Edition Number**: The edition number of the new edition NFT to print. This is typically the current **Supply** of the **Master Edition** account plus 1.
- **Token Standard**: The token standard of the printable NFT. This can be `NonFungible` or `ProgrammableNonFungible`.

Here is how you can use our SDKs to print a new edition from a printable NFT.

{% dialect-switcher title="Create the Master Edition NFT" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import {
  printV1,
  fetchMasterEditionFromSeeds,
} from '@metaplex-foundation/mpl-token-metadata'

// (Optional) Fetch the master edition account to mint the next edition number.
const masterEdition = await fetchMasterEditionFromSeeds(umi, {
  mint: masterEditionMint,
})

const editionMint = generateSigner(umi)
await printV1(umi, {
  masterTokenAccountOwner: originalOwner,
  masterEditionMint,
  editionMint,
  editionTokenAccountOwner: ownerOfThePrintedEdition,
  editionNumber: masterEdition.supply + 1n,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /dialect %}
{% /dialect-switcher %}
