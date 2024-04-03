---
title: Overview
metaTitle: Lighthouse - Overview
description:
---

**Lighthouse** is a runtime assertion Solana program designed to fail a transaction when onchain state is found to have diverged from expected state.

**Example**: when a bad actor spoofs simulation results, lighthouse will catch the discrepancy at runtime and fail the transaction.

**Example**: a token account has overspent during a transaction and guarantees that the transaction will fail.

**Example**: an oracle has provided a price feed that is incorrect, put bounds of the min/max of the price feed to ensure the transaction will fail if the price feed is out of range.

Typically a transaction builder would need their own program to make onchain state assertions—Lighthouse eliminates that need by creating a composable & generic assertion instruction framework which can be included in existing transaction building flows.

## Wallet Drain Example

Lighthouse is all about protecting transactions, take for example a malicious transaction where the simulation shows that a token account's balance is unchanged but when processed the transaction actually transfers the entire token balance to a bad actor.

{% figure src="/assets/slide_1.webp" alt="TODO" caption="The contents of a wallet draining transaction" /%}

A wallet provider injects a lighthouse assertion instruction into the transaction to check if the token account balance is equal to what was expected to happen in the simulation. If the lighthouse instruction fails, the transaction is rejected.

{% figure src="/assets/slide_2.png" alt="TODO" caption="Wallet draining transaction with injected lighthouse instruction" /%}

In the event that a malicious transaction is too large or is cosigned (and thus unalterable). The wallet provider creates a lighthouse transaction, signs, and sends the transactions in a bundle to the jito validators. Jito bundles X Lighthouse are the perfect silver bullet for mitigating fraudulent transfers in a transaction.

{% figure src="/assets/slide_3.png" alt="TODO" caption="Jito bundle of malicious transaction and lighthouse transaction combined. Jito bundles fail atomically meaning if the lighthouse transaction fails, the entire bundle fails." /%}

## Next steps

Whilst this provides a good overview of Lighthouse, there is a lot more to discover and learn about them. Head over to [Getting Started](/getting-started).
