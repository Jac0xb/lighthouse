---
title: withdraw
metaTitle: Candy Machine - Sugar - withdraw
description: withdraw command.
---

When the mint from a Candy Machine is complete, it is possible to recover the funds used to pay rent for the data stored on-chain. You can initiate the withdrawal by running:

```
sugar withdraw --candy-machine <CANDY MACHINE ID>
```

where the `<CANDY MACHINE ID>` is the Candy Machine ID (Public Key) — the ID given by the `deploy` command. 

It is also possible to withdraw funds from all Candy Machines associated with the current keypair:

```
sugar withdraw
```

Alternatively, you can list all Candy Machines and their associated funds from the current keypair:

```
sugar withdraw --list
```

{% callout %}

You should not withdraw the rent of a live Candy Machine, as the Candy Machine will stop working when you drain its account.

{% /callout %}
