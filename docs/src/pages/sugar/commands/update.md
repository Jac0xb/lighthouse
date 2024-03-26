---
title: update
metaTitle: Candy Machine - Sugar - update
description: update command.
---

The `update` command is used to modify the current configuration of a Candy Machine. Most configuration settings can be updated by this command, except:

- The number of items in the Candy Machine can only be updated when `hiddenSettings` are being used;
- Switching to use `hiddenSettings` is only possible if the number of items is equal to 0. After the switch, you will be able to update the number of items.


To update the configuration, modify your config.json (or equivalent) file and execute:

```
sugar update
```

You can also specify custom config and cache files with the `--config` and `--cache` options:

```
sugar update -c <CONFIG> --cache <CACHE>
```

{% callout %}

You need to be careful when updating a live Candy Machine, since setting a wrong value will immediately affect its functionality.

{% /callout %}