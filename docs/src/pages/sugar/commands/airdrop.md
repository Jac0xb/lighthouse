---
title: airdrop
metaTitle: Candy Machine - Sugar - airdrop
description: Command to airdrop (p)NFTs with sugar.
---

## Useage

The `airdrop` command mints NFTs from a Candy Machine to a list of wallets from the command-line.

It requires a file, by default called `airdrop_list.json` which contains the wallet public keys and the amount of NFTs each wallet should receive. In the following example `address1` would receive 2 NFTs, `address2` would receive 7. The file should have the following format:

```json
{
"address1": 2,
"address2": 7
}
```

After completing you will find a `airdrop_results.json` file with the results of your airdrop and possible issues.

{% callout %}

It is not possible to use the airdrop command if there are guards enabled.

{% /callout %}

When using the default `cache.json` and `airdrop_list.json`, you can use the following command to initate the airdrop:

```
sugar airdrop
```

Otherwise, specify your airdrop_list file with `--airdrop-list`:

```
sugar airdrop --airdrop-list <AIRDROP_LIST>
```

By default sugar will use the default cache file `cache.json`. You can also override the cache file name with `--cache`:

```
sugar mint --cache <CACHE>
```

You can also tell sugar to use a specific candy machine with `--candy-machine`: 

```
sugar mint --candy-machine <CANDY_MACHINE>
```

## Rerunning the command
In some cases mints will fail, e.g. because a blockhash was not found or similar RPC / Network related reasons. The results of your airdrop will be saved in `airdrop_results.json`. When rerunning the command the airdrop list and airdrop results will be compared.

Be careful: In some cases you will see that a transaction could not be confirmed before a timeout happened. In those cases you should confirm e.g. on an explorer if the NFT was minted.