---
title: Inscription Sharding
metaTitle: Inscriptions - Sharding
description: Explains the method used to prevent write-lock contention on Inscription minting.
---

## Solana Write Locks

## Sharded Counters

This rank is stored in 32 Shards to prevent write locks when creating new inscriptions. This sharding allows for up to 32 Inscriptions to be minted in the same slot, preventing resource contention and making Inscription transactions much more likely to succeed.
