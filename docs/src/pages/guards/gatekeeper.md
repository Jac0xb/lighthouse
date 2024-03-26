---
title: "Gatekeeper"
metaTitle: "Candy Machine Guards - Gatekeeper"
description: "The Gatekeeper guard checks whether the minting wallet has a valid Gateway Token from a specified Gatekeeper Network."
---

## Overview

The **Gatekeeper** guard checks whether the minting wallet has a valid **Gateway Token** from a specified **Gatekeeper Network**.

In most cases, this token will be obtained after completing a Captcha challenge but any Gatekeeper Network may be used.

There isn’t much to set up on the Candy Machine side but, depending on the selected Gatekeeper Network, you may need to ask the minting wallet to perform so pre-validation checks to grant them the required Gateway Token.

Here are some additional recommended materials you may find helpful when setting up a Gatekeep Network.

- [The CIVIC Documentation](https://docs.civic.com/civic-pass/overview)
- [Gateway JS Library](https://www.npmjs.com/package/@identity.com/solana-gateway-ts)
- [Gateway React Components](https://www.npmjs.com/package/@civic/solana-gateway-react)

{% diagram  %}

{% node %}
{% node #candy-machine label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine" y="100" x="22" %}
{% node #candy-guard label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node #candy-guard-guards label="Guards" theme="mint" z=1/%}
{% node #gatekeeper label="Gatekeeper" /%}
{% node #gatekeeper-network label="- Gatekeeper Network" /%}
{% node #expire label="- Expire on use" /%}
{% node label="..." /%}
{% /node %}

{% node parent="gatekeeper" x="250" y="-17" %}
{% node #request-token theme="indigo" %}
Request Gateway Token

from the Gatekeeper

Network e.g. Captcha
{% /node %}
{% /node %}

{% node parent="request-token" y="140" x="34" %}
{% node #gateway-token theme="indigo" label="Gateway Token" /%}
{% /node %}

{% node parent="candy-machine" x="600" %}
  {% node #mint-candy-guard theme="pink" %}
    Mint from

    _Candy Guard Program_
  {% /node %}
{% /node %}
{% node parent="mint-candy-guard" y="-20" x="100" theme="transparent" %}
  Access Control
{% /node %}

{% node parent="mint-candy-guard" y="150" x="-9" %}
  {% node #mint-candy-machine theme="pink" %}
    Mint from 
    
    _Candy Machine Program_
  {% /node %}
{% /node %}
{% node parent="mint-candy-machine" y="-20" x="140" theme="transparent" %}
  Mint Logic
{% /node %}

{% node #nft parent="mint-candy-machine" y="140" x="78" theme="blue" %}
  NFT
{% /node %}
{% edge from="mint-candy-machine" to="nft" path="straight" /%}

{% edge from="candy-guard" to="candy-machine" /%}
{% edge from="gatekeeper-network" to="request-token" /%}
{% edge from="request-token" to="gateway-token" /%}

{% edge from="gateway-token" to="mint-candy-guard" arrow="none" dashed=true /%}
{% node theme="transparent" parent="mint-candy-guard" x="-210" %}
if a valid token for the given

Network and payer does not exist 

Minting will fail
{% /node %}
{% edge from="mint-candy-guard" to="mint-candy-machine" path="straight" /%}


{% /diagram %}
## Guard Settings

The Gatekeeper guard contains the following settings:

- **Gatekeeper Network**: The public key of the Gatekeeper Network that will be used to check the validity of the minting wallet. For instance, you may use the "**Civic Captcha Pass**" Network — which ensures the minting wallet has passed a captcha — by using the following address: `ignREusXmGrscGNUesoU9mxfds9AiYTezUKex2PsZV6`.
- **Expire On Use**: Whether we should mark the Gateway Token of the minting wallet as expired after the NFT has been minting.
  - When set to `true`, they will need to go through the Gatekeeper Network again to mint another NFT.
  - When set to `false`, they will be able to mint another NFT until the Gateway Token expires naturally.

{% dialect-switcher title="Set up a Candy Machine using the Gatekeeper guard" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
create(umi, {
  // ...
  guards: {
    gatekeeper: some({
      network: publicKey("ignREusXmGrscGNUesoU9mxfds9AiYTezUKex2PsZV6"),
      expireOnUse: true,
    }),
  },
});
```

API References: [create](https://mpl-candy-machine-js-docs.vercel.app/functions/create.html), [Gatekeeper](https://mpl-candy-machine-js-docs.vercel.app/types/Gatekeeper.html)

{% /totem %}
{% /dialect %}
{% dialect title="Sugar" id="sugar" %}
{% totem %}

Add this object into the guard section your config.json file:

```json
"gatekeeper" : {
    "gatekeeperNetwork": "<PUBKEY>",
    "expireOnUse": boolean
}
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Mint Settings

The Gatekeeper guard accepts the following mint settings:

- **Gatekeeper Network**: The public key of the Gatekeeper Network that will be used to check the validity of the minting wallet.
- **Expire On Use**: Whether we should mark the Gateway Token of the minting wallet as expired after the NFT has been minting.
- **Token Account** (optional): As a little disclaimer, you should very rarely need to provide this setting but it’s here if you need to. This refers to the Gateway Token PDA derived from the payer and the Gatekeeper Network which is used to verify the payer's eligibility to mint. This PDA address can be inferred by our SDKs which is why you do not need to provide it. However, some Gatekeeper Networks may issue multiple Gateway Tokens to the same wallet. To differentiate their PDA addresses, it uses a **Seeds** array which defaults to `[0, 0, 0, 0, 0, 0, 0, 0]`.

Note that, if you’re planning on constructing instructions without the help of our SDKs, you will need to provide these Mint Settings and more as a combination of instruction arguments and remaining accounts. See the [Candy Guard’s program documentation](https://github.com/metaplex-foundation/mpl-candy-machine/tree/main/programs/candy-guard#gatekeeper) for more details.

{% dialect-switcher title="Mint with the Gatekeeper Guard" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

You may pass the Mint Settings of the Gatekeeper guard using the `mintArgs` argument like so.

```ts
mintV2(umi, {
  // ...
  mintArgs: {
    gatekeeper: some({
      network: publicKey("ignREusXmGrscGNUesoU9mxfds9AiYTezUKex2PsZV6"),
      expireOnUse: true,
    }),
  },
});
```
{% /totem %}
{% /dialect %}
{% dialect title="Sugar" id="sugar" %}
{% totem %}

_As soon as a guard is assigned you cannot use sugar to mint - therefore there are no specific mint settings._

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Route Instruction

_The Gatekeeper guard does not support the route instruction._
