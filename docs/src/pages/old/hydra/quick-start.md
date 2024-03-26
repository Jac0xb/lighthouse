---
title: Quick Start
metaTitle: Hydra - Quick Start
description: Provides a high-level overview of Hydra wallets.
---

To get started with Hydra, you'll need the package for your programming environment.

If you are using Rust grab the crate here:

[https://crates.io/crates/hydra_wallet](https://crates.io/crates/hydra_wallet)

If you are using Javascript, grab the package here:

[https://www.npmjs.com/package/@glasseaters/hydra-sdk](https://www.npmjs.com/package/@glasseaters/hydra-sdk)

## Quick Start - JS

Install the package from npm:

```bash
yarn add @glasseaters/hydra-sdk
```

This is how you'd set up a Hydra with the Wallet [membership model](./intro#adding-members).

```ts
import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { airdrop } from "@metaplex-foundation/amman";
import {
  Fanout,
  FanoutClient,
  FanoutMembershipMintVoucher,
  FanoutMembershipVoucher,
  FanoutMint,
  MembershipModel
} from "@glasseaters/hydra-sdk";


const connection = new Connection("devnet", "confirmed");
const authorityWallet = Keypair.generate();

await airdrop(connection, authorityWallet.publicKey, LAMPORTS_PER_SOL * 2);

const fanoutSdk = new FanoutClient(
  connection,
  new NodeWallet(new Account(authorityWallet.secretKey))
);

// Initialize the Hydra Wallet
const { fanout, nativeAccount } = await fanoutSdk.initializeFanout({
  totalShares: 100,
  name: `Your Globally Unique Wallet Name`,
  membershipModel: MembershipModel.Wallet,
});

// fanout is your fanout config address
// nativeAccount is your account address

// Retrieve the On-chain Hydra Wallet
const fanoutAccount = await fanoutSdk.fetch<Fanout>(
  fanout,
  Fanout
);

console.log(fanoutAccount); // Shows you all the parameters in your Hydra

// This is your Hydra Wallet Address
let HydraAccountKey = fanoutAccount.accountKey // this is the same thing as nativeAccount above


// If you only know the Hydra name, this is how you can retrieve the account key
let name = `Your Globally Unique Wallet Name`
let [key, bump] = await fanoutSdk.fanoutKey(name)
let [holdingAccount, bump] = await fanoutSdk.nativeAccount(key)


// Add members

const member1 = new Keypair();
const { membershipAccount1 } = await fanoutSdk.addMemberWallet({
  fanout: init.fanout,
  fanoutNativeAccount: init.nativeAccount,
  membershipKey: member1.publicKey,
  shares: 10
});

//Repeat for all members until sum(shares) == totalShares from initialization
...

// Send some Sol to the Hydra Wallet so you can distribute
await airdrop(connection, HydraAccountKey, 2);

// Generate the distribution instructions
let distMember1 = await fanoutSdk.distributeWalletMemberInstructions(
  {
    distributeForMint: false,
    member: member1.wallet.publicKey,
    fanout: fanout,
    payer: authorityWallet.publicKey, // This can be changed to whoever sends the tx
  },
);

// Send the distribution instructions
const tx = await fanoutSdk.sendInstructions(
  [...distMember1.instructions],
  [authorityWallet],
  authorityWallet.publicKey
);
if (!!tx.RpcResponseAndContext.value.err) {
  const txdetails = await connection.getConfirmedTransaction(tx.TransactionSignature);
  console.log(txdetails, tx.RpcResponseAndContext.value.err);
}

// Member1 Should have 0.2 more sol in their wallet

```
