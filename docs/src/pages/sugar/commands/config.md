---
title: config
metaTitle: Candy Machine - Sugar - config
description: config command.
---

The `config` command allows you manage your Candy Machine configuration. By default, Sugar looks for a `config.json` file in the current directory to load the Candy Machine configuration – the configuration file name can be specified with a `-c` or `--config` option on every command that requires it.

You can either create this file manually, following these [instructions](/candy-machine/sugar/configuration), or use the config create command:

```
sugar config create
```

Executing the command starts an interactive process consisting in a sequence of prompts to gather information about all configuration options. At the end of it, a configuration file is saved (default to config.json) or its content is displayed on screen. To specify a custom file name, use the option `-c`:

```
sugar config create -c my-config.json
```

Once your Candy Machine is deployed, any changes to the configuration file must be set to the Candy Machine account using the `update` sub-command:

```
sugar config update
```

You can update the Candu Machine authority (the public key that controls the Candy Machine) using the `-n` option:

```
sugar config update -n <NEW PUBLIC KEY>
```

You can also change the token standard of the assets minted through the Candy Machine by using the `set` sub-command. This command supports changing the type of asset to either `NFT`s or `pNFT`s using the `-t` option. It also allows you to specify a rule set for minted pNFTs.

```
sugar config set -t "pnft" --rule-set <PUBLIC KEY>
```
