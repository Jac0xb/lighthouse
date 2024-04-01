---
title: mint
metaTitle: Candy Machine - Sugar - mint
description: mint command.
---

The `mint`` command mints NFTs from a Candy Machine from the command-line.

When using the default `cache.json`, you can use:

```
sugar mint
```

Otherwise, specify your cache file with the option `--cache`:

```
sugar mint --cache <CACHE>
```

You can also specify the number of NFTs to mint using the option `-n` (e.g., 10)`:

```
sugar mint -n 10
```

{% callout %}

It is not possible to use the mint command if there are guards enabled.

{% /callout %}