---
title: Merkle Tree Canopy
metaTitle: Bubblegum - Merkle Tree Canopy
description: Learn more about the Merkle Tree Canopy on Bubblegum
---

## Introduction

Solana's networking stack uses an MTU size of 1280 bytes which, after accounting for headers, leaves 1232 bytes for data.  The effect of this on compressed NFTs (cNFTs) is that it would currently be impossible to modify a Merkle tree of depth greater than 24, as the required proofs would take up too much transaction size.

To circumvent these proof size restrictions, spl-account-compression provides the ability to cache the upper most nodes of the Merkle tree. This is called the **Canopy**, and is stored at the end of the concurrent Merkle tree account.

By caching the upper *n* levels of a depth *d* tree, proofs can be truncated to the first *d - n* nodes. This helps reduce the size of account compression transactions, and makes it possible to modify trees up to depth 31, which can store more than 1 billion cNFTs.

To initialize a canopy on a Merkle tree account, you must initialize the account with additional bytes. The number of additional bytes needed is (2*ⁿ*⁺¹ - 1) * 32, where *n* is the number of levels of the Merkle tree you want the canopy to cache.

The canopy will be updated everytime the concurrent merkle tree is modified.  No additional work is needed.  Note however that you cannot change the canopy size after the tree is created.

## Composability vs. Cost Savings

The table below was generated with help from [compressed.app](https://compressed.app/), and shows how the total cost of minting 1,000,000 cNFTs can vary widely depending on canopy size.

### Cost for 1,000,000 cNFTs with various Canopy depth
*A Merkle tree of depth 20 can store 1,048,576 cNFTs.*
| Canopy Depth     | Proof Bytes   | Storage Cost | Mint cost (3 mint/tx w/ LUT) | Total cost |
| ---------------- | ------------- | ------------ | -----------------------------| ---------- |
| 0                | 640           | 0.3091       | 1.6667                       | 1.9758     |
| 14               | 192           | 7.6067       | 1.6667                       | 9.2734     |
| 17               | 96            | 58.6933      | 1.6667                       | 60.36      |

The reason to have a canopy depth of zero is to have the cheapest mint possible.  However, this requires sending lots of proof data with instructions such as `transfer`, `delegate`, and `burn`.  In the zero-depth canopy case, slighty more than half of the transaction size limit is consumed with proof data, which negatively affects the ability to compose Bubblegum instructions with other program instructions.

Ultimately, the decision for canopy size must consider the tradeoff between cost and composability.  This assessment should take into account factors such as the intended use of the cNFTs, the development platform's compatibility, and the ownership structure of the tree.