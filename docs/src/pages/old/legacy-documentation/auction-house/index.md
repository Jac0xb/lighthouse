---
title: Overview
metaTitle: Auction House - Overview
description: Gives an overview of the Auction House program
---


{% callout type="warning" %}

Please note that this program is marked as deprecated and is no longer actively maintained by the Metaplex Foundation team. New features, security fixes and backward compatibility are not guaranteed. Please use with caution.

{% /callout %}

## Introduction

Auction House is a program that allows users to exchange assets within the Solana blockchain.

There are plenty of ways to exchange assets on Solana, so why another program focused on solving this problem? Let's dive deep into that.

The ethos of this program is to allow anyone to create and configure their own marketplace and even provide their own custom logic on how assets should be exchanged. The motivation behind the Auction House protocol is to create a healthy ecosystem of marketplaces that focus on different use-cases, and more importantly, each bringing their own flavor into the way they allow users to trade assets.

The most important aspect of the Auction House program is that it provides ownership of assets to the user.

Traditionally, as soon as the user lists an asset on a marketplace, the asset is moved from the user's wallet into a wallet known as the [Escrow](https://www.investopedia.com/terms/e/escrow.asp) wallet owned by the marketplace and is kept or **escrowed** there until the asset is delisted or sold. This poses some concerns:

- The user can not list the same asset on multiple marketplaces
- The user has to rely on the marketplaces’ escrow contracts to safely hold their asset.

This is where Auction House shows its power. Its a transaction protocol that allows marketplaces to implement an **escrow-less** sales contract, thus providing ownership of assets to the users.

## Creating an Auction House

The Auction House program can be used to create a new marketplace by instantiating a new **Auction House** account. The Auction House account is a [Program Derived Address (PDA)](../../understanding-programs#program-derived-addresses-pda) which is derived from a given public key and, optionally, an SPL Token to use a currency (more on that below).

   ![Properties.PNG](https://i.imgur.com/2HPpM9g.png#radius)


The account can be configured in whichever way the user wants. We'll talk [more about these configurations in a dedicated page](settings) but here are some interesting configurable parameters:

- `requireSignOff`: this allows marketplaces to gate which assets can be listed and which bids can be placed. On every relevant instruction, the Auction House [authority](https://docs.solana.com/staking/stake-accounts#understanding-account-authorities) needs to sign the transaction.
- `canChangeSalePrice`: this parameter is only intended to be used on Auction Houses with `requireSignOff` set to `true`. This allows the Auction House to perform custom order matching to find the best price for the seller.
- `sellerFeeBasisPoints`: this represents the share the marketplace takes on all trades. For instance, if this is set to `200`, i.e. 2%, then the marketplace takes 2% of every single trade that happens on their platform.

## Listing and Bidding

Once we have an active Auction House, users can start listing assets and bidding on assets on the marketplace.

### Listing

When a user lists an asset, the Auction House does two things:

1. Auction House creates a **Sell Order**: in other words, creates a PDA known as the `SellerTradeState` which represents the listing of the asset. Trade States are special PDAs that are very cheap in comparison to other PDAs / Accounts. This is because these PDAs only store 1 byte of data, which is the [bump](https://solanacookbook.com/core-concepts/pdas.html#generating-pdas) of the PDA. All other information related to listings such as list price, amount of tokens, mint account etc, are hashed into the seeds of the PDA, instead of storing them inside the PDA itself, and therefore the PDA acts as a "proof of existence" for that listing, while being extremely cost efficient.

![](https://i.imgur.com/ki27Ds8.png#radius)

2. Auction House also assigns another PDA: `programAsSigner` PDA as the **Delegate**. Delegates are a feature of the Solana SPL-token program and are discussed in detail [here](https://spl.solana.com/token#authority-delegation). Delegation allows the Auction House to pull assets out of a token account when a sale goes through at a later point. This way, the asset need not be escrowed and can stay in the user's wallet up until the sale goes through.

![](https://i.imgur.com/aIRl7Hb.png#radius)

### Bidding

Similar to the case of listing, when a user places a bid, the Auction House creates a **Buy Order**. In other words, it creates the `BuyerTradeState` PDA representing the bid. The bid amount (native or SPL tokens) needs to be transferred manually by the marketplace to the `BuyerEscrowAccount` PDA, which holds this amount till the sale goes through.

> Example:
>
> - Alice lists an asset A for 5 SOL. In doing so, the Auction House creates the `SellerTradeState` PDA representing the bid. The Auction House also assigns the `programAsSigner` PDA as the **Delegate**, hence giving it the **Authority** to pull the asset from Alice's wallet when the sale goes through.
> - Bob places a bid of 5 SOL on asset A. In doing so, the marketplace pulls 5 SOL from Bob's wallet to the `BuyerEscrowAccount` PDA. This amount will stay here up until the sale goes through.

## Executing a Sale

Once we have a listing and at least one bid placed for a given asset, a trade can be completed by calling the `executeSale` instruction.

The `executeSale` instruction is a permission-less crank: in other words, can be executed by anyone without any fee or reward. On the execution of the `executeSale` instruction, two things happen:

- The Auction House pulls the bid amount stored in the `BuyerEscrowAccount` and transfers this amount to the lister (minus Auction House fees).
- The `programAsSigner` PDA, which the Auction House assigned as the **Delegate**, pulls the asset from the lister's wallet (more specifically, from the Token Account in the lister's wallet), and transfers the asset into the bidder's wallet, thus completing the trade.
  ![](https://i.imgur.com/gpAX63m.png#radius)

Now that we know how the `executeSale` instruction works, let's discuss the three trade scenarios in which the `executeSale` instruction is executed in different ways:

1. _"Buying" at list price_: This is the case when a user places a bid for a listed asset, at the listed amount itself. In such cases, the `bid` and the `executeSale` instructions are executed in the same transaction, and thus the bidder effectively "buys" the asset.

> Example:
>
> - Alice lists an asset A for 5 SOL. This creates a **Sell Order** for asset A.
> - Bob notices the listing and places a bid of 5 SOL for the asset A. This creates a **Buy Order** for asset A.
> - This enables the marketplace to place a bid on the asset and execute the sale in the same transaction, in practice allowing Bob to "buy" asset A.

2. _"Selling" at bid price_: This is the case when a user, interested in an unlisted asset, places a bid on it. If the asset owner now lists the asset for the bid amount, the `list` and the `executeSale` instructions are executed in the same instruction, and thus the lister effectively "sells" the asset at the requested price.

> Example:
>
> - Bob places a bid of 5 SOL for an unlisted asset A. This creates a **Buy Order** for asset A.
> - Alice notices the bid and lists the asset A for 5 SOL. This creates a **Sell Order** for asset A.
> - This enables the marketplace to list the asset and execute the sale in the same transaction, in practice allowing Alice to "sell" asset A.

3. _Lister agreeing to a bid_: This is the case when the `executeSale` instruction is executed independently, after a **Buy Order** and a **Sell Order** exist for a given asset.

> Example:
>
> - Alice lists an asset A for 5 SOL. This creates a **Sell Order** for asset A.
> - Bob places a bid of 5 SOL for asset A, unaware of Alice's listing. This creates a **Buy Order** for asset A.
> - Alice notices the matching bid and executes the sale.

## Auctioning Fungible Assets

So far, we've talked about exchanging assets using an Auction House account, but we've not dug into what type of assets can be exchanged that way. The most popular assets that can be listed in an Auction House are [Non-Fungible Tokens (NFTs)](/token-metadata/token-standard#the-non-fungible-standard).

However, these are not the only assets that can benefit from the Auction House program. In fact, an asset can be any SPL Token so long as it has a Metadata account attached to its Mint account. If you'd like to know more about SPL Token and Metadata accounts, you can [read more about this in the overview of our Token Metadata program](/token-metadata).

## Buying asset using a custom SPL Token

In the examples showcased above, we've used SOL as the exchange currency to discuss how the Auction House program works. But SOL is not the only currency that can be configured for exchanging assets. The Auction House program allows marketplaces to configure any SPL-token to act as their currency.

This can be achieved by setting the `treasuryMint` parameter in the Auction House account to the mint account of the SPL-token of your liking.

## Custom Order Matching

When we were discussing Trade States, there was a third Trade State which was shown in the Trade State diagram: the `FreeSellerTradeState`. What's the utility of this Trade State?

During the introduction to the Auction House program, we briefly discussed how Auction House can be used by marketplaces to provide their own custom logic on how assets should be exchanged. This is where the `FreeSellerTradeState` comes in.

When a buyer intentionally lists their asset for a price of 0 SOL / SPL-tokens, the `FreeSellerTradeState` is generated. The Auction House can then change the sale price to match a matching bid that is greater than 0. This allows the Auction House to do complicated order matching to find the best price for the seller and the marketplaces can write their own custom logic to do this order matching.

## Auctioneer

All of the auctions we've seen so far have one thing in common. They are, what we call, [**Double Auctions**](https://blogs.cornell.edu/info2040/2021/11/29/four-common-types-of-auctions/#:~:text=sealed%2Dbid%20auction.-,DOUBLE%20AUCTION,-Both%20buyers%20and). That is, they are un-timed auctions where buyers and sellers, bid and list until they find a common ground.
However, there are several other forms of auctions such as English auctions and Dutch auctions which have become popular in the Solana ecosystem.
The Auction House implementation is purposefully designed with instant sales in mind and does not offer other auction types out-of-the-box.

**Auctioneer** is a customized contract type, written by the user, that uses the composability pattern of Auction House to control an individual Auction House account.

To enable an Auctioneer instance on an Auction House, it must first be explicitly delegated. This Auctioneer instance will then be able to intercept most of the Auction House instructions in order to inject its own custom logic. Metaplex also provides some Auctioneer implementations like Timed Auctions. We will talk about this in greater detail in later pages of this documentation.

![](https://i.imgur.com/RyZUfR9.png#radius)

## Next steps

On this page, we have gone through the very basics of the Auction House protocol and the power it possesses. There is a lot more that the Auction House is capable of.

We'll start by listing various libraries that can be used to get started with this program:

- [Getting Started](auction-house/getting-started)

We will proceed to dive deeper into the Auction House settings and how to manage Auction House instances:

- [Auction House Settings](auction-house/settings)
- [Managing Auction Houses](auction-house/manage)

Once we understand the basics of Auction House, we can begin to learn how to trade assets on Auction House powered marketplaces:

- [Trading assets on Auction House](auction-house/trading-assets)
- [Managing Buyer Account](auction-house/buyer-escrow)

We will also explore how to track listings, bids and sales on Auction Houses and how to fetch them:

- [Printing Receipts](auction-house/receipts)
- [Finding bids, listings and purchases](auction-house/find)

## Additional Reading Material

- [Prof Lupin's Auction House guide](https://proflupin.xyz/metaplex-auction-house)
- [Jordan's twitter thread](https://twitter.com/redacted_j/status/1453926144248623104)
- [Armani's twitter thread](https://twitter.com/armaniferrante/status/1460760940454965248)
