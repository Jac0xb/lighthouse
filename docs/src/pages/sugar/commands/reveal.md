---
title: reveal
metaTitle: Candy Machine - Sugar - reveal
description: reveal command.
---

When using *hiddenSettings* to do a mint and reveal, the `reveal` command can be used to update all minted NFTs with the values from the cache file:

```
sugar reveal
```

It works by first retrieving all NFTs minted from the Candy Machine and then match them up to the values in the cache file by NFT number and then update the NFT data. The command checks if an NFTs URI already matches that in the cache file, and if it does, it skips updating, so the command can be rerun to only update newly minted NFTs or to retry ones that failed to update the first run.