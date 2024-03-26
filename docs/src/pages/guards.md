---
title: Candy Guards
metaTitle: Candy Machine - Candy Guards
description: Explains how guards work and how to enable them.
---

Now that we know how Candy Machines work and how to load them, it’s time we talk about the last piece of the puzzle: Guards. {% .lead %}

## What is a guard?

A guard is a modular piece of code that can restrict access to the mint of a Candy Machine and even add new features to it!

There is a large set of guards to choose from and each of them can be activated and configured at will.

We’ll touch on all available guards later in this documentation but let’s go through a few examples here to illustrate that.

- When the **Start Date** guard is enabled, minting will be forbidden before the preconfigured date. There is also an **End Date** guard to forbid minting after a given date.
- When the **Sol Payment** guard is enabled, the minting wallet will have to pay a configured amount to a configured destination wallet. Similar guards exist for paying with tokens or NFTs of a specific collection.
- The **Token Gate** and **NFT Gate** guards restrict minting to certain token holders and NFT holders respectively.
- The **Allow List** guard only allows minting if the wallet is part of a predefined list of wallets. Kind of like a guest list for minting.

As you can see, each guard takes care of one responsibility and one responsibility only which makes them composable. In other words, you can pick and choose the guards your need to create your perfect Candy Machine.

## The Candy Guard account

If you remember the content of our [Candy Machine account](/candy-machine/manage#candy-machine-account), you’ll see no signs of guards in there. This is because guards live in another account called the **Candy Guard account** which is created by the **Candy Guard program**.

Each Candy Machine account should typically be associated with its own Candy Guard account which will add a layer of protection to it.

This works by creating a Candy Guard account and making it the **Mint Authority** of the Candy Machine account. By doing so, it is no longer possible to mint directly from the main Candy Machine program — known as the **Candy Machine Core program**. Instead, we must mint via the Candy Guard program which, if all guards are resolved successfully, will defer to the Candy Machine Core program to finish the minting process.

{% diagram %}

{% node %}
{% node #candy-machine-1 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% node label="Features" /%}
{% node label="Authority" /%}
{% node #mint-authority-1 %}

Mint Authority = Candy Guard {% .font-semibold %}

{% /node %}
{% node label="..." /%}
{% /node %}

{% node parent="candy-machine-1" y=160 x=20 %}
{% node #candy-guard-1 label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node label="Guards" theme="mint" z=1 /%}
{% node label="Sol Payment" /%}
{% node label="Token Payment" /%}
{% node label="Start Date" /%}
{% node label="End Date" /%}
{% node label="..." /%}
{% /node %}

{% node parent="candy-machine-1" x=350 %}
{% node #mint-1 label="Mint" theme="pink" /%}
{% node label="Candy Guard Program" theme="pink" /%}
{% /node %}
{% node parent="mint-1" x=45 y=-20 label="Access Control" theme="transparent" /%}
{% node parent="mint-1" x=-120 y=-35 theme="transparent" %}
Anyone can mint as long \
as they comply with the \
activated guards.
{% /node %}

{% node parent="mint-1" x=-22 y=100 %}
{% node #mint-2 label="Mint" theme="pink" /%}
{% node label="Candy Machine Core Program" theme="pink" /%}
{% /node %}
{% node parent="mint-2" x=120 y=-20 label="Mint Logic" theme="transparent" /%}
{% node parent="mint-2" x=200 y=-18 theme="transparent" %}
Only Alice \
can mint.
{% /node %}

{% node #nft parent="mint-2" x=62 y=100 label="NFT" /%}

{% node parent="mint-2" x=280 %}
{% node #candy-machine-2 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% node label="Features" /%}
{% node label="Authority" /%}
{% node #mint-authority-2 %}

Mint Authority = Alice {% .font-semibold %}

{% /node %}
{% node label="..." /%}
{% /node %}

{% edge from="candy-guard-1" to="mint-authority-1" fromPosition="left" toPosition="left" arrow=false dashed=true /%}
{% edge from="mint-1" to="mint-2" theme="pink" path="straight" /%}
{% edge from="mint-2" to="nft" theme="pink" path="straight" /%}
{% edge from="candy-machine-1" to="mint-1" theme="pink" /%}
{% edge from="candy-guard-1" to="mint-1" theme="pink" /%}
{% edge from="candy-machine-2" to="mint-2" theme="pink" path="straight" /%}

{% /diagram %}

Note that, since Candy Machine and Candy Guard accounts work hand and hand together, our SDKs treat them as one entity. When you create a Candy Machine with our SDKs, an associated Candy Guard account will also be created by default. The same goes when updating Candy Machines as they allow you to update guards at the same time. We will see some concrete examples on this page.

## Why another program?

The reason guards don’t live in the main Candy Machine program is to separate the access control logic from the main Candy Machine responsibility which is to mint an NFT.

This enables guards to not only be modular but extendable. Anyone can create and deploy their own Candy Guard program to create custom guards whilst relying on the Candy Machine Core program for all the rest.

{% diagram %}

{% node %}
{% node #candy-machine-1 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine-1" y=80 x=20 %}
{% node #candy-guard-1 label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node label="Guards" theme="mint" z=1 /%}
{% node label="Sol Payment" /%}
{% node label="Token Payment" /%}
{% node label="Start Date" /%}
{% node label="End Date" /%}
{% node label="..." /%}
{% /node %}

{% node parent="candy-machine-1" x=300 %}
{% node #mint-1 label="Mint" theme="pink" /%}
{% node label="Candy Guard Program" theme="pink" /%}
{% /node %}
{% node parent="mint-1" x=160 %}
{% node #mint-1b label="Mint" theme="pink" /%}
{% node label="Custom Candy Guard Program" theme="pink" /%}
{% /node %}
{% node parent="mint-1b" x=-80 y=-22 label="Different Access Control" theme="transparent" /%}

{% node parent="mint-1" x=60 y=100 %}
{% node #mint-2 label="Mint" theme="pink" /%}
{% node label="Candy Machine Core Program" theme="pink" /%}
{% /node %}
{% node parent="mint-2" x=95 y=-20 label="Same Mint Logic" theme="transparent" /%}

{% node #nft parent="mint-2" x=62 y=100 label="NFT" /%}

{% node parent="mint-1b" x=250 %}
{% node #candy-machine-2 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine-2" y=80 x=0 %}
{% node #candy-guard-2 label="Candy Guard" theme="blue" /%}
{% node label="Owner: Custom Candy Guard Program" theme="dimmed" /%}
{% node label="Guards" theme="mint" z=1 /%}
{% node label="Sol Payment" /%}
{% node label="Token Payment" /%}
{% node label="Start Date" /%}
{% node %}
My Custom Guard {% .font-semibold %}
{% /node %}
{% node label="..." /%}
{% /node %}

{% edge from="candy-guard-1" to="candy-machine-1" fromPosition="left" toPosition="left" arrow=false /%}
{% edge from="candy-guard-2" to="candy-machine-2" fromPosition="right" toPosition="right" arrow=false /%}
{% edge from="mint-1" to="mint-2" theme="pink" fromPosition="bottom" toPosition="top" /%}
{% edge from="mint-1b" to="mint-2" theme="pink" fromPosition="bottom" toPosition="top" /%}
{% edge from="mint-2" to="nft" theme="pink" path="straight" /%}
{% edge from="candy-machine-1" to="mint-1" theme="pink" /%}
{% edge from="candy-guard-1" to="mint-1" theme="pink" /%}
{% edge from="candy-machine-2" to="mint-1b" theme="pink" /%}
{% edge from="candy-guard-2" to="mint-1b" theme="pink" /%}

{% /diagram %}

Note that our SDKs also offer ways to register your own Candy Guard programs and their custom guards so you can leverage their friendly API and easily share your guards with others.

## All available guards

Alright, now that we understand what guards are, let’s see what default guards are available to us.

In the following list, we’ll provide a short description of each guard with a link pointing to their dedicated page for more advanced reading.

- [**Address Gate**](/candy-machine/guards/address-gate): Restricts the mint to a single address.
- [**Allow List**](/candy-machine/guards/allow-list): Uses a wallet address list to determine who is allowed to mint.
- [**Bot Tax**](/candy-machine/guards/bot-tax): Configurable tax to charge invalid transactions.
- [**End Date**](/candy-machine/guards/end-date): Determines a date to end the mint.
- [**Freeze Sol Payment**](/candy-machine/guards/freeze-sol-payment): Set the price of the mint in SOL with a freeze period.
- [**Freeze Token Payment**](/candy-machine/guards/freeze-token-payment): Set the price of the mint in token amount with a freeze period.
- [**Gatekeeper**](/candy-machine/guards/gatekeeper): Restricts minting via a Gatekeeper Network e.g. Captcha integration.
- [**Mint Limit**](/candy-machine/guards/mint-limit): Specifies a limit on the number of mints per wallet.
- [**Nft Burn**](/candy-machine/guards/nft-burn): Restricts the mint to holders of a specified collection, requiring a burn of the NFT.
- [**Nft Gate**](/candy-machine/guards/nft-gate): Restricts the mint to holders of a specified collection.
- [**Nft Payment**](/candy-machine/guards/nft-payment): Set the price of the mint as an NFT of a specified collection.
- [**Redeemed Amount**](/candy-machine/guards/redeemed-amount): Determines the end of the mint based on the total amount minted.
- [**Sol Payment**](/candy-machine/guards/sol-payment): Set the price of the mint in SOL.
- [**Start Date**](/candy-machine/guards/start-date): Determines the start date of the mint.
- [**Third Party Signer**](/candy-machine/guards/third-party-signer): Requires an additional signer on the transaction.
- [**Token Burn**](/candy-machine/guards/token-burn): Restricts the mint to holders of a specified token, requiring a burn of the tokens.
- [**Token Gate**](/candy-machine/guards/token-gate): Restricts the mint to holders of a specified token.
- [**Token Payment**](/candy-machine/guards/token-payment): Set the price of the mint in token amount.

## Creating a Candy Machine with guards

So far, the Candy Machine we created did not have any guards enabled. Now that we know all the guards available to us, let’s see how we can set up new Candy Machines with some guards enabled.

The concrete implementation will depend on which SDK you are using (see below) but the main idea is that you enable guards by providing their required settings. Any guard that has not been set up will be disabled.

{% dialect-switcher title="Create a Candy Machine with guards" %}
{% dialect title="JavaScript" id="js" %}

To enable guards using the Umi library, simply provides the `guards` attribute to the `create` function and pass in the settings of every guard you want to enable. Any guard set to `none()` or not provided will be disabled.

```ts
import { some, sol, dateTime } from '@metaplex-foundation/umi'

await create(umi, {
  // ...
  guards: {
    botTax: some({ lamports: sol(0.01), lastInstruction: true }),
    solPayment: some({ lamports: sol(1.5), destination: treasury }),
    startDate: some({ date: dateTime('2023-04-04T16:00:00Z') }),
    // All other guards are disabled...
  },
}).sendAndConfirm(umi)
```

API References: [create](https://mpl-candy-machine-js-docs.vercel.app/functions/create.html), [DefaultGuardSetArgs](https://mpl-candy-machine-js-docs.vercel.app/types/DefaultGuardSetArgs.html)

{% /dialect %}
{% /dialect-switcher %}

## Updating guards

Did you set something wrong in your guards? Did you change your mind about the mint price? Do you need to delay the start of the mint of a little? No worries, guards can easily be updated following the same settings used when creating them.

You can enable new guards by providing their settings or disable current ones by giving them empty settings.

{% dialect-switcher title="Update guards" %}
{% dialect title="JavaScript" id="js" %}

You may update the guards of a Candy Machine the same way you created them. That is, by providing their settings inside the `guards` object of the `updateCandyGuard` function. Any guard set to `none()` or not provided will be disabled.

Note that the entire `guards` object will be updated meaning **it will override all existing guards**!

Therefore, make sure to provide the settings for all guards you want to enable, even if their settings are not changing. You may want to fetch the candy guard account first to fallback to its current guards.

```tsx
import { some, none, sol } from '@metaplex-foundation/umi'

const candyGuard = fetchCandyGuard(umi, candyMachine.mintAuthority)
await updateCandyGuard(umi, {
  candyGuard: candyGuard.publicKey,
  guards: {
    ...candyGuard.guards,
    botTax: none(),
    solPayment: some({ lamports: sol(3), destination: treasury }),
  },
})
```

API References: [updateCandyGuard](https://mpl-candy-machine-js-docs.vercel.app/functions/updateCandyGuard.html), [CandyGuard](https://mpl-candy-machine-js-docs.vercel.app/types/CandyGuard.html), [DefaultGuardSetArgs](https://mpl-candy-machine-js-docs.vercel.app/types/DefaultGuardSetArgs.html)

{% /dialect %}
{% /dialect-switcher %}

## Viewing the guards of a Candy Machine

Once you have set up your guards on a Candy Machine, all the provided settings can be retrieved and viewed by anyone on the Candy Guard account.

{% dialect-switcher title="Fetch guards" %}
{% dialect title="JavaScript" id="js" %}

You may access the candy guard associated with a candy machine by using the `fetchCandyGuard` function on the `mintAuthority` attribute of the candy machine account.

```ts
import {
  fetchCandyMachine,
  fetchCandyGuard,
} from '@metaplex-foundation/mpl-candy-machine'

const candyMachine = await fetchCandyMachine(umi, candyMachineAddress)
const candyGuard = await fetchCandyGuard(umi, candyMachine.mintAuthority)

candyGuard.guards // All guard settings.
candyGuard.guards.botTax // Bot Tax settings.
candyGuard.guards.solPayment // Sol Payment settings.
// ...
```

Note that, when using the `create` function, an associated candy guard account is automatically created for each candy machine such that their address is deterministic. Therefore, in this case, we could fetch both accounts using only one RPC call like so.

```ts
import { assertAccountExists } from '@metaplex-foundation/umi'
import {
  findCandyGuardPda,
  deserializeCandyMachine,
  deserializeCandyGuard,
} from '@metaplex-foundation/mpl-candy-machine'

const candyGuardAddress = findCandyGuardPda(umi, { base: candyMachineAddress })
const [rawCandyMachine, rawCandyGuard] = await umi.rpc.getAccounts([
  candyMachineAddress,
  candyGuardAddress,
])
assertAccountExists(rawCandyMachine)
assertAccountExists(rawCandyGuard)

const candyMachine = deserializeCandyMachine(umi, rawCandyMachine)
const candyGuard = deserializeCandyGuard(umi, rawCandyGuard)
```

API References: [fetchCandyGuard](https://mpl-candy-machine-js-docs.vercel.app/functions/fetchCandyGuard.html), [findCandyGuardPda](https://mpl-candy-machine-js-docs.vercel.app/functions/findCandyGuardPda.html), [CandyGuard](https://mpl-candy-machine-js-docs.vercel.app/types/CandyGuard.html), [DefaultGuardSetArgs](https://mpl-candy-machine-js-docs.vercel.app/types/DefaultGuardSetArgs.html)

{% /dialect %}
{% /dialect-switcher %}

## Wrapping and unwrapping Candy Guard accounts manually

So far we’ve managed both Candy Machine and Candy Guard accounts together because that makes the most sense for most projects.

However, it is important to note that Candy Machines and Candy Guards can be created and associated in different steps, even using our SDKs.

You will first need to create the two accounts separately and associate/dissociate them manually.

{% dialect-switcher title="Associate and dissociate guards from a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

The `create` function of the Umi library already takes care of creating and associating a brand new Candy Guard account for every Candy Machine account created.

However, if you wanted to create them separately and manually associate/dissociate them, this is how you’d do it.

```ts
import { some, percentAmount, sol, dateTime } from '@metaplex-foundation/umi'

// Create a Candy Machine without a Candy Guard.
const candyMachine = generateSigner(umi)
await createCandyMachineV2({
  candyMachine,
  tokenStandard: TokenStandard.NonFungible,
  collectionMint: collectionMint.publicKey,
  collectionUpdateAuthority: umi.identity,
  itemsAvailable: 100,
  sellerFeeBasisPoints: percentAmount(1.23),
  creators: [
    { address: umi.identity.publicKey, verified: false, percentageShare: 100 },
  ],
  configLineSettings: some({
    prefixName: 'My NFT #',
    nameLength: 3,
    prefixUri: 'https://example.com/',
    uriLength: 20,
    isSequential: false,
  }),
}).sendAndConfirm(umi)

// Create a Candy Guard.
const base = generateSigner(umi)
const candyGuard = findCandyGuardPda(umi, { base: base.publicKey })
await createCandyGuard({
  base,
  guards: {
    botTax: { lamports: sol(0.01), lastInstruction: false },
    solPayment: { lamports: sol(1.5), destination: treasury },
    startDate: { date: dateTime('2022-10-17T16:00:00Z') },
  },
}).sendAndConfirm(umi)

// Associate the Candy Guard with the Candy Machine.
await wrap({
  candyMachine: candyMachine.publicKey,
  candyGuard,
}).sendAndConfirm(umi)

// Dissociate them.
await unwrap({
  candyMachine: candyMachine.publicKey,
  candyGuard,
}).sendAndConfirm(umi)
```

API References: [createCandyMachineV2](https://mpl-candy-machine-js-docs.vercel.app/functions/createCandyMachineV2.html), [createCandyGuard](https://mpl-candy-machine-js-docs.vercel.app/functions/createCandyGuard.html), [wrap](https://mpl-candy-machine-js-docs.vercel.app/functions/wrap.html), [unwrap](https://mpl-candy-machine-js-docs.vercel.app/functions/unwrap.html)

{% /dialect %}
{% /dialect-switcher %}

## Conclusion

Guards are important components of Candy Machines. They make it easy to configure the minting process whilst allowing anyone to create their own guards for application-specific needs. [On the next page](/candy-machine/guard-groups), we’ll see how we can create even more minting scenarios by using guard groups!
