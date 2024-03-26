---
title: Find bids, listings and sales
metaTitle: Auction House - Find
description: "Explains how to find bids, listings and sales."
---
## Introduction

In the previous page we saw how to make receipts for bids, listings and sales. These receipts make it easier for the marketplace operators to keep track of these actions. But how does one fetch these bids, listings and sales?

There are three types of functions provided for fetching bids, listings and sales:

1. **Find all in an auction house**: using this type of function, all bids / listings / sales can be found for a given Auction House.

2. **Find by receipt**: using this type of function, a single bid / listing / sale can be found, given the address of the corresponding receipt account.

3. **Find by trade state**: We talked about [Trade States in the overview page](/programs/auction-house/overview). Trade state PDA accounts encoding the bid / listing / sale orders can also be used to find the corresponding action.

### Find All in an Auction House

There are multiple criteria to find all bids, listings and sales (or *purchases*) in an Auction House.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}

Below is the snippet for finding bids by multiple criteria. You can use any combination of keys.
     
```tsx
// Find all bids in an Auction House.
const bids = await metaplex
  .auctionHouse()
  .findBids({ auctionHouse });

// Find bids by buyer and mint.
const bids = await metaplex
  .auctionHouse()
  .findBids({ auctionHouse, buyer, mint });

// Find bids by metadata.
const bids = await metaplex
  .auctionHouse()
  .findBids({ auctionHouse, metadata });
```

Here's a snippet for finding listings by multiple criteria. Again, you can use any combination of keys.

```tsx
// Find all listings in an Auction House.
const listings = await metaplex
  .auctionHouse()
  .findListings({ auctionHouse });

// Find listings by seller and mint.
const listings = await metaplex
  .auctionHouse()
  .findListings({ auctionHouse, seller, mint });
```

Below is a snippet for finding purchases by multiple criteria. It supports only 3 criteria at the same time including the required `auctionHouse` attribute.

```ts
// Find all purchases in an Auction House.
const purchases = await metaplex
  .auctionHouse()
  .findPurchases({ auctionHouse });

// Find purchases by buyer and mint.
const purchases = await metaplex
  .auctionHouse()
  .findPurchases({ auctionHouse, buyer, mint });

// Find purchases by metadata.
const purchases = await metaplex
  .auctionHouse()
  .findPurchases({ auctionHouse, metadata });

// Find purchases by seller and buyer.
const purchases = await metaplex
  .auctionHouse()
  .findPurchases({ auctionHouse, seller, buyer });
```

{% /dialect %}
{% /dialect-switcher %}

### Find by Receipt

Below is the snippet for finding bids, listings and sales by corresponding receipt acccount address.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}
     
```tsx
// Find a bid by receipt
const nft = await metaplex
  .auctionHouse()
  .findBidByReceipt({ receiptAddress, auctionHouse };

// Find a listing by receipt
const nft = await metaplex
  .auctionHouse()
  .findListingByReceipt({ receiptAddress, auctionHouse };

// Find a sale / purchase by receipt
const nft = await metaplex
  .auctionHouse()
  .findPurchaseByReceipt({ receiptAddress, auctionHouse };
```

{% /dialect %}
{% /dialect-switcher %}

### Find by Trade State
Below is the snippet for finding bids, listings and sales by corresponding trade state PDA accounts.

{% dialect-switcher title="JS SDK" %}
{% dialect title="JavaScript" id="js" %}
     
```tsx
// Find a bid by trade state
const nft = await metaplex
  .auctionHouse()
  .findBidByTradeState({ tradeStateAddress, auctionHouse };

// Find a listing by trade state
const nft = await metaplex
  .auctionHouse()
  .findListingByTradeState({ tradeStateAddress, auctionHouse };

// Find a sale / purchase by trade state
const nft = await metaplex
  .auctionHouse()
  .findPurchaseByTradeState({ sellerTradeState, buyerTradeState, auctionHouse };
```

{% /dialect %}
{% /dialect-switcher %}

## Conclusion

We have finally covered all corners for managing trading on a marketplace. Everything covered till now was explained using code snippets using the JS SDK. 

Next up we'll see how one can create and manage Auction Houses and the trading on them [using CLI](/programs/auction-house/how-to-guides/manage-auction-house-using-cli).
