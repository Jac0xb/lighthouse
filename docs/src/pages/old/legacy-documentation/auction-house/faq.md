---
title: FAQ
metaTitle: Auction House - FAQ
description: "FAQ for Auction House"
---

## Can I get fees when NFTs get sold-on my Auction House?
Yes, An Auction House may be configured to take `seller fee basis points`. This is part of the create and update command; see the CLI use it.

Fees are paid to Creators,Then the Auction house and the seller gets the remainder of the sale. This is easy to calculate on your UI by taking the NFT royalties, Sale price, Auction House fee and displaying to the buyer what their total gains will be.

## Does the Auction House restrict the user from selling their NFT on another Non-Auction House marketplace?
No, the Auction House cannot stop users from sending their NFT even if they have a for-sale listing. If this happens, the `execute_sale` operation will fail and the buyer can get their funds back by canceling their bid.
Marketplaces creating an Auction House experience will need to track the Buy/Sell trade state accounts and watch the TokenAccounts of sellers, so they can automatically cancel the listing and bids on NFTs that have been transferred from the original seller.

Specifically Marketplaces should currently store:

1. Trade Stade Account Keys
2. Trade State Token Size and Price parts of the seed
3. Token Account Keys that are stored in the trade state
4. Auction House Receipts (Listing Receipts, Bid Receipts, and Purchase Receipts)

Specifically Marketplaces need to track these two events on Token Accounts:

1. Ownership has changed from the original Seller of the NFT
2. Token Account Amount has changed to 0

If these events happen the Auction House Authority can call instructions to cancel the bids and listings without the seller or buyer needing to be present.

## Can people view the settings of my Auction House?
Yes anyone can and should be able to verify the settings of your Auction House especially the `Can Change Sale Price` parameter.
This can be done on the CLI with the `show` command.


## Can the Auction House change the sale price on my NFT?
Yes, but only in a certain scenario. The following conditions are required in order for an Auction House to be able to use this feature:

1. The Auction House instance must have `Can Change Sale Price` set to `true`
2. The NFT seller must list the NFT for sale at a price of 0. 

{% callout type="warning" %}
The Auction House can only sell it for 0 if you sign the transaction with your key, but currently it can sell it for an arbitrarily low price, e.g. 1 lamport. It is important to only list with Auction Houses you trust.
{% /callout %}

3. The Auction House now can use the `0` priced trade state you made in #2 to create new `sale` listings at different prices. 


## What's the difference between public and private bids?
A standard bid, also called a private bid, refers to a bid made that's specific to an auction. When the auction is complete the bid can be canceled and the funds in escrow returned to the bidder. However, Auction House also supports public bids which are specific to the token itself and not to any specific auction. This means that a bid can stay active beyond the end of an auction and be resolved if it meets the criteria for subsequent auctions of that token.

Example:
1. Alice places a public bid on Token A for 1 SOL.
2. Bob also bids on Token A for 2 SOL.
3. Bob wins the auction and becomes the new owner of Token A.
4. A week later, Bob places Token A for auction but no one new places a bid.
5. Because Alice never canceled her public bid, hers is the sold bid on the new auction of Token A, and she wins the auction.
