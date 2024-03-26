---
title: deploy
metaTitle: Candy Machine - Sugar - deploy
description: deploy command.
---

Once all assets are uploaded and the cache file is successfully created, you are ready to deploy your items to Solana:

```
sugar deploy
```

The deploy command will write the information of your cache file to the Candy Machine account on-chain. This effectively creates the Candy Machine and displays its on-chain ID (Public Key) — use this ID to query its information on-chain using an [explorer](https://explorer.solana.com). You can specify the path for the configuration file with the `-c` option (default `config.json`) and the name of the cache file with the option `--cache` (default `cache.json``) in case you are not using the default names.