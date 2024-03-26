---
title: Timed Auction using Auctioneer
metaTitle: Auction House - Timed Auction using Auctioneer
description: Explains how to manage timed Auctions using Auctioneer.
---

The Timed Auction Auctioneer is an Auctioneer implementation that adds English-style auction rules built on top of Auction House.

# Listing Config
The listing config struct is used to store all feature-related parameters attached to each listing. All features can be configured on a per-listing basis.

# Features
* English Auction with start and end dates
* Highest bid tracking
* Reserve price (Minimum Bid)
* Minimum Bid Increment - Require new bidders to bid a certain amount more than the current bid
* Automatic Time Extension - Bids made close to the end of an auction (this period is customizable) will extend the auction end date a configurable amount
* Prevent Highest Bidder cancellation - The highest bidder won't be able to cancel their bid and are required to purchase the NFT if they win*

{% callout type="warning" %}

*Due to Auction House's escrowless nature, it is unable to prevent users from transferring listed tokens from their wallets and nullifying their bids at this time.

{% /callout %}
