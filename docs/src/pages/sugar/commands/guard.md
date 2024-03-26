---
title: guard
metaTitle: Candy Machine - Sugar - guard
description: guard command.
---

The `guard` command is used to manage the [guards](/candy-machine/guards) configuration of the Candy Machine.

Once you completed the guards configuration in your Sugar config file, you can add a Candy Guard using:

```
sugar guard add
```

At this point, the `mint` command will stop working since the `mint authority` is now the Candy Guard.

To update the Candy Guard configuration, you first need to make the required modification in the Sugar config file and the run the command:

```
sugar guard update
```

To print the on-chain configuration of the Candy Machine guards, use the command:

```
sugar guard show
```

To remove the guards from a Candy Machine, use the command:

```
sugar guard remove
```

After removing the guards, you can use the `mint` command to mint from the Candy Machine.

The `remove` command does not close the Candy Guard account. To close the account and retrieve the rent fee, use the command:

```
sugar guard withdraw
```
