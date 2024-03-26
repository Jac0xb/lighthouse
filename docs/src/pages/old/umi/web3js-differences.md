---
title: Umi and Web3js Differences
metaTitle: Umi - Umi and Web3js Differences
description: Differences between Metaplex Umi and Solana web3js.
---

## Differences

When using Umi you may come across some differences between Umi and solana web3js.

Although having the same or similar import names these function differently and are not compatible between the two librarys out the box.

### PublicKeys

{% dialect-switcher title="Umi PublicKey" %}
{% dialect title="JavaScript" id="js" %}
```ts
import { publicKey } from '@metaplex-foundation/umi'
const publicKey = publicKey('tst24HZ6pbcnraCv4r8acexfgXvyQwMSRgZRCg9gEX1')
```
{% /dialect %}
{% /dialect-switcher %}
{% seperator h="6" /%}
{% dialect-switcher title="Solana Web3js PublicKey" %}
{% dialect title="JavaScript" id="js" %}
```js
import { PublicKey } from '@solana/web3js'
const publicKey = new PublicKey('tst24HZ6pbcnraCv4r8acexfgXvyQwMSRgZRCg9gEX1')
```
{% /dialect %}
{% /dialect-switcher %}

These are just basic examples. To learn more about Umi's keypairs check out [PublicKeys and Signers](/umi/public-keys-and-signers). There are also converters between both Umi and web3js [Web3Js Adapters](/umi/web3js-adapters)

### Keypairs

{% dialect-switcher title="Umi Keypair" %}
{% dialect title="JavaScript" id="js" %}
```js
const umi = createUmi(...)
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(secretKey))

```
{% /dialect %}
{% /dialect-switcher %}

{% seperator h="6" /%}

{% dialect-switcher title="Web3js Keypair" %}
{% dialect title="JavaScript" id="js" %}
```js
import { Keypair } from '@solana/web3js'
const publicKey = Keypair.fromSecretKey(new Uint8Array(JSON.parse(Wallet.DEV1)))
```
{% /dialect %}
{% /dialect-switcher %}

These are just basic examples. To learn more about Umi's keypairs check out [PublicKeys and Signers](/umi/public-keys-and-signers). There are also converters between both Umi and web3js keypair types [Web3Js Adapters](/umi/web3js-adapters)

### Transactions

{% dialect-switcher title="Umi Transaction" %}
{% dialect title="JavaScript" id="js" %}
```js
const blockhash = await umi.rpc.getLatestBlockhash()

const transaction = const tx = umi.transactions.create({
    version: 0,
    payer: umi.identity.publicKey,
    instructions: ix,
    blockhash: blockhash.blockhash,
  });

await umi.rpc.sendTransaction(tx)
```
{% /dialect %}
{% /dialect-switcher %}

{% seperator h="6" /%}

{% dialect-switcher title="Web3js Transaction" %}
{% dialect title="JavaScript" id="js" %}

```js
const wallet = useWallet()

const messageV0 = new TransactionMessage({
  payerKey: SIGNER_WALLET.publicKey,
  recentBlockhash: latestBlockhash.blockhash,
  instructions: txInstructions,
}).compileToV0Message()

const tx = new VersionedTransaction(messageV0)

// send via useWallet hook
await wallet.sendTransaction(tx)
//or send via connection
await connection.sendTransaction(tx)
```
{% /dialect %}
{% /dialect-switcher %}

These are just basic examples. To learn more about Umi's Transiactions check out [Transactions](/umi/transactions). There are also converters between both Umi and web3js Transaction types [Web3Js Adapters](/umi/web3js-adapters)
