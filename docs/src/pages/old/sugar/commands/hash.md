---
title: hash
metaTitle: Candy Machine - Sugar - hash
description: hash command.
---

When using *hiddenSettings*, you should specify a hash value in your config file so that the assets can be verified when the mint is complete and a reveal is performed. The hash value is automatically updated by the deploy command when *hiddenSettings* are enabled, but there could be scenarios where you are modifying the cache file manually.

The `hash` command computes a hash of the cache file and updates the hash value in the config file.

```
sugar hash
```

It also allows comparing a published hash value with the value from a cache file with the `--compare` option. The cache file can be specified manually with `--cache`, or it will default to the `cache.json` file in the current directory.

Running the `--compare` against the default `cache.json`:

```
sugar hash --compare 44oZ3goi9ivakeUnbjWbWJpvdgcWCrsi
```

Running the `--compare` against a specific cache file:

```
sugar hash --compare 44oZ3goi9ivakeUnbjWbWJpvdgcWCrsi --cache my_custom_cache.json
```

{% callout %}

After updating the hash value, you will need to update your Candy Machine configuration so that the new value is on-chain using the `update` command.

{% /callout %}