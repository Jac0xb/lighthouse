---
title: Programmable NFTs
metaTitle: Candy Machine - Programmable NFTs
description: Explains how to mint Programmable NFTs from candy machines.
---

Version `1.7` of Token Metadata introduced a [new type of asset class called Programmable NFTs](/token-metadata/pnfts) allowing, amongst other things, creators to enforce royalties on secondary sales.

Since version `1.0` of Candy Machine Core and version `1.0` of Candy Guard, it is now possible to **mint Programmable NFTs from candy machines** and even update the token standard of existing candy machines.

## For new candy machines

A new instruction called `initializeV2` has been added to the Candy Machine Core program. This instruction is similar to the `initialize` instruction, but it allows you to specify the token standard you want to use for your candy machine. This instruction will mark the newly created Candy Machine as `V2` to differentiate it from the `V1` Candy Machines that do not store the token standard. These new fields are using existing padding in the Candy Machine account data to avoid breaking changes in the Candy Machine serialization logic.

The `initializeV2` instruction can also be used to create a Candy Machine that mints regular NFTs and, therefore, the `initialize` instruction is now deprecated. Note that no changes are needed for the Candy Guard program here since it delegates to the Candy Machine Core when minting the NFT.

Also, note that some optional accounts may be required depending on the token standard you choose. For example, the `ruleSet` account may be provided to assign a specific rule set to all minted Programmable NFTs. If no `ruleSet` account is provided, it will use the rule set of the Collection NFT if any. Otherwise, minted Programmable NFTs will simply not have any rule set assigned. On the other hand, the `ruleSet` account will be ignored when minting regular NFTs.

Additionally, the `collectionDelegateRecord` account should now refer to the new [Metadata Delegate Record](https://docs.rs/mpl-token-metadata/latest/mpl_token_metadata/state/struct.MetadataDelegateRecord.html) from Token Metadata.

You may want to read the "[Create Candy Machines](/candy-machine/managing-candy-machines#create-candy-machines)" section of this documentation for more details but here are some examples on how to use our SDKs to create a new Candy Machine that mints Programmable NFTs.

{% dialect-switcher title="Create a new PNFT Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { TokenStandard } from '@metaplex-foundation/mpl-token-metadata'
import { create } from '@metaplex-foundation/mpl-candy-machine'
import { generateSigner } from '@metaplex-foundation/umi'

await create(umi, {
  // ...
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

API References: [create](https://mpl-candy-machine-js-docs.vercel.app/functions/create.html)

{% /dialect %}
{% /dialect-switcher %}

## For existing candy machines

It is possible to update the token standard of existing Candy Machines via the new `setTokenStandard` instruction. When calling this instruction on a Candy Machine `V1`, it will also upgrade the Candy Machine to `V2` and store the token standard in the account data.

You may want to read the "[Update Token Standard](/candy-machine/manage#update-token-standard)" section of this documentation for more details but here are some examples on how to use our SDKs to update the token standard of an existing Candy Machine to Programmable NFTs.

{% dialect-switcher title="Change the Token Standard of a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { TokenStandard } from '@metaplex-foundation/mpl-token-metadata'
import { setTokenStandard } from '@metaplex-foundation/mpl-candy-machine'

await setTokenStandard(umi, {
  candyMachine: candyMachine.publicKey,
  collectionMint: candyMachine.collectionMint,
  collectionUpdateAuthority,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
}).sendAndConfirm(umi)
```

API References: [setTokenStandard](https://mpl-candy-machine-js-docs.vercel.app/functions/setTokenStandard.html)

{% /dialect %}
{% /dialect-switcher %}

Additionally, a new `setCollectionV2` instruction has been added to support setting a collection that is compatible with Programmable NFTs. This instruction also works with regular NFTs and deprecates the `setCollection` instruction.

Here as well, you can read more about it in the "[Update Collection](/candy-machine/manage#update-collection)" section of this documentation.

{% dialect-switcher title="Update the collection of your Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { setCollectionV2 } from '@metaplex-foundation/mpl-candy-machine'

await setCollectionV2(umi, {
  candyMachine: candyMachine.publicKey,
  collectionMint: candyMachine.collectionMint,
  collectionUpdateAuthority: collectionUpdateAuthority.publicKey,
  newCollectionMint: newCollectionMint.publicKey,
  newCollectionUpdateAuthority,
}).sendAndConfirm(umi)
```

API References: [setCollectionV2](https://mpl-candy-machine-js-docs.vercel.app/functions/setCollectionV2.html)

{% /dialect %}
{% /dialect-switcher %}

## A new minting instruction

The `mint` instruction of both the Candy Machine Core and the Candy Guard programs has been updated to support minting Programmable NFTs. This new instruction is called `mintV2` and it is similar to the `mint` instruction, but requires additional accounts to be passed in. Here as well, the new `mintV2` instructions can be used to mint regular NFTs and, therefore, they deprecate the existing `mint` instructions.

The entire "[Minting](/candy-machine/mint)" page has been updated to use the new `mintV2` instructions but here's a quick example of how to use them with Programmable NFTs.

{% dialect-switcher title="Mint from your Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

```ts
import { mintV2 } from '@metaplex-foundation/mpl-candy-machine'
import { setComputeUnitLimit } from '@metaplex-foundation/mpl-toolbox'
import { transactionBuilder, generateSigner } from '@metaplex-foundation/umi'

const nftMint = generateSigner(umi)
await transactionBuilder()
  .add(setComputeUnitLimit(umi, { units: 800_000 }))
  .add(
    mintV2(umi, {
      candyMachine: candyMachine.publicKey,
      nftMint,
      collectionMint: collectionNft.publicKey,
      collectionUpdateAuthority: collectionNft.metadata.updateAuthority,
    })
  )
  .sendAndConfirm(umi)
```

API References: [mintV2](https://mpl-candy-machine-js-docs.vercel.app/functions/mintV2.html)

{% /dialect %}
{% /dialect-switcher %}

Note that some of the guards offered by the Candy Guard program have also been updated to support Programmable NFTs. Whilst the updates do not introduce breaking changes when minting regular NFTs, they may expect more remaining accounts when minting depending on the token standard.

The guards affected by these changes are:

- The `nftBurn` and `nftPayment` guards now allow the burned/sent NFT to be a Programmable NFT.
- The `FreezeSolPayment` and `FreezeTokenPayment` guards. Since Programmable NFTs are by definition always frozen, they are Locked when minted via a Utility delegate and Unlocked when the thaw conditions have been met.

## Additional reading

You may find the following resources about Programmable NFTs and Candy Machines useful:

- [Programmable NFTs Guide](/token-metadata/pnfts)
- [Candy Machine Core Program](https://github.com/metaplex-foundation/mpl-candy-machine/tree/main/programs/candy-machine-core)
- [Candy Guard Program](https://github.com/metaplex-foundation/mpl-candy-machine/tree/main/programs/candy-guard)
