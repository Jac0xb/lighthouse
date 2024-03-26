---
title: freeze
metaTitle: Candy Machine - Sugar - freeze
description: freeze command.
---

When the Candy Machine has the freeze guard enabled, the `freeze` command can be used to manege its different stages.

After enabling the freeze guard on the default guards or an individual group, it needs to be initialized before minting can start. To initialize the freeza guard, use the `initialize` sub-command:

```
sugar freeze initialize --period <SECONDS>
```

where `--period` determines the interval in seconds that minted assets will be frozen. After this period, holders can thaw their assets.

{% callout %}

You can only initialize the freeze once. After initialization, it is not possible to update the period.

{% /callout %}

To thaw an asset, you can use the `thaw` sub-command:

```
sugar freeze thaw <NFT MINT>
```

You can also thaw all NFTs from the same Candy Machine using the `--all` option:

```
sugar freeze thaw --all
```

Once all NFTs are thaw, the funds can be unlocked:

```
sugar freeze unlock-funds
```