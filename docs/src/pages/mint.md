---
title: Minting
metaTitle: Candy Machine - Minting
description: Explains how to mint from Candy Machines and how to handle pre-mint requirements.
---

So far, we’ve learned how to create and maintain Candy Machines. We’ve seen how to configure them and how to set up complex minting workflows using guard and guard groups. It’s about time we talk about the last piece of the puzzle: Minting! {% .lead %}

## Basic Minting

As mentioned [in the Candy Guards page](/candy-machine/guards#why-another-program), there are two programs responsible for minting NFTs from Candy Machines: The Candy Machine Core program — responsible for minting the NFT — and the Candy Guard program which adds a configurable Access Control layer on top of it and can be forked to offer custom guards.

As such, there are two ways to mint from a Candy Machine:

- **From a Candy Guard program** which will then delegate the minting to the Candy Machine Core program. Most of the time, you will want to do this as it allows for much more complex minting workflows. You may need to pass extra remaining accounts and instruction data to the mint instruction based on the guards configured in the account. Fortunately, our SDKs make this easy by requiring a few extra parameters and computing the rest for us.

- **Directly from the Candy Machine Core program**. In this case, only the configured mint authority can mint from it and, therefore, it will need to sign the transaction.

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

If everything went well, an NFT will be created following the parameters configured in the Candy Machine. For instance, if the given Candy Machine uses **Config Line Settings** with **Is Sequential** set to `false`, then we will get the next item at random.

Starting from version `1.0` of the Candy Guard program, The mint instruction accepts an additional `minter` signer which can be different than the existing `payer` signer. This allows us to create minting workflows where the wallet that mints the NFT is no longer requires to pay SOL fees — such as storage fees and SOL mint payments — as the `payer` signer will abstract away those fees. Note that the `minter` signer will still need to pay for token-based fees and will be used to validate the configured guards.

Please note that the latest mint instruction relies on the latest Token Metadata instructions which use a fair amount of compute units. As such, you may need to increase the compute unit limit of the transaction to ensure it is successful. Our SDKs may also help with this.

{% dialect-switcher title="Mint from a Candy Machine" %}
{% dialect title="JavaScript" id="js" %}

To mint from a Candy Machine via a configured Candy Guard account, you may use the `mintV2` function and provide the mint address and update authority of the collection NFT the minted NFT will belong to. A `minter` signer and `payer` signer may also be provided but they will default to Umi's identity and payer respectively.

As mentioned above, you may need to increase the compute unit limit of the transaction to ensure the `mintV2` instruction is successful. You may do this by using the `setComputeUnitLimit` helper function on the `mpl-toolbox` Umi library as illustrated in the code snippet below.

If you want to mint pNFT (e.g. for royalty enforcement) and have your Candy Machine set up accordingly you will have to add the `tokenStandard` field. By default `NonFungible` is used. If you fetched the Candy Machine before you could use `candyMachine.tokenStandard`, otherwise you have to assign it yourself by using `tokenStandard: TokenStandard.ProgrammableNonFungible` from `@metaplex-foundation/mpl-token-metadata`.

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
      tokenStandard: candyMachine.tokenStandard,
    })
  )
  .sendAndConfirm(umi)
```

Note that the `mintV2` instruction takes care of creating the Mint and Token accounts for us by default and will set the NFT owner to the `minter`. If you wish to create these yourself beforehand, you may simply give the NFT mind address as a public key instead of a signer. Here's an example using the `createMintWithAssociatedToken` function from the `mpl-toolbox` Umi library:

```ts
import { mintV2 } from '@metaplex-foundation/mpl-candy-machine'
import {
  createMintWithAssociatedToken,
  setComputeUnitLimit,
} from '@metaplex-foundation/mpl-toolbox'
import { transactionBuilder, generateSigner } from '@metaplex-foundation/umi'

const nftMint = generateSigner(umi)
const nftOwner = generateSigner(umi).publicKey
await transactionBuilder()
  .add(setComputeUnitLimit(umi, { units: 800_000 }))
  .add(createMintWithAssociatedToken(umi, { mint: nftMint, owner: nftOwner }))
  .add(
    mintV2(umi, {
      candyMachine: candyMachine.publicKey,
      nftMint: nftMint.publicKey,
      collectionMint: collectionNft.publicKey,
      collectionUpdateAuthority: collectionNft.metadata.updateAuthority,
    })
  )
  .sendAndConfirm(umi)
```

In the rare event that you wish to mint directly from the Candy Machine Core program, you may use the `mintFromCandyMachineV2` function instead. This function requires the mint authority of the candy machine to be provided as a signer and accepts an explicit `nftOwner` attribute.

```ts
import { mintFromCandyMachineV2 } from '@metaplex-foundation/mpl-candy-machine'
import { setComputeUnitLimit } from '@metaplex-foundation/mpl-toolbox'
import { transactionBuilder, generateSigner } from '@metaplex-foundation/umi'

const nftMint = generateSigner(umi)
const nftOwner = generateSigner(umi).publicKey
await transactionBuilder()
  .add(setComputeUnitLimit(umi, { units: 800_000 }))
  .add(
    mintFromCandyMachineV2(umi, {
      candyMachine: candyMachine.publicKey,
      mintAuthority: umi.identity,
      nftOwner,
      nftMint,
      collectionMint: collectionNft.publicKey,
      collectionUpdateAuthority: collectionNft.metadata.updateAuthority,
    })
  )
  .sendAndConfirm(umi)
```

API References: [mintV2](https://mpl-candy-machine-js-docs.vercel.app/functions/mintV2.html), [mintFromCandyMachineV2](https://mpl-candy-machine-js-docs.vercel.app/functions/mintFromCandyMachineV2.html)

{% /dialect %}
{% /dialect-switcher %}

## Minting With Guards

When minting from a Candy Machine that uses a bunch of guards, you may need to provide additional guard-specific information.

If you were to build the mint instruction manually, that information would be provided as a mixture of instruction data and remaining accounts. However, using our SDKs, each guard that requires additional information at mint time defines a set of settings that we call **Mint Settings**. These Mint Settings will then be parsed into whatever the program needs.

A good example of a guard that requires Mint Settings is the **NFT Payment** guard which requires the mint address of the NFT we should use to pay for the mint amongst other things.

{% diagram %}

{% node %}
{% node #candy-machine-1 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine-1" y=80 x=20 %}
{% node #candy-guard-1 label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node label="Guards" theme="mint" z=1 /%}
{% node #nft-payment-guard label="NFT Payment" /%}
{% node label="Token Payment" /%}
{% node label="Start Date" /%}
{% node #third-party-signer-guard label="Third Party Signer" /%}
{% node label="..." /%}
{% /node %}

{% node parent="candy-machine-1" x=700 %}
{% node #mint-1 label="Mint" theme="pink" /%}
{% node label="Candy Guard Program" theme="pink" /%}
{% /node %}
{% node parent="mint-1" x=45 y=-20 label="Access Control" theme="transparent" /%}

{% node parent="mint-1" x=-22 y=100 %}
{% node #mint-2 label="Mint" theme="pink" /%}
{% node label="Candy Machine Core Program" theme="pink" /%}
{% /node %}
{% node parent="mint-2" x=120 y=-20 label="Mint Logic" theme="transparent" /%}

{% node #nft parent="mint-2" x=62 y=100 label="NFT" /%}

{% node parent="mint-2" x=-400 %}
{% node #mint-settings label="Mint Settings" /%}
{% node label="Using our SDKs" theme="dimmed" /%}
{% /node %}

{% node #mint-args label="Mint Arguments" parent="mint-settings" x=100 y=80 theme="slate" /%}
{% node #mint-accounts label="Mint Remaining Accounts" parent="mint-args" y=50 theme="slate" /%}

{% edge from="candy-guard-1" to="candy-machine-1" fromPosition="left" toPosition="left" arrow=false /%}
{% edge from="mint-1" to="mint-2" theme="pink" path="straight" /%}
{% edge from="mint-2" to="nft" theme="pink" path="straight" /%}
{% edge from="candy-machine-1" to="mint-1" theme="pink" /%}
{% edge from="nft-payment-guard" to="mint-settings" theme="slate" /%}
{% edge from="third-party-signer-guard" to="mint-settings" theme="slate" /%}
{% edge from="mint-settings" to="mint-args" theme="slate" fromPosition="bottom" /%}
{% edge from="mint-settings" to="mint-accounts" theme="slate" fromPosition="bottom" /%}
{% edge from="mint-args" to="mint-1" theme="pink" /%}
{% edge from="mint-accounts" to="mint-1" theme="pink" /%}

{% /diagram %}

[Each available guard](/candy-machine/guards) contains its own documentation page and it will tell you whether or not that guard expects Mint Settings to be provided when minting.

If you were to only use guards that do not require Mint Settings, you may mint in the same way described by the “Basic Minting” section above. Otherwise, you’ll need to provide an additional object attribute containing the Mint Settings of all guards that require them. Let’s have a look at what that looks like in practice using our SDKs.

{% dialect-switcher title="Mint from a Candy Machine with guards" %}
{% dialect title="JavaScript" id="js" %}

When minting via the Umi library, you may use the `mintArgs` attribute to provide the required **Mint Settings**.

Here’s an example using the **Third Party Signer** guard which requires an additional signer and the **Mint Limit** guard which keeps track of how many times a wallet minted from the Candy Machine.

```ts
import {
  some,
  generateSigner,
  transactionBuilder,
} from '@metaplex-foundation/umi'
import { create, mintV2 } from '@metaplex-foundation/mpl-candy-machine'
import { setComputeUnitLimit } from '@metaplex-foundation/mpl-toolbox'

// Create a Candy Machine with guards.
const thirdPartySigner = generateSigner()
await create(umi, {
  // ...
  guards: {
    thirdPartySigner: some({ signer: thirdPartySigner.publicKey }),
    mintLimit: some({ id: 1, limit: 3 }),
  },
}).sendAndConfirm(umi)

// Mint from the Candy Machine.
const nftMint = generateSigner(umi)
await transactionBuilder()
  .add(setComputeUnitLimit(umi, { units: 800_000 }))
  .add(
    mintV2(umi, {
      candyMachine: candyMachine.publicKey,
      nftMint,
      collectionMint: collectionNft.publicKey,
      collectionUpdateAuthority: collectionNft.metadata.updateAuthority,
      mintArgs: {
        thirdPartySigner: some({ signer: thirdPartySigner }),
        mintLimit: some({ id: 1 }),
      },
    })
  )
  .sendAndConfirm(umi)
```

API References: [mintV2](https://mpl-candy-machine-js-docs.vercel.app/functions/mintV2.html), [DefaultGuardSetMintArgs](https://mpl-candy-machine-js-docs.vercel.app/types/DefaultGuardSetMintArgs.html)

{% /dialect %}
{% /dialect-switcher %}

## Minting With Guard Groups

When minting from a Candy Machine using guard groups, **we must explicitly select which group we want to mint from** by providing its label.

Additionally, Mint Settings may also be required as explained in [the previous section](#minting-with-guards). However, **the Mint Settings will apply to the “Resolved Guards” of the selected group**.

For instance, imagine a Candy Machine with the following guards:

- **Default Guards**:
  - Bot Tax
  - Third Party Signer
  - Start Date
- **Group 1**
  - Label: “nft”
  - Guards:
    - NFT Payment
    - Start Date
- **Group 2**
  - Label: “public”
  - Guards:
    - Sol Payment

The Resolved Guards of Group 1 — labelled “nft” — are:

- Bot Tax: from the **Default Guards**.
- Third Party Signer: from the **Default Guards**.
- NFT Payment: from **Group 1**.
- Start Date: from **Group 1** because it overrides the default guard.

Therefore, the provided Mint Settings must be related to these Resolved Guards. In the example above, Mint Settings must be provided for the Third Party Signer guard and the NFT Payment guard.

{% diagram %}

{% node %}
{% node #candy-machine-1 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine-1" y=80 x=20 %}
{% node #candy-guard-1 label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node label="Guards (default guards)" theme="mint" z=1 /%}
{% node label="Bot Tax" /%}
{% node #third-party-signer-guard label="Third Party Signer" /%}
{% node label="Start Date" /%}
{% node #nft-group theme="mint" z=1 %}
Group 1: "nft" {% .font-semibold %}
{% /node %}
{% node #nft-payment-guard label="NFT Payment" /%}
{% node label="Start Date" /%}
{% node theme="mint" z=1 %}
Group 2: "public"
{% /node %}
{% node label="SOL Payment" /%}
{% /node %}

{% node parent="candy-machine-1" x=700 %}
{% node #mint-1 label="Mint" theme="pink" /%}
{% node label="Candy Guard Program" theme="pink" /%}
{% /node %}
{% node parent="mint-1" x=45 y=-20 label="Access Control" theme="transparent" /%}

{% node parent="mint-1" x=-22 y=100 %}
{% node #mint-2 label="Mint" theme="pink" /%}
{% node label="Candy Machine Core Program" theme="pink" /%}
{% /node %}
{% node parent="mint-2" x=120 y=-20 label="Mint Logic" theme="transparent" /%}

{% node #nft parent="mint-2" x=62 y=100 label="NFT" /%}

{% node parent="mint-2" x=-400 y=60 %}
{% node #mint-settings label="Mint Settings" /%}
{% node label="Using our SDKs" theme="dimmed" /%}
{% /node %}

{% node #mint-args label="Mint Arguments" parent="mint-settings" x=100 y=80 theme="slate" /%}
{% node #mint-accounts label="Mint Remaining Accounts" parent="mint-args" y=50 theme="slate" /%}

{% edge from="candy-guard-1" to="candy-machine-1" fromPosition="left" toPosition="left" arrow=false /%}
{% edge from="mint-1" to="mint-2" theme="pink" path="straight" /%}
{% edge from="mint-2" to="nft" theme="pink" path="straight" /%}
{% edge from="candy-machine-1" to="mint-1" theme="pink" /%}
{% edge from="nft-payment-guard" to="mint-settings" theme="slate" /%}
{% edge from="third-party-signer-guard" to="mint-settings" theme="slate" /%}
{% edge from="mint-settings" to="mint-args" theme="slate" fromPosition="bottom" /%}
{% edge from="mint-settings" to="mint-accounts" theme="slate" fromPosition="bottom" /%}
{% edge from="mint-args" to="mint-1" theme="pink" /%}
{% edge from="mint-accounts" to="mint-1" theme="pink" /%}
{% edge from="nft-group" to="mint-1" theme="pink" /%}

{% /diagram %}

{% dialect-switcher title="Mint from a Candy Machine with guard groups" %}
{% dialect title="JavaScript" id="js" %}

When minting from a Candy Machine using guard groups, the label of the group we want to select must be provided via the `group` attribute.

Additionally, the Mint Settings for the Resolved Guards of that group may be provided via the `mintArgs` attribute.

Here is how we would use the Umi library to mint from the example Candy Machine described above.

```ts
// Create a Candy Machine with guards.
const thirdPartySigner = generateSigner()
await create(umi, {
  // ...
  guards: {
    botTax: some({ lamports: sol(0.001), lastInstruction: true }),
    thirdPartySigner: some({ signer: thirdPartySigner.publicKey }),
    startDate: some({ date: dateTime('2022-10-18T17:00:00Z') }),
  },
  groups: [
    {
      label: 'nft',
      guards: {
        nftPayment: some({ requiredCollection, destination: nftTreasury }),
        startDate: some({ date: dateTime('2022-10-18T16:00:00Z') }),
      },
    },
    {
      label: 'public',
      guards: {
        solPayment: some({ lamports: sol(1), destination: solTreasury }),
      },
    },
  ],
}).sendAndConfirm(umi)

// Mint from the Candy Machine.
const nftMint = generateSigner(umi)
await transactionBuilder()
  .add(setComputeUnitLimit(umi, { units: 800_000 }))
  .add(
    mintV2(umi, {
      candyMachine: candyMachine.publicKey,
      nftMint,
      collectionMint: collectionNft.publicKey,
      collectionUpdateAuthority: collectionNft.metadata.updateAuthority,
      group: some('nft'),
      mintArgs: {
        thirdPartySigner: some({ signer: thirdPartySigner }),
        nftPayment: some({
          mint: nftFromRequiredCollection.publicKey,
          destination: nftTreasury,
          tokenStandard: TokenStandard.NonFungible,
        }),
      },
    })
  )
  .sendAndConfirm(umi)
```

API References: [mintV2](https://mpl-candy-machine-js-docs.vercel.app/functions/mintV2.html), [DefaultGuardSetMintArgs](https://mpl-candy-machine-js-docs.vercel.app/types/DefaultGuardSetMintArgs.html)

{% /dialect %}
{% /dialect-switcher %}

## Minting With Pre-Validation

It is important to note that some guards may require additional verification steps before we can mint from their Candy Machine. This pre-validation step usually creates an account on the blockchain or rewards the wallet with a token that acts as proof of that verification.

### Using the route instruction

One way guards can require a pre-validation step is by using [their own special instruction](/candy-machine/guard-route) via the “route” instruction.

A good example of that is the **Allow List** guard. When using this guard, we must verify that our wallet belongs to a predefined list of wallets by calling the route instruction and providing a valid Merkle Proof. If this route instruction is successful, it will create an Allow List PDA for that wallet which the mint instruction can then read to validate the Allow List guard. [You can read more about the Allow List guard on its dedicated page](/candy-machine/guards/allow-list).

{% diagram %}

{% node %}
{% node #candy-machine-1 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine-1" y=80 x=20 %}
{% node #candy-guard-1 label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node label="Guards" theme="mint" z=1 /%}
{% node #allow-list-guard label="Allow List" /%}
{% node label="..." /%}
{% /node %}

{% node parent="candy-machine-1" x=550 %}
{% node #mint-1 label="Mint" theme="pink" /%}
{% node label="Candy Guard Program" theme="pink" /%}
{% /node %}
{% node parent="mint-1" x=45 y=-20 label="Access Control" theme="transparent" /%}

{% node parent="mint-1" x=-22 y=100 %}
{% node #mint-2 label="Mint" theme="pink" /%}
{% node label="Candy Machine Core Program" theme="pink" /%}
{% /node %}
{% node parent="mint-2" x=120 y=-20 label="Mint Logic" theme="transparent" /%}

{% node #nft parent="mint-2" x=62 y=100 label="NFT" /%}

{% node parent="mint-2" x=-250 %}
{% node #route label="Route" theme="pink" /%}
{% node label="Candy Machine Core Program" theme="pink" /%}
{% /node %}
{% node parent="route" x=70 y=-20 label="Verify Merkle Proof" theme="transparent" /%}

{% node #allow-list-pda parent="route" x=23 y=100 label="Allow List PDA" /%}

{% edge from="candy-guard-1" to="candy-machine-1" fromPosition="left" toPosition="left" arrow=false /%}
{% edge from="mint-1" to="mint-2" theme="pink" path="straight" /%}
{% edge from="mint-2" to="nft" theme="pink" path="straight" /%}
{% edge from="candy-machine-1" to="mint-1" theme="pink" /%}
{% edge from="allow-list-guard" to="route" theme="pink" /%}
{% edge from="route" to="allow-list-pda" theme="pink" path="straight" /%}
{% edge from="allow-list-pda" to="mint-1" theme="pink" /%}

{% /diagram %}

### Using external services

Another way guards may perform that pre-validation step is by relying on an external solution.

For instance, when using the **Gatekeeper** guard, we must request a Gateway Token by performing a challenge — such as completing a Captcha — which depends on the configured Gatekeeper Network. The Gatekeeper guard will then check for the existence of such Gateway Token to either validate or reject the mint. [You can learn more about the Gatekeeper guard on its dedicated page](/candy-machine/guards/gatekeeper).

{% diagram %}

{% node %}
{% node #candy-machine-1 label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine-1" y=80 x=20 %}
{% node #candy-guard-1 label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node label="Guards" theme="mint" z=1 /%}
{% node #gatekeeper-guard label="Gatekeeper" /%}
{% node label="..." /%}
{% /node %}

{% node parent="candy-machine-1" x=550 %}
{% node #mint-1 label="Mint" theme="pink" /%}
{% node label="Candy Guard Program" theme="pink" /%}
{% /node %}
{% node parent="mint-1" x=45 y=-20 label="Access Control" theme="transparent" /%}

{% node parent="mint-1" x=-22 y=100 %}
{% node #mint-2 label="Mint" theme="pink" /%}
{% node label="Candy Machine Core Program" theme="pink" /%}
{% /node %}
{% node parent="mint-2" x=120 y=-20 label="Mint Logic" theme="transparent" /%}

{% node #nft parent="mint-2" x=62 y=100 label="NFT" /%}

{% node parent="mint-2" x=-250 y=-40 %}
{% node #network label="Gatekeeper Network" theme="slate" /%}
{% node theme="slate" %}
Request Gateway Token \
from the Gatekeeper \
Network, e.g. Captcha.
{% /node %}
{% /node %}

{% node #gateway-token parent="network" x=23 y=140 label="Gateway Token" /%}

{% edge from="candy-guard-1" to="candy-machine-1" fromPosition="left" toPosition="left" arrow=false /%}
{% edge from="mint-1" to="mint-2" theme="pink" path="straight" /%}
{% edge from="mint-2" to="nft" theme="pink" path="straight" /%}
{% edge from="candy-machine-1" to="mint-1" theme="pink" /%}
{% edge from="gatekeeper-guard" to="network" theme="slate" /%}
{% edge from="network" to="gateway-token" theme="slate" path="straight" /%}
{% edge from="gateway-token" to="mint-1" theme="pink" /%}

{% /diagram %}

## Minting With Bot Taxes

One guard you’ll likely want to include in your Candy Machine is the Box Tax guard which protects your Candy Machine against bots by charging failed mints a configurable amount of SOL. This amount is usually small to hurt bots without affecting genuine mistakes from real users. All bot taxes will be transferred to the Candy Machine account so that, once minting is over, you can access these funds by deleting the Candy Machine account.

This guard is a bit special and affects the minting behaviour of all other guards. When the Bot Tax is activated and any other guard fails to validate the mint, **the transaction will pretend to succeed**. This means no errors will be returned by the program but no NFT will be minted either. This is because the transaction must succeed for the funds to be transferred from the bot to the Candy Machine account. [You can learn more about the Bot Tax guard on its dedicated page](/candy-machine/guards/bot-tax).

## Conclusion

Congratulations, you now know how Candy Machines work from A to Z!

Here are some additional reading resources you might be interested in:

- [All Available Guards](/candy-machine/guards): Have a look through all the guards available to you so you can cherry-pick the ones you need.
- _Create Your First Candy Machine (coming soon)_: This How-To guide helps you upload your assets and create a new Candy Machine from scratch using a CLI tool called “[Sugar](/candy-machine/sugar)”. It also uses our JS SDK to spin up a minting website for your Candy Machine.
