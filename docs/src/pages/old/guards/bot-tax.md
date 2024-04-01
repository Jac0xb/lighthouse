---
title: "Bot Tax"
metaTitle: "Candy Machine Guards - Bot Tax"
description: "Configurable tax to charge invalid transactions."
---

## Overview

The **Bot Tax** guard charges a penalty for invalid transactions to discourage bots from attempting to mint NFTs. This amount is usually small to hurt bots without affecting genuine mistakes from real users. All bot taxes will be transferred to the Candy Machine account so that, once minting is over, you can access these funds by deleting the Candy Machine account.

This guard is a bit special and affects the minting behaviour of all other guards. When the Bot Tax is activated and any other guard fails to validate the mint, **the transaction will pretend to succeed**. This means no errors will be returned by the program but no NFT will be minted either. This is because the transaction must succeed for the funds to be transferred from the bot to the Candy Machine account.

Additionally, the Bot Tax guard enables us to ensure the mint instruction was the last instruction of the transaction. This prevents bots from adding malicious instructions after the mint and returns an error to avoid paying the tax.

{% diagram  %}

{% node %}
{% node #candy-machine label="Candy Machine" theme="blue" /%}
{% node label="Owner: Candy Machine Core Program" theme="dimmed" /%}
{% /node %}

{% node parent="candy-machine" y="100" x="22" %}
{% node #candy-guard label="Candy Guard" theme="blue" /%}
{% node label="Owner: Candy Guard Program" theme="dimmed" /%}
{% node #candy-guard-guards label="Guards" theme="mint" z=1 /%}
{% node #botTax label="botTax" /%}
{% node #lamports label="- Lamports" /%}
{% node #lastInstruction label="- Last Instruction" /%}
{% node label="..." /%}
{% /node %}

{% node parent="candy-machine" x="700" %}
  {% node #mint-candy-guard theme="pink" %}
    Mint from

    _Candy Guard Program_
  {% /node %}
{% /node %}
{% node parent="mint-candy-guard" y="-20" x="100" theme="transparent" %}
  Access Control
{% /node %}

{% node parent="mint-candy-guard" y="150" x="-8" %}
  {% node #mint-candy-machine theme="pink" %}
    Mint from 
    
    _Candy Machine Program_
  {% /node %}
{% /node %}
{% node parent="mint-candy-machine" y="-20" x="110" theme="transparent" %}
  Mint Logic
{% /node %}

{% node #nft parent="mint-candy-machine" y="120" x="76" theme="blue" %}
  NFT
{% /node %}
{% edge from="mint-candy-machine" to="nft" path="straight" /%}

{% edge from="candy-guard" to="candy-machine" /%}
{% edge from="lamports" to="mint-candy-guard" arrow="none" dashed=true /%}
{% node parent="lamports" y="-30" x="200" theme="transparent" %}
If any other guard fails to validate

charge this amount of SOL
{% /node %}
{% edge from="lastInstruction" to="mint-candy-guard" arrow="none" dashed=true %}

{% /edge %}
{% node parent="lastInstruction" y="15" x="200" theme="transparent" %}
If the mint instruction is not the last

Instruction of the transaction minting will fail
{% /node %}
{% edge from="candy-guard-guards" to="guards" /%}
{% edge from="mint-candy-guard" to="mint-candy-machine" path="straight" /%}


{% /diagram %}

## Guard Settings

The Bot Tax guard contains the following settings:

- **Lamports**: The amount in SOL (or lamports) to charge for an invalid transaction. We recommend setting a fairly small amount to avoid affecting real users who made a genuine mistake. Client-side validation can also help reduce affecting real users.
- **Last Instruction**: Whether or not we should forbid minting and charge a bot tax when the mint instruction is not the last instruction of the transaction. We recommend setting this to `true` to be better protected against bots.

{% dialect-switcher title="Set up a Candy Machine using the Bot Tax guard" %}
{% dialect title="JavaScript" id="js" %}
{% totem %}

```ts
create(umi, {
  // ...
  guards: {
    botTax: some({
      lamports: sol(0.01),
      lastInstruction: true,
    }),
  },
});
```

API References: [create](https://mpl-candy-machine-js-docs.vercel.app/functions/create.html), [BotTax](https://mpl-candy-machine-js-docs.vercel.app/types/BotTax.html)

{% /totem %}
{% /dialect %}
{% dialect title="Sugar" id="sugar" %}
{% totem %}

Add this object into the guard section your config.json file: 

```json
"botTax" : {
    "value": SOL value,
    "lastInstruction": boolean
}
```

{% /totem %}
{% /dialect %}
{% /dialect-switcher %}

## Mint Settings

_The Bot Tax guard does not need Mint Settings._

## Route Instruction

_The Bot Tax guard does not support the route instruction._
