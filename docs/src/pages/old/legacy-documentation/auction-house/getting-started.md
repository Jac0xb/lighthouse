---
title: Getting Started
metaTitle: Auction House - Getting Started
description: Lists the various libraries and SDKs you can use to manage Auction Houses.
---

The Auction House is a Solana program running on Mainnet Beta and Devnet. While you may interact with it like any other Solana program by sending transactions to a Solana node, Metaplex has built some tools to make working with it much easier. We have a **CLI** tool that will let you manage your auction house and a **JS SDK** to help you kick-start a user interface.

## SDKs

### JavaScript SDK
The **JS SDK** provides an easy-to-use API to web developers to create and configure one's own Auction House. The SDK also allows developers to perform complicated procedures like bidding, listing, withdrawing funds from the Auction House treasury and fee accounts, and much more. 

The main module that interacts with the Auction House program is the [Auction House module](https://github.com/metaplex-foundation/js/tree/main/packages/js/src/plugins/auctionHouseModule). This module contains several methods that make the process of making marketplaces painless. You may access this client via the `auctionHouse()` method of your `Metaplex` instance.
```ts
const auctionHouseClient = metaplex.auctionHouse();
```

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Here are some of the useful methods provided by the SDK:

```ts
// Creating and updating the Auction House
metaplex.auctionHouse().create();
metaplex.auctionHouse().update();

// Trading on Auction House
metaplex.auctionHouse().bid();
metaplex.auctionHouse().list();
metaplex.auctionHouse().executeSale();

// Cancelling a bid or listing
metaplex.auctionHouse().cancelBid();
metaplex.auctionHouse().cancelListing();

// Finding bids, listings and purchases
metaplex.auctionHouse().findBidBy();
metaplex.auctionHouse().findBidByTradeState();
metaplex.auctionHouse().findListingsBy();
metaplex.auctionHouse().findListingByTradeState();
metaplex.auctionHouse().findPurchasesBy();
```

{% /dialect %}
{% /dialect-switcher %}


There are also other methods that already exist in the Auction House module, and more methods will be added in the future. The *README* of the Auction House module will be updated with a detailed documentation of all these methods very soon.

**Helpful links:**
* [Github repository](https://github.com/metaplex-foundation/js/tree/main/packages/js/src/plugins/auctionHouseModule)
* [NPM package](https://www.npmjs.com/package/@metaplex-foundation/js)

## Program Libraries
Program Libraries are auto-generated from the Program’s IDL using Solita. Whilst they require you to understand Solana programs and wire your own instructions, they have the advantage of immediately using the latest features when SDKs might take a bit longer to implement them.

### JavaScript Program Libraries
This is a lower level, auto-generated JavaScript library, which gets generated whenever the Auction House program (written in Rust) gets updated. 

Therefore, this library is intended for advanced developers who wish to interact with the program by preparing instructions and sending transactions directly.

**Helpful links:**
* [Github repository](https://github.com/metaplex-foundation/metaplex-program-library/tree/master/auction-house/js)
* [NPM package](https://www.npmjs.com/package/@metaplex-foundation/mpl-auction-house)

## Rust Crates
If you are developing in Rust, you can also use the Rust crates to interact with Metaplex’s programs. Since our programs are written in Rust, theses crates should contain everything you need to parse accounts and build instructions.

This can be helpful when developing Rust clients but also when making CPI calls to Metaplex programs within your own program.

**Helpful links:**
* [Github repository](https://github.com/metaplex-foundation/metaplex-program-library/tree/master/auction-house/program)
* [Crate page](https://crates.io/crates/mpl-auction-house)
* [API references](https://docs.rs/mpl-auction-house/latest/mpl_auction_house/)
