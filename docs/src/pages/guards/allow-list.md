---
title: "Allowlist"
metaTitle: "Candy Machine Guards - Allowlist"
description: "Uses a wallet address list to determine who is allowed to mint."
---

## Overview

The **Allow List** guard validates the minting wallet against a predefined list of wallets. If the minting wallet is not part of this list, minting will fail.

Providing a big list of wallets in the settings of this guard would require a lot of storage on the blockchain and would likely need more than one transaction to insert them all. Therefore, the Allow List guard uses [**Merkle Trees**](https://en.m.wikipedia.org/wiki/Merkle_tree) to verify that the minting wallet is part of the preconfigured list of wallets.

This works by creating a binary tree of hashes where all leaves hash themselves two by two until we reach the final hash known as the **Merkle Root**. This means that if any leaf were to change, the final Merkle Root would be corrupted.

{% diagram %}
{% node #hash-7 label="Hash 7" theme="brown" /%}
{% node #merkle-root label="Merkle Root" theme="transparent" parent="hash-7" x="-90" y="8" /%}
{% node #hash-5 label="Hash 5" parent="hash-7" y="100" x="-200" theme="orange" /%}
{% node #hash-6 label="Hash 6" parent="hash-7" y="100" x="200" theme="orange" /%}

{% node #leaves label="Leaves" parent="hash-5" y="105" x="-170" theme="transparent" /%}
{% node #hash-1 label="Hash 1" parent="hash-5" y="100" x="-100" theme="orange" /%}
{% node #hash-2 label="Hash 2" parent="hash-5" y="100" x="100" theme="orange" /%}
{% node #hash-3 label="Hash 3" parent="hash-6" y="100" x="-100" theme="orange" /%}
{% node #hash-4 label="Hash 4" parent="hash-6" y="100" x="100" theme="orange" /%}

{% node #data label="Data" parent="hash-1" y="105" x="-80" theme="transparent" /%}
{% node #Ur1C label="Ur1C...bWSG" parent="hash-1" y="100" x="-23" /%}
{% node #sXCd label="sXCd...edkn" parent="hash-2" y="100" x="-20" /%}
{% node #RbJs label="RbJs...Ek7u" parent="hash-3" y="100" x="-17" /%}
{% node #rwAv label="rwAv...u1ud" parent="hash-4" y="100" x="-16" /%}

{% edge from="hash-5" to="hash-7" fromPosition="top" toPosition="bottom" /%}
{% edge from="hash-6" to="hash-7" fromPosition="top" toPosition="bottom" /%}

{% edge from="hash-1" to="hash-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="hash-2" to="hash-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="hash-3" to="hash-6" fromPosition="top" toPosition="bottom" /%}
{% edge from="hash-4" to="hash-6" fromPosition="top" toPosition="bottom" /%}

{% edge from="Ur1C" to="hash-1" fromPosition="top" toPosition="bottom" path="straight" /%}
{% edge from="sXCd" to="hash-2" fromPosition="top" toPosition="bottom" path="straight" /%}
{% edge from="RbJs" to="hash-3" fromPosition="top" toPosition="bottom" path="straight" /%}
{% edge from="rwAv" to="hash-4" fromPosition="top" toPosition="bottom" path="straight" /%}

{% /diagram %}

To verify that a leaf is part of the tree, we simply need a list of all the intermediary hashes that allow us to go up the tree and re-compute the Merkle Root. We call this list of intermediary hashes a **Merkle Proof**. If the computed Merkle Root matches the stored Merkle Root, we can be sure that the leaf is part of the tree and therefore part of the original list.

{% diagram %}
{% node #hash-7 label="Hash 7" theme="brown" /%}
{% node #merkle-root label="Merkle Root" theme="transparent" parent="hash-7" x="-90" y="8" /%}
{% node #hash-5 label="Hash 5" parent="hash-7" y="100" x="-200" theme="mint" /%}
{% node #hash-6 label="Hash 6" parent="hash-7" y="100" x="200" theme="blue" /%}

{% node #legend-merkle-proof label="Merkle Proof =" theme="transparent" parent="hash-7" x="200" y="10" /%}
{% node #legend-hash-4 label="Hash 4" parent="legend-merkle-proof" x="100" y="-7" theme="mint" /%}
{% node #plus label="+" parent="legend-hash-4" theme="transparent" x="81" y="8" /%}
{% node #legend-hash-5 label="Hash 5" parent="legend-hash-4" x="100" theme="mint" /%}


{% node #leaves label="Leaves" parent="hash-5" y="105" x="-170" theme="transparent" /%}
{% node #hash-1 label="Hash 1" parent="hash-5" y="100" x="-100" theme="orange" /%}
{% node #hash-2 label="Hash 2" parent="hash-5" y="100" x="100" theme="orange" /%}
{% node #hash-3 label="Hash 3" parent="hash-6" y="100" x="-100" theme="blue" /%}
{% node #hash-4 label="Hash 4" parent="hash-6" y="100" x="100" theme="mint" /%}

{% node #data label="Data" parent="hash-1" y="105" x="-80" theme="transparent" /%}
{% node #Ur1C label="Ur1C...bWSG" parent="hash-1" y="100" x="-23" /%}
{% node #sXCd label="sXCd...edkn" parent="hash-2" y="100" x="-20" /%}
{% node #RbJs label="RbJs...Ek7u" parent="hash-3" y="100" x="-17" theme="blue" /%}
{% node #rwAv label="rwAv...u1ud" parent="hash-4" y="100" x="-16" /%}

{% edge from="hash-5" to="hash-7" fromPosition="top" toPosition="bottom" theme="mint" /%}
{% edge from="hash-6" to="hash-7" fromPosition="top" toPosition="bottom" theme="blue" /%}

{% edge from="hash-1" to="hash-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="hash-2" to="hash-5" fromPosition="top" toPosition="bottom" /%}
{% edge from="hash-3" to="hash-6" fromPosition="top" toPosition="bottom" theme="blue" /%}
{% edge from="hash-4" to="hash-6" fromPosition="top" toPosition="bottom" theme="mint" /%}

{% edge from="Ur1C" to="hash-1" fromPosition="top" toPosition="bottom" path="straight" /%}
{% edge from="sXCd" to="hash-2" fromPosition="top" toPosition="bottom" path="straight" /%}
{% edge from="RbJs" to="hash-3" fromPosition="top" toPosition="bottom" path="straight" theme="blue" /%}
{% edge from="rwAv" to="hash-4" fromPosition="top" toPosition="bottom" path="straight" /%}

{% /diagram %}

Therefore, the Allow List guard’s settings require a Merkle Root which acts as a source of truth for the preconfigured list of allowed wallets. For a wallet to prove it is on the allowed list, it must provide a valid Merkle Proof that allows the program to re-compute the Merkle Root and ensure it matches the guard’s settings.

Note that our SDKs provide helpers to make it easy to create Merkle Root and Merkle Proofs for a given list of wallets.

{% diagram  %}

{% node %}
{% node #candy-machine label="Candy Machine" theme="blue" /%}
{% node theme="dimmed" %}
Owner: Candy Machine Core Program {% .whitespace-nowrap %}
{% /node %}
{% /node %}

{% node parent="candy-machine" y="100" x="20" %}
{% node #candy-guard label="Candy Guard" theme="blue" /%}
{% node theme="dimmed" %}
Owner: Candy Guard Program {% .whitespace-nowrap %}
{% /node %}
{% node #candy-guard-guards label="Guards" theme="mint" z=1 /%}
{% node #allowList label="AllowList" /%}
{% node #guardMerkleRoot label="- Merkle Root" /%}
{% node label="..." /%}
{% /node %}

{% node parent="allowList" x="250" y="10" %}
{% node #merkleRoot theme="slate" %}
Merkle Root {% .whitespace-nowrap %}
{% /node %}
{% /node %}

{% node parent="merkleRoot" x="170" %}
{% node #merkleProof theme="slate" %}
Merkle Proof {% .whitespace-nowrap %}
{% /node %}
{% /node %}

{% node parent="merkleRoot" y="100" x="-12" %}
{% node #walletList  %}
List of wallets

allowed to mint
{%/node %}
{% /node %}
{% edge from="merkleProof" to="walletList" arrow="none" fromPosition="bottom" toPosition="top" arrow="start" /%}
{% edge from="merkleRoot" to="walletList" arrow="none" fromPosition="bottom" toPosition="top" arrow="start" /%}


{% node parent="merkleProof" y="100" %}
{% node #payer label="Payer" theme="indigo" /%}
{% node theme="dimmed"%}
Owner: Any Program {% .whitespace-nowrap %}
{% /node %}
{% /node %}
{% edge from="merkleProof" to="payer" arrow="none" fromPosition="bottom" toPosition="top" arrow="start" path="straight" /%}

{% node parent="candy-machine" x="740" %}
  {% node #route-validation theme="pink" %}
    Route from the

    _Candy Guard Program_
  {% /node %}
{% /node %}
{% node parent="route-validation" y="-20" x="100" theme="transparent" %}
  Verify Merkle Proof
{% /node %}

{% node parent="route-validation" #allowList-pda y="130" x="32" %}
{% node theme="slate" %}
Allowlist PDA {% .whitespace-nowrap %}
{% /node %}
{% /node %}

{% node parent="allowList-pda" #mint-candy-guard y="90" x="-31" %}
  {% node theme="pink" %}
    Mint from

    _Candy Guard Program_ {% .whitespace-nowrap %}
  {% /node %}
{% /node %}
{% node parent="mint-candy-guard" y="-20" x="100" theme="transparent" %}
  Access Control
{% /node %}

{% node parent="mint-candy-guard" #mint-candy-machine y="110" x="-8" %}
  {% node theme="pink" %}
    Mint from 
    
    _Candy Machine Program_ {% .whitespace-nowrap %}
  {% /node %}
{% /node %}
{% node parent="mint-candy-machine" y="-20" x="110" theme="transparent" %}
  Mint Logic
{% /node %}

{% node #nft parent="mint-candy-machine" y="110" x="70" theme="blue" %}
  NFT
{% /node %}
{% edge from="mint-candy-machine" to="nft" path="straight" /%}

{% edge from="candy-guard" to="candy-machine" /%}
{% edge from="guardMerkleRoot" to="merkleRoot" arrow="start" path="straight" /%}
{% edge from="merkleRoot" to="route-validation" arrow="none" fromPosition="top" dashed=true /%}
{% edge from="merkleProof" to="route-validation" arrow="none" fromPosition="top" dashed=true  %}
if the payer's Merkle Proof does not match 

the guard's Merkle Root validation will fail
{% /edge %}
{% edge from="candy-guard-guards" to="guards" /%}
{% edge from="route-validation" to="allowList-pda" path="straight" /%}
{% edge from="allowList-pda" to="mint-candy-guard" path="straight" /%}
{% edge from="mint-candy-guard" to="mint-candy-machine" path="straight" /%}


{% /diagram %}

## Guard Settings

The Allow List guard contains the following settings:

- **Merkle Root**: The Root of the Merkle Tree representing the allow list.

{% dialect-switcher title="Set up a Candy Machine using the Allowlist guard" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

To help us manage Merkle Trees, the Umi library provides two helper methods called `getMerkleRoot` and `getMerkleProof` that you may use like so.

```ts
import {
  getMerkleProof,
  getMerkleRoot,
} from "@metaplex-foundation/mpl-candy-machine";

const allowList = [
  "Ur1CbWSGsXCdedknRbJsEk7urwAvu1uddmQv51nAnXB",
  "GjwcWFQYzemBtpUoN5fMAP2FZviTtMRWCmrppGuTthJS",
  "AT8nPwujHAD14cLojTcB1qdBzA1VXnT6LVGuUd6Y73Cy",
];

const merkleRoot = getMerkleRoot(allowList);
const validMerkleProof = getMerkleProof(
  allowList,
  "Ur1CbWSGsXCdedknRbJsEk7urwAvu1uddmQv51nAnXB"
);
const invalidMerkleProof = getMerkleProof(allowList, "invalid-address");
```

Once we have computed the Merkle Root of our allow list, we can use it to set up the Allow List guard on our Candy Machine.

```ts
import { getMerkleRoot } from "@metaplex-foundation/mpl-candy-machine";

const allowList = [
  "Ur1CbWSGsXCdedknRbJsEk7urwAvu1uddmQv51nAnXB",
  "GjwcWFQYzemBtpUoN5fMAP2FZviTtMRWCmrppGuTthJS",
  "AT8nPwujHAD14cLojTcB1qdBzA1VXnT6LVGuUd6Y73Cy",
];

create(umi, {
  // ...
  guards: {
    allowList: some({ merkleRoot: getMerkleRoot(allowList) }),
  },
});
```

API References: [create](https://mpl-candy-machine-js-docs.vercel.app/functions/create.html), [AllowList](https://mpl-candy-machine-js-docs.vercel.app/types/AllowList.html)

{% /totem %}
{% /dialect %}
{% dialect title="Sugar" id="sugar" %}
{% totem %}

Sugar does not contain a function to create manage the merkle root. When using a allowlist with sugar you would have to compute it beforehand e.g. with the previously described JavaScript function or [sol-tools](https://sol-tools.tonyboyle.io/cmv3/allow-list) and than add the merkle root hash to your config like this:

```json
"allowList" : {
    "merkleRoot": "<HASH>"
}
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Mint Settings

The Allow List guard contains the following Mint Settings:

- **Merkle Root**: The Root of the Merkle Tree representing the allow list.

Note that, before being able to mint, **we must validate the minting wallet by providing a Merkle Proof**. See [Validate a Merkle Proof](#validate-a-merkle-proof) below for more details.

Also note that, if you’re planning on constructing instructions without the help of our SDKs, you will need to add the Allow List Proof PDA to the remaining accounts of the mint instruction. See the [Candy Guard’s program documentation](https://github.com/metaplex-foundation/mpl-candy-machine/tree/main/programs/candy-guard#allowlist) for more details.

{% dialect-switcher title="Mint with the Allow List guard" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

You may pass the Mint Settings of the Allow List guard using the `mintArgs` argument like so.

```ts
import { getMerkleRoot } from "@metaplex-foundation/mpl-candy-machine";

const allowList = [
  "Ur1CbWSGsXCdedknRbJsEk7urwAvu1uddmQv51nAnXB",
  "GjwcWFQYzemBtpUoN5fMAP2FZviTtMRWCmrppGuTthJS",
  "AT8nPwujHAD14cLojTcB1qdBzA1VXnT6LVGuUd6Y73Cy",
];

mintV2(umi, {
  // ...
  mintArgs: {
    allowList: some({ merkleRoot: getMerkleRoot(allowList) }),
  },
});
```

API References: [mintV2](https://mpl-candy-machine-js-docs.vercel.app/functions/mintV2.html), [AllowListMintArgs](https://mpl-candy-machine-js-docs.vercel.app/types/AllowListMintArgs.html)

{% /totem %}
{% /dialect %}
{% dialect title="Sugar" id="sugar" %}
{% totem %}

_As soon as a guard is assigned you cannot use sugar to mint - therefore there are no specific mint settings._

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Route Instruction

The Allow List route instruction supports the following features.

### Validate a Merkle Proof

_Path: `proof`_

Instead of passing the Merkle Proof directly to the mint instruction, the minting wallet must perform a [Pre-Validation](/programs/candy-machine/minting#minting-with-pre-validation) by using the route instruction of the Allow List guard.

This route instruction will compute the Merkle Root from the provided Merkle Proof and, if valid, will create a new PDA account acting as proof that the minting wallet is part of the allowed list. Therefore, when minting, the Allow List guard only needs to check for the existence of this PDA account to authorize or deny minting to the wallet.

So why can’t we just verify the Merkle Proof directly within the mint instruction? That’s simply because, for big allow lists, Merkle Proofs can end up being pretty lengthy. After a certain size, it becomes impossible to include it within the mint transaction that already contains a decent amount of instructions. By separating the validation process from the minting process, we make it possible for allow lists to be as big as we need them to be.

This path of the route instruction accepts the following arguments:

- **Path** = `proof`: Selects the path to execute in the route instruction.
- **Merkle Root**: The Root of the Merkle Tree representing the allow list.
- **Merkle Proof**: The list of intermediary hashes that should be used to compute the Merkle Root and verify that it matches the Merkle Root stored on the guard’s settings.
- **Minter** (optional): The minter account as a signer if it is not the same as the payer. When provided, this account must be part of the allow list for the proof to be valid.

{% dialect-switcher title="Pre-Validate a Wallet" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

You may pass the "Proof" Route Settings of the Allow List guard using the `routeArgs` argument like so.

```ts
import {
  getMerkleProof,
  getMerkleRoot,
} from "@metaplex-foundation/mpl-candy-machine";
import { publicKey } from "@metaplex-foundation/umi";

const allowList = [
  "Ur1CbWSGsXCdedknRbJsEk7urwAvu1uddmQv51nAnXB",
  "GjwcWFQYzemBtpUoN5fMAP2FZviTtMRWCmrppGuTthJS",
  "AT8nPwujHAD14cLojTcB1qdBzA1VXnT6LVGuUd6Y73Cy",
];

await route(umi, {
  // ...
  guard: "allowList",
  routeArgs: {
    path: "proof",
    merkleRoot: getMerkleRoot(allowList),
    merkleProof: getMerkleProof(allowList, publicKey(umi.identity)),
  },
}).sendAndConfirm(umi);
```

The `umi.identity` wallet is now allowed to mint from the Candy Machine.

API References: [route](https://mpl-candy-machine-js-docs.vercel.app/functions/route.html), [AllowListRouteArgs](https://mpl-candy-machine-js-docs.vercel.app/types/AllowListRouteArgs.html)

{% /totem %}
{% /dialect %}
{% dialect title="Sugar" id="sugar" %}
{% totem %}

_Sugar can not be used to call the "Proof" Route._ 

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}