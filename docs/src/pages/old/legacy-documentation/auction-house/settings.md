---
title: Settings
metaTitle: Auction House - Settings
description: Explains Auction House settings in great detail.
---

## Introduction

On this page, we will discuss settings that are available on an Auction House. These settings include some general settings that define how an Auction House operates, defination of some accounts (PDAs) that support the operation of the Auction House and some more specific settings that provide further configurability to the Auction House program.

## The Authority

The authority is the wallet which controls the usage of an account, and in this case, the Auction House instance. The authority address can be be mentioned when creating an Auction House. If not mentioned, the wallet which is being used to create the Auction House defaults as the authority. 

The authority can also be transferred to another wallet after the creation of the Auction House, which transfers control of the Auction House. This action should be performed carefully.

Authority wallet also plays another important role of guarding which assets could be listed and sold on the marketplace. We'll talk more about this functionality of the authority when we discuss [`requireSignOff`](#requiresignoff)

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

When using the JS SDK, the authority of a Auction House will always default to the wallet being used to create the Auction House. You may explicitly set this authority by providing a valid signer to the authority property.

```tsx
import { Keypair } from "@solana/web3.js";

const myCustomAuthority = Keypair.generate();
const auctionHouseSettings = {
  authority: myCustomAuthority,
};
```

{% /dialect %}
{% /dialect-switcher %}

## Trade Settings

These are trading-specific settings that can be set on an Auction House. These settings help in defining how a user interacts with the marketplace:

1. `treasuryMint`: This defines the mint account of the SPL-token to be used as the currency of exchange in the marketplace. Most marketplaces on Solana usually use SOL as the currency of exchange and for trading assets. Using this setting, the authority of the Auction House can set any SPL-token to be used to buy and sell assets on the given marketplace.

2. `sellerFeeBasisPoints`: This defines the secondary sale royalties that a marketplace receives on each sale of every asset on the given marketplace. `250` means `2.5%` royalty share.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

In this snippet we are creating an spl-token and setting it as the `treasuryMint` of the Auction House. We are also setting the marketplace royalties using `sellerFeeBasisPoints`.

```tsx
import { clusterApiUrl, Connection, Keypair } from "@solana/web3.js";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const myKeypair = Keypair.generate();
const connection = new Connection(
    clusterApiUrl('devnet'),
    'confirmed',
);
const myCustomToken = splToken.Token.createMint(connection, myKeypair, myKeypair.publicKey, null, 9, splToken.TOKEN_PROGRAM_ID)
const auctionHouseSettings = {
    treasuryMint: myCustomToken,
    sellerFeeBasisPoints: 150
};
```

{% /dialect %}
{% /dialect-switcher %}


## Helper Accounts

There are several accounts that are necessary for the Auction House to function properly. Once set by the Auction House, the authority can reset and configure these accounts as per their liking.

There are some accounts that are created and controlled by the Auction House program. These accounts are Program Derived Addresses (PDAs) which you can read more about [here](https://solanacookbook.com/core-concepts/pdas.html). These are the two settings that can be used to set these accounts:

1. `auctionHouseFeeAccount`: The public key of the fee account which stores funds for paying for Auction House related transactions on behalf of the users. 

2. `auctionHouseTreasury`: The public key of the treasury account which stores the funds received on every sale, as marketplace royalty.

There are other accounts that are not created by the Auction House program, but are essential for withdrawing different types of funds from the Auction House, back to the authority:

1. `feeWithdrawalDestination`: The public key of the account to which the funds can be withdrawn from the fee account. 

2. `treasuryWithdrawalDestination`: The public key of the account to which the funds can be withdrawn from the treasury account.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

The following code snippet builds four different keypairs, corresponding to the four accounts discussed above and sets them.

```tsx
import { Keypair } from "@solana/web3.js";

const feeAccount = Keypair.generate();
const treasuryAccount = Keypair.generate();
const feeWithdrawalDestination = Keypair.generate();
const treasuryWithdrawalDestination = Keypair.generate();
const auctionHouseSettings = {
    auctionHouseFeeAccount: feeAccount,
    auctionHouseTreasury: treasuryAccount,
    feeWithdrawalDestination: feeWithdrawalDestination,
    treasuryWithdrawalDestination: treasuryWithdrawalDestination,
};
```

{% /dialect %}
{% /dialect-switcher %}


## Require Sign Off
This setting allows marketplaces to gate asset listing and sales. As discussed in the authority section, the Auction House authority plays a role in the gating of assets. This censorship or centralised control can only take place when `requireSignOff = true`.

When this happens, every transaction on the marketplace: listing, bidding and execution of sales needs to be signed by the Auction House authority. Fully decentralised marketplaces can choose to keep the `requireSignOff` setting as `false` to avoid censorship or centralised control of actions on that marketplace. 

Setting `requireSignOff = true` has other powers as well: it allows marketplaces to implement their own custom order matching algorithms. We will talk more about this in the next section.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

The following code snippet sets `requireSignOff` to `true`.

```tsx
const auctionHouseSettings = {
    requireSignOff: true
};
```

{% /dialect %}
{% /dialect-switcher %}

## Can Change Sale Price

`canChangeSalePrice` allows marketplaces to change the sale price of an asset, when a user intentionally lists an asset for free, or for 0 SOL (or any other SPL-token). By listing the asset for 0 SOL, the user allows marketplaces to apply custom matching algorithms in order to find the best price match for the "freely" listed asset.


An important point to note here is that `canChangeSalePrice` can be set to `true` only if `requireSignOff` is also set to `true`. This is because custom matching is not possible in the case of permissionless listing and bidding. The Auction House should be able to "sign off" on a matching bid and execute the sale of the asset.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

The following code snippet sets `canChangeSalePrice` to `true`, while also ensuring that `requireSignOff` is also `true`

```tsx
const auctionHouseSettings = {
    requireSignOff: true,
    canChangeSalePrice: true
};
```

{% /dialect %}
{% /dialect-switcher %}

## Auctioneer Settings

The `Auctioneer` account is a PDA which uses the composability pattern of the Auction House program to control an Auction House Instance.

The Auctioneer has the ability to be given the control, or Delegation over an Auction House instance using the `DelegateAuctioneer` instruction which we will discuss in the Auctioneer guide (*comming soon*).

There are three setting pertaining to the Auctioneer which can be configured in the Auction House:

1. `hasAuctioneer`: True if an `Auctioneer` instance exists for the given Auction House instance.
2. `auctioneerAuthority`: The Auctioneer authority key. It is required when the Auction House is going to have Auctioneer enabled.
3. `auctioneerScopes`: The list of scopes available to the user in the Auctioneer, for example: Bid, List, Execute Sale. It only takes place when the Auction House has Auctioneer enabled.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

The following code snippet sets `hasAuctioneer` to `true`. It also points the `auctioneerAuthority` to a generated public key and sets `auctioneerScopes` to allow the Auctioneer to buy, sell and excecute the sale on behalf of the Auction House.

```tsx
import { Keypair } from "@solana/web3.js";
import { AuthorityScope } from '@metaplex-foundation/mpl-auction-house';

const newAuthority = Keypair.generate();
const auctionHouseSettings = {
    hasAuctioneer: true,
    auctioneerAuthority: newAuthority,
    auctioneerScopes: [
        AuthorityScope.Buy,
        AuthorityScope.Sell,
        AuthorityScope.ExecuteSale,
    ]
};
```

{% /dialect %}
{% /dialect-switcher %}

## Conclusion
Now that we know about Auction House settings, on the [next page](/programs/auction-house/manage), we’ll see how we can use them to create and update our own Auction House.
