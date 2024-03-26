---
title: Managing Candy Machines
metaTitle: Candy Machine - Create, Update, Fetch and Delete
description: Explains how to manage Candy Machines.
---

On [the previous page](/candy-machine/settings), we went through the various settings of a Candy Machine. So now, let’s see how we can use these settings to create and update Candy Machines. We’ll also talk about how to fetch an existing Candy Machine and how to delete it when it has served its purpose. {% .lead %}

Essentially, we’ll be going through the Create, Read, Update and Delete steps of a Candy Machine. Let’s go!

## Create Candy Machines

You may use the settings discussed on the previous page to create a brand-new Candy Machine account.

Our SDKs push this even further and will associate every new Candy Machine account with a new Candy Guard account which keeps track of all activated guards affecting the minting process. On this page, we will focus on the Candy Machine account but we’ll dig into Candy Guard accounts and what we can do with them on [dedicated pages](/candy-machine/guards).

Remember that a Candy Machine [must be associated with a Collection NFT](/candy-machine/settings#metaplex-certified-collections) and its update authority must authorize this operation. If you haven’t got a Collection NFT for your Candy Machine yet, our SDKs can help with that too.

{% dialect-switcher title="Create a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

Here’s how you can create a Candy Machine using a brand new Collection NFT via the Umi library.

```ts
import {
  createNft,
  TokenStandard,
} from '@metaplex-foundation/mpl-token-metadata'
import { create } from '@metaplex-foundation/mpl-candy-machine'
import { generateSigner, percentAmount } from '@metaplex-foundation/umi'

// Create the Collection NFT.
const collectionMint = generateSigner(umi)
await createNft(umi, {
  mint: collectionMint,
  authority: umi.identity,
  name: 'My Collection NFT',
  uri: 'https://example.com/path/to/some/json/metadata.json',
  sellerFeeBasisPoints: percentAmount(9.99, 2), // 9.99%
  isCollection: true,
}).sendAndConfirm(umi)

// Create the Candy Machine.
const candyMachine = generateSigner(umi)
await create(umi, {
  candyMachine,
  collectionMint: collectionMint.publicKey,
  collectionUpdateAuthority: umi.identity,
  tokenStandard: TokenStandard.NonFungible,
  sellerFeeBasisPoints: percentAmount(9.99, 2), // 9.99%
  itemsAvailable: 5000,
  creators: [
    {
      address: umi.identity.publicKey,
      verified: true,
      percentageShare: 100,
    },
  ],
  configLineSettings: some({
    prefixName: '',
    nameLength: 32,
    prefixUri: '',
    uriLength: 200,
    isSequential: false,
  }),
}).sendAndConfirm(umi)
```

As mentioned above, this operation will also take care of creating and associating a new Candy Guard account with the created Candy Machine. That’s because a Candy Machine without a Candy Guard is not very useful and you’ll want to do that most of the time. Still, if you’d like to disable that behaviour, you may use the `createCandyMachineV2` method instead.

```tsx
import { createCandyMachineV2 } from '@metaplex-foundation/mpl-candy-machine'

await createCandyMachineV2(umi, {
  // ...
}).sendAndConfirm(umi)
```

In these examples, we only focused on the required parameters but you may want to check out the following API References to see what you can do with this `create` function.

API References: [create](https://mpl-candy-machine-js-docs.vercel.app/functions/create.html), [createCandyMachineV2](https://mpl-candy-machine-js-docs.vercel.app/functions/createCandyMachineV2.html).

{% /dialect %}
{% /dialect-switcher %}

## Candy Machine Account

Now that we’ve created the Candy Machine account, let’s see what data is stored inside it.

First of all, it stores all the settings provided when the account was created and keeps track of any changes. See the [previous page](/candy-machine/settings) for more details on these settings.

Additionally, it stores the following attributes:

- **Items Redeemed**. This keeps track of the number of NFTs that were minted from the Candy Machine. Note that, as soon as this number goes from 0 to 1, most settings will no longer be updatable.
- **Account Version**. This enum is used to keep track of the account version of the Candy Machine. It is used to determine which features are available and how the account should be interpreted. Note that this is not to be confused with "Candy Machine V3" which refers to the third and latest iteration of the Candy Machine programs (including the Candy Machine Core and Candy Guard programs).
- **Feature Flags**. This helps the program with backward and forward compatibility as more features get introduced.

Finally, it stores all items inserted in the Candy Machine and whether or not they have been minted. This only applies for Candy Machine using [**Config Line Settings**](/candy-machine/settings#config-line-settings) since [**Hidden Settings**](/candy-machine/settings#hidden-settings) don’t allow you to insert any items. This section contains the following information:

- The number of items that have been loaded.
- A list of all items that have been or will be inserted. When an item is not inserted yet, the name and URI of the item at that position are empty.
- A bitmap — a list of yes or no — that keeps track of which items have been loaded. When this bitmap is full of yeses, all items have been loaded.
- A list of all items that have _not_ been minted yet when minting using a random order. This allows the program to grab an index at random without having to worry about picking an index that has already been minted and start again.

Note that this last section is purposely not deserialised on the program but our SDKs parse all that data for you in a human-friendly format.

For more detailed information about the Candy Machine account, check out the [program’s API References](https://github.com/metaplex-foundation/mpl-candy-machine/tree/main/programs/candy-machine-core#account).

{% dialect-switcher title="Inside Candy Machine accounts" %}
{% dialect title="JavaScript" id="js" %}

The best way to check how Candy Machines are modelled in the Umi library is by checking [the API References of the `CandyMachine` account](https://mpl-candy-machine-js-docs.vercel.app/types/CandyMachine.html). You may also want to check out the [API References of the `candyGuard` account](https://mpl-candy-machine-js-docs.vercel.app/types/CandyGuard.html) since one is automatically created for each candy machine when using the `create` function.

Here’s a small code example showcasing some of the Candy Machine attributes.

```tsx
import {
  fetchCandyMachine,
  fetchCandyGuard,
} from '@metaplex-foundation/mpl-candy-machine'

const candyMachine = await fetchCandyMachine(umi, candyMachineAddress)
const candyGuard = await fetchCandyGuard(umi, candyMachine.mintAuthority)

candyMachine.publicKey // The public key of the Candy Machine account.
candyMachine.mintAuthority // The mint authority of the Candy Machine which, in most cases, is the Candy Guard address.
candyMachine.data.itemsAvailable // Total number of NFTs available.
candyMachine.itemsRedeemed // Number of NFTs minted.
candyMachine.items[0].index // The index of the first loaded item.
candyMachine.items[0].name // The name of the first loaded item (with prefix).
candyMachine.items[0].uri // The URI of the first loaded item (with prefix).
candyMachine.items[0].minted // Whether the first item has been minted.
```

{% /dialect %}
{% /dialect-switcher %}

## Fetch Candy Machines

To fetch an existing Candy Machine, you simply need to provide its address and our SDKs will take care of parsing the account data for you.

{% dialect-switcher title="Fetch Candy Machines" %}
{% dialect title="JavaScript" id="js" %}

Here’s how you can fetch a Candy Machine using its address and its associated Candy Guard account if any.

```ts
import { publicKey } from '@metaplex-foundation/umi'
import {
  fetchCandyMachine,
  fetchCandyGuard,
} from '@metaplex-foundation/mpl-candy-machine'

const candyMachine = await fetchCandyMachine(umi, publicKey('...'))
const candyGuard = await fetchCandyGuard(umi, candyMachine.mintAuthority)
```

API References: [fetchCandyMachine](https://mpl-candy-machine-js-docs.vercel.app/functions/fetchCandyMachine.html), [fetchCandyGuard](https://mpl-candy-machine-js-docs.vercel.app/functions/fetchCandyGuard.html).

{% /dialect %}
{% /dialect-switcher %}

## Update Authorities

Once a Candy Machine is created, you can update most of its settings later on, as long as you are the authority of the Candy Machine. In the next few sections we’ll see how to update these settings but first, let's see how you can update the **Authority** and **Mint Authority** of a Candy Machine.

- To update the authority, you need to pass the current authority as a signer and the address of the new authority.
- To update the mint authority, you need to pass both the current authority and the new mint authority as signers. This is because the mint authority is mostly used to associate a Candy Guard with a Candy Machine. Making the mint authority a signer prevents anyone to use someone else Candy Guard as this could create side effects on the original Candy Machine.

{% dialect-switcher title="Update the authority of a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

Here’s how you can update the authority of a Candy Machine using the Umi library. Note that, in most cases, you'll want to update the authority of the associated Candy Guard account as well.

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import {
  setCandyMachineAuthority,
  setCandyGuardAuthority,
} from '@metaplex-foundation/mpl-candy-machine'

const newAuthority = generateSigner(umi)
await setCandyMachineAuthority(umi, {
  candyMachine: candyMachine.publicKey,
  authority: currentAuthority,
  newAuthority: newAuthority.publicKey,
})
  .add(
    setCandyGuardAuthority(umi, {
      candyGuard: candyMachine.mintAuthority,
      authority: currentAuthority,
      newAuthority: newAuthority.publicKey,
    })
  )
  .sendAndConfirm(umi)
```

Whilst you’d probably never want to update the `mintAuthority` directly since it would override the associated Candy Guard account, this is how you’d do it.

```ts
import { generateSigner } from '@metaplex-foundation/umi'
import { setMintAuthority } from '@metaplex-foundation/mpl-candy-machine'

const newMintAuthority = generateSigner(umi)
await setMintAuthority(umi, {
  candyMachine: candyMachine.publicKey,
  authority: currentAuthority,
  mintAuthority: newMintAuthority,
}).sendAndConfirm(umi)
```

API References: [setCandyMachineAuthority](https://mpl-candy-machine-js-docs.vercel.app/functions/setCandyMachineAuthority.html), [setCandyGuardAuthority](https://mpl-candy-machine-js-docs.vercel.app/functions/setCandyGuardAuthority.html), [setMintAuthority](https://mpl-candy-machine-js-docs.vercel.app/functions/setMintAuthority.html).

{% /dialect %}
{% /dialect-switcher %}

## Update Shared NFT Data

You may also update the attributes shared between all minted NFTs of a Candy Machine. As mentioned in [the previous page](/candy-machine/settings#settings-shared-by-all-nf-ts), these are: Seller Fee Basis Points, Symbol, Max Edition Supply, Is Mutable and Creators.

Note that once the first NFT has been minted, these attributes can no longer be updated.

{% dialect-switcher title="Update the NFT data of a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

Here’s an example of updating some of the shared NFT data on a Candy Machine.

```tsx
import { percentAmount } from '@metaplex-foundation/umi'
import {
  updateCandyMachine,
  fetchCandyMachine,
} from '@metaplex-foundation/mpl-candy-machine'

const candyMachine = await fetchCandyMachine(umi, candyMachineAddress)
await updateCandyMachine(umi, {
  candyMachine: candyMachine.publicKey,
  data: {
    ...candyMachine.data,
    symbol: 'NEW',
    sellerFeeBasisPoints: percentAmount(5.5, 2),
    creators: [{ address: newCreator, verified: false, percentageShare: 100 }],
  },
}).sendAndConfirm(umi)
```

API References: [updateCandyMachine](https://mpl-candy-machine-js-docs.vercel.app/functions/updateCandyMachine.html).

{% /dialect %}
{% /dialect-switcher %}

## Update Token Standard

The Token Standard and Rule Set attributes can also be updated on a Candy Machine using the "Set Token Standard" instruction. This allows us to switch from regular NFTs to programmable NFTs and vice versa. When switching to programmable NFTs, we can optionally specify or update the Rule Set that minted NFTs should adhere to.

Note that, if you candy machine is using an old account version, this instruction will also automatically upgrade it to the latest account version that supports programmable NFTs as well as regular NFTs. Once upgraded, you will need to use the latest instructions for minting from the candy machine or candy guard.

{% dialect-switcher title="Update the Token Standard of a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

Here's an example of updating the token standard and rule set on a Candy Machine using Umi.

```ts
import { TokenStandard } from '@metaplex-foundation/mpl-token-metadata'
import { setTokenStandard } from '@metaplex-foundation/mpl-candy-machine'

await setTokenStandard(umi, {
  candyMachine: candyMachine.publicKey,
  collectionMint: candyMachine.collectionMint,
  collectionUpdateAuthority,
  tokenStandard: TokenStandard.ProgrammableNonFungible,
  ruleSet: newRuleSetAccount,
}).sendAndConfirm(umi)
```

Note that if your candy machine is using account version `V1`, you will need to explicitly set the `collectionAuthorityRecord` account as it uses the legacy collection delegate authority record account.

```ts
import { findCollectionAuthorityRecordPda } from '@metaplex-foundation/mpl-token-metadata'
import { findCandyMachineAuthorityPda } from '@metaplex-foundation/mpl-candy-machine'

await setTokenStandard(umi, {
  // ...
  collectionAuthorityRecord: findCollectionAuthorityRecordPda(umi, {
    mint: candyMachine.collectionMint,
    collectionAuthority: findCandyMachineAuthorityPda(umi, {
      candyMachine: candyMachine.publicKey,
    }),
  }),
}).sendAndConfirm(umi)
```

API References: [setTokenStandard](https://mpl-candy-machine-js-docs.vercel.app/functions/setTokenStandard.html).

{% /dialect %}
{% /dialect-switcher %}

## Update Collection

Changing the Collection NFT associated with a Candy Machine is also possible. You’ll need to provide the address of the Collection NFT’s mint account as well as its update authority as a signer to approve this change.

Note that, here also, once the first NFT has been minted, the collection cannot be changed.

{% dialect-switcher title="Update the collection of a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

To update the Collection NFT of a Candy Machine using the Umi library you may use the `setCollectionV2` method like so.

```ts
await setCollectionV2(umi, {
  candyMachine: candyMachine.publicKey,
  collectionMint: candyMachine.collectionMint,
  collectionUpdateAuthority: collectionUpdateAuthority.publicKey,
  newCollectionMint: newCollectionMint.publicKey,
  newCollectionUpdateAuthority,
}).sendAndConfirm(umi)
```

Note that if your candy machine is using account version `V1`, you will need to explicitly set the `collectionDelegateRecord` account as it uses the legacy collection delegate authority record account.

```ts
import { findCollectionAuthorityRecordPda } from '@metaplex-foundation/mpl-token-metadata'
import { findCandyMachineAuthorityPda } from '@metaplex-foundation/mpl-candy-machine'

await setCollectionV2(umi, {
  // ...
  collectionDelegateRecord: findCollectionAuthorityRecordPda(umi, {
    mint: candyMachine.collectionMint,
    collectionAuthority: findCandyMachineAuthorityPda(umi, {
      candyMachine: candyMachine.publicKey,
    }),
  }),
}).sendAndConfirm(umi)
```

API References: [setCollectionV2](https://mpl-candy-machine-js-docs.vercel.app/functions/setCollectionV2.html).

{% /dialect %}
{% /dialect-switcher %}

## Update Item Settings

The item settings of a Candy Machine can also be updated but there are some limitations.

- The item settings cannot be updated such that we are swapping between **Config Line Settings** and **Hidden Settings**. However, if we’re not swapping the modes, the properties inside these settings can be updated.
- When using **Config Line Settings**:
  - The **Items Available** attribute cannot be updated.
  - The **Name Length** and **URI Length** properties can only be updated to smaller values as the program will not resize the Candy Machine account during updates.
- Once the first NFT has been minted, these settings can no longer be updated.

{% dialect-switcher title="Update the item settings of a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

The following example shows how to update the Config Line Settings of a Candy Machine using the Umi library. The same can be done with Hidden Settings and the Items Available attribute (when using Hidden Settings).

```ts
import {
  updateCandyMachine,
  fetchCandyMachine,
} from '@metaplex-foundation/mpl-candy-machine'

const candyMachine = await fetchCandyMachine(umi, candyMachineAddress)
await updateCandyMachine(umi, {
  candyMachine: candyMachine.publicKey,
  data: {
    ...candyMachine.data,
    hiddenSettings: none(),
    configLineSettings: some({
      type: 'configLines',
      prefixName: 'My New NFT #$ID+1$',
      nameLength: 0,
      prefixUri: 'https://arweave.net/',
      uriLength: 43,
      isSequential: true,
    }),
  },
}).sendAndConfirm(umi)
```

API References: [updateCandyMachine](https://mpl-candy-machine-js-docs.vercel.app/functions/updateCandyMachine.html).

{% /dialect %}
{% /dialect-switcher %}

## Delete Candy Machines

Once a Candy Machine has been fully minted, it has served its purpose and can safely be disposed of. This allows its creator to gain back the storage cost of the Candy Machine account and, optionally, the Candy Guard account too.

{% dialect-switcher title="Delete a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

You may delete a Candy Machine account and/or its associated Candy Guard account using the Umi library like so.

```ts
import {
  deleteCandyMachine,
  deleteCandyGuard,
} from '@metaplex-foundation/mpl-candy-machine'

await deleteCandyMachine(umi, {
  candyMachine: candyMachine.publicKey,
}).sendAndConfirm(umi)

await deleteCandyGuard(umi, {
  candyGuard: candyMachine.mintAuthority,
}).sendAndConfirm(umi)
```

API References: [deleteCandyMachine](https://mpl-candy-machine-js-docs.vercel.app/functions/deleteCandyMachine.html), [deleteCandyGuard](https://mpl-candy-machine-js-docs.vercel.app/functions/deleteCandyGuard.html).

{% /dialect %}
{% /dialect-switcher %}

## Conclusion

We can now create, read, update and delete Candy Machines but we still don’t know how to load them with items. Let’s tackle this on [the next page](/candy-machine/insert-items)!
