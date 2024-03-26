---
title: verify
metaTitle: Candy Machine - Sugar - verify
description: verify command.
---

The `verify` command checks that all items in your cache file have been successfully written on-chain:

```
sugar verify
```

To specify a different cache file other than the default `cache.json`, use the `--cache` option:

```
sugar verify --cache <CACHE>
```

If you deploy has been successfully, the verification return no errors.
