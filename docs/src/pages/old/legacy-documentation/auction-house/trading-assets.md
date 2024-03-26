---
title: Trading Assets
metaTitle: Auction House - Trading Assets
description: Explains how to manage the trading of assets on Auction House.
---
## Introduction
In the previous pages, we've talked about Auction Houses and how to create & manage them. Once an Auction House is created, assets can be traded on it. A simple trade on a marketplace usually comprises of three actions:

1. The seller lists an asset
2. The buyer makes a bid on the asset
3. Once a matching bid is found for a listing, the sale is executed

On this page, we will talk about these three actions and see code examples to easily execute these actions. We will also see trade scenarios that are different from the above simple trade scenario, and go through a code example to execute each scenario. Finally, we'll also explore how listings and bids can be cancelled once they are created.

Let us start with listing an asset on an Auction House.

## Listing assets

We went through the process of listing an asset in the [Overview page](../auction-house). This action is also referred to as creating a **Sell Order**. When a sell order is created using Auction House, the asset being listed remains in the wallet of the seller. This is a very important feature of Auction House as it allows users to list assets in an escrow-less fashion and thus users still maintain custody of assets while the assets are listed.

The asset seller can create two types of listings depending on the price at which they list the asset:

1. **Listing at price greater than 0**: when a user lists an asset at a price which is greater than 0 SOL (or any other SPL-token). In this case, the seller's wallet needs to be the signer and thus this wallet should be 

2. **Listing at price of 0**: when a user lists an asset for 0 SOL (or any other SPL-token). In this case, the authority can sign on behalf of the seller if `canChangeSalePrice` option is set to `true` which was discussed in the [Auction House settings page](settings). When this happens, the Auction House finds a non-0 matching bid on behalf of the seller. The asset can only be listed and sold for a price of 0 if the seller acts as the signer. There must be one and only one signer; authority or seller must sign.

Depending on the type of token being listed, the number of tokens to be listed can also be specified when creating a sell order:

1. In case of **Non-Fungible Tokens (NFTs)**: due to the non-fungibility and uniqueness of every token, only 1 token can be listed.

2. In case of **Fungible Assets**: the seller can list more than 1 tokens per listing. For example: If Alice owns 5 DUST tokens, they can list 1 or more (but less than or equal to 5) of these DUST tokens in the same sell order.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Let us look at an example for making a listing or sell order on Auction House.

In the following code snippet we are making a sell order for 3 DUST tokens (fungible tokens) for a total price of 5 SOL. Important to note here is that if we were creating a sell order for an NFT, we do not have to specify the number of tokens to be listed as it will default to 1 token. Specifying any other amount will result in an error.
    
    
```tsx
await metaplex
    .auctionHouse()
    .createListing({
        auctionHouse,                              // A model of the Auction House related to this listing
        seller: Keypair.generate(),                // Creator of a listing
        authority: Keypair.generate(),             // The Auction House authority
        mintAccount: new PublicKey("DUST...23df"), // The mint account to create a listing for, used to find the metadata
        tokenAccount: new PublicKey("soC...87g4"), // The token account address that's associated to the asset a listing created is for 
        price: 5,                                  // The listing price
        tokens: 3                                  // The number of tokens to list, for an NFT listing it must be 1 token
    });
```

{% /dialect %}
{% /dialect-switcher %}

## Bidding on assets

A user looking to buy an asset can make bids, or **Buy Orders** for that asset. 

There can be two types of buy orders depending on that whether the asset is listed or not:

1. **Private bids**: This is the most common type of bid. A user, interested in a listed asset on an Auction House, creates a private bid on the given asset. This bid is tied to the specific auction and not the asset itself. This means that when the auction is closed (either the bid is rejected and the listing is cancelled, or the bid is accepted and the asset is sold), the bid is also closed.

2. **Public bids**: A user can post a public bid on a non-listed NFT by skipping seller and tokenAccount properties. Public bids are specific to the token itself and not to any specific auction. This means that a bid can stay active beyond the end of an auction and be resolved if it meets the criteria for subsequent auctions of that token.

Like in the case of sell orders, buy orders can also specifiy the number of tokens to be bid upon depending on the type of asset listed:

1. **Partial Buy Order**: We discussed the case of listing more than 1 token when listing a fungible asset. When such a sell order exists, a user can make a bid to buy only a portion of the listed tokens, or make a partial buy order. For example: if Alice listed `3 DUST` tokens for `5 SOL`, Alice can make a bid to buy `2 DUST` tokens for `2 SOL`. In other words, a user can create a buy order of said assets that is less than the `token_size` of the sell order.

2. **Complete Buy Order**: This is the case where the buyer creates a bid to buy all the tokens listed in the sell order. In case of non-fungible assets (NFTs) where only 1 token can be listed per sell order, a complete buy order is created. Complete buy orders can also be created in case of fungible tokens.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Let us look at an example for making a bid or buy order on Auction House.

In the following code snippet we are making a buy order for 3 DUST tokens (fungible tokens) for a total price of 5 SOL. Important to note here is that if we were creating a sell order for an NFT, we do not have to specify the number of tokens to be listed as it will default to 1 token. Specifying any other amount will result in an error.
    
This is an example of a private bid as we are specifying the seller account and the token account. If either one of them is not specified while creating the bid, the bid will be public.
     
```tsx
await metaplex
    .auctionHouse()
    .createBid({
        auctionHouse,                              // A model of the Auction House related to this listing
        buyer: Keypair.generate(),                 // Creator of a bid
        seller: Keypair,generate(),                // The account address that holds the asset a bid created is for, if this or tokenAccount isn't provided, then the bid will be public.
        authority: Keypair.generate(),             // The Auction House authority
        mintAccount: new PublicKey("DUST...23df"), // The mint account to create a bid for
        tokenAccount: new PublicKey("soC...87g4"), // The token account address that's associated to the asset a bid created is for, if this or seller isn't provided, then the bid will be public.
        price: 5,                                  // The buyer's price
        tokens: 3                                  // The number of tokens to bid on, for an NFT bid it must be 1 token
    });
```

{% /dialect %}
{% /dialect-switcher %}

## Executing sale of assets

Now that we know how to create a listing (sell order) and a bid (buy order), we can learn how to execute sales of assets. When the sale of an asset is executed:

1. The Auction House transfers the bid amount from the buyer escrow account to the seller's wallet. We will talk more about the buyer escrow account and how can the marketplace authority manage funds in that account.

2. The Auction House transfers the asset from the seller's wallet to the buyer's wallet.

Now that we know what the excution of a sale means, lets explore different trade scenarios in which assets can be sold using Auction House. We have already discussed them in great detail in the [overview page] but here's a brief explanation in addition to a code snippet for each scenario:

1. **Direct Buy**, or *"Buying" at list price*: This is the case when the execution of the sale happens when a user bids on a listed asset. In other words, a direct buy operation creates a bid on a given asset and then executes a sale on the created bid and listing. 
    
    In most cases, this scenario will occur when the buyer makes a bid at the listed price of the asset. But there can be cases where marketplaces have custom order matching algorithms that work on thresholds. For example: a marketplace may have a rule to execute the sale of a given asset as soon as there is a bid which is within a range of +-20% from the listed price.
    
{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Here's an example of direct buying an asset by a user who is interested in the listed asset.
     
```tsx
const listing = await metaplex
    .auctionHouse()
    .findListingByReceipt({...}) // we will see how to fetch listings in the coming pages
    
const directBuyResponse = await metaplex
    .auctionHouse()
    .buy({
        auctionHouse,                   // The Auction House in which to create a Bid and execute a Sale
        buyer: Keypair.generate(),      // Creator of a bid, should not be the same as seller who creates a Listing
        authority: Keypair.generate(),  // The Auction House authority, if this is the Signer the
                                        // transaction fee will be paid from the Auction House Fee Account
        listing: listing,               // The Listing that is used in the sale, we only need a
                                        // subset of the `Listing` model but we need enough information
                                        // regarding its settings to know how to execute the sale.
        price: 5,                       // The buyer's price
    });
```

{% /dialect %}
{% /dialect-switcher %}

2. **Direct Sell**, or *"Selling" at bid price*: Counterpart to the case of direct buy, this is the case when a user, interested in an unlisted asset, places a bid on it. If the asset owner now lists the asset for the bid amount, the execution of the sale can take place, thus direct selling the asset.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Here's an example of direct selling an asset by a user who is interested in a bid on the asset.
     
```tsx
const bid = await metaplex
    .auctionHouse()
    .findBidByReceipt({...}) // we will see how to fetch bids in the coming pages
    
const directSellResponse = await metaplex
    .auctionHouse()
    .sell({
        auctionHouse,                              // The Auction House in which to create a listing and execute a Sale
        seller: Keypair.generate(),                // Creator of a listing, there must be one and only one signer; Authority or Seller must sign.
        authority: Keypair.generate(),             // The Auction House authority, if this is the Signer the
                                                   // transaction fee will be paid from the Auction House Fee Account
        bid: bid,                                  // The Public Bid that is used in the sale, we only need a
                                                   // subset of the `Bid` model but we need enough information
                                                   // regarding its settings to know how to execute the sale.
        sellerToken: new PublicKey("DUST...23df")  // The Token Account of an asset to sell, public Bid doesn't
                                                   // contain a token, so it must be provided externally via this parameter
    });
```

{% /dialect %}
{% /dialect-switcher %}

3. **Independant Sale Execution**, or *Lister agreeing to a bid*: This is the case when the execution of the sale takes place independantly, after a **Buy Order** (bid) and a **Sell Order** (listing) exist for a given asset.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Here's an example of an independant sale execution.
     
```tsx
const listing = await metaplex
    .auctionHouse()
    .findListingByReceipt({...}) // we will see how to fetch listings in the coming pages
    
const bid = await metaplex
    .auctionHouse()
    .findBidByReceipt({...})     // we will see how to fetch bids in the coming pages
    
const executeSaleResponse = await metaplex
    .auctionHouse()
    .executeSale({
        auctionHouse,                   // The Auction House in which to create a Bid and execute a Sale
        authority: Keypair.generate(),  // The Auction House authority, if this is the Signer the
                                        // transaction fee will be paid from the Auction House Fee Account
        listing: listing,               // The Listing that is used in the sale, we only need a
                                        // subset of the `Listing` model but we need enough information
                                        // regarding its settings to know how to execute the sale.
        bid: bid,                       // The Public Bid that is used in the sale, we only need a
                                        // subset of the `Bid` model but we need enough information
                                        // regarding its settings to know how to execute the sale.
    });
```

{% /dialect %}
{% /dialect-switcher %}

## Cancel Listings and Bids

Till now we have seen how to create bids and listings, and also execute the sales of assets in an Auction House. Once listings and bids are created in an Auction House, they can be cancelled via the authority.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Here's an example of cancelling a bid and a listing using the JS SDK.
     
```tsx
const listing = await metaplex
    .auctionHouse()
    .findListingByReceipt({...}) // we will see how to fetch listings in the coming pages
    
const bid = await metaplex
    .auctionHouse()
    .findBidByReceipt({...})     // we will see how to fetch bids in the coming pages
    
// Cancel a bid
const cancelBidResponse = await metaplex               
    .auctionHouse()
    .cancelBid({
        auctionHouse,            // The Auction House in which to cancel Bid
        bid: bid,                // The Bid to cancel
    });

// Cancel a listing
const cancelListingResponse = await metaplex
    .auctionHouse()
    .cancelListing({
        auctionHouse,            // The Auction House in which to cancel listing
        listing: listing,        // The listing to cancel
    });
```

{% /dialect %}
{% /dialect-switcher %}

## Conclusion

In this page we covered all the components to manage trading of assets on a marketplace. 

One key point which we haven't discussed is the buyer escrow account, which is needed to escrow, or temporarily hold buyer's funds when the buyer makes a bid on an asset. How are these funds managed in this account and who is responsible for keeping track of these funds? Let's find out in the [next page](buyer-escrow).
