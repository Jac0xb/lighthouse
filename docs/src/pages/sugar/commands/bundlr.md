---
title: bundlr
metaTitle: Candy Machine - Sugar - bundlr
description: bundlr command.
---

When you use Bundlr as your upload method, Sugar automatically funds your account on the Bundlr Network to cover the storage costs. Once the upload is completed, there could be leftover funds in the account.

You can verify your balance on the Bundlr Network with the following command:

```
sugar bundlr balance
```

This will retrieve the balance for the current keypair. You can specify an alternative keypair using the option `--keypair`. The remaining balance (if there is any) can be withdrawn:

```
sugar bundlr withdraw
```

At the end of the withdrawal, the funds available on the Bundlr Network will be transferred to your Solana address.