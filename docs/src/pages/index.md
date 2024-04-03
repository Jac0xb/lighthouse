---
title: Overview
metaTitle: Lighthouse - Overview
description:
---

**Lighthouse** is a runtime state assertion Solana program designed to add guardrails on transactions that will fail the transaction when undesired state changes are found during assertion checks such as when a bad actor spoofs simulation results, there is overspending during the transaction, or an oracle account is in an undesired state. Typically a transaction builder would need their own program to make assertions on these changes—Lighthouse eliminates that need by making it simple to append assertion instructions to existing transaction building flows.
Lighthouse is an open source, public utility Solana program with an emphasis on security (multisig, verifiable build, non-upgradable releases, etc coming soon), composability (program-agnostic use cases), and community (open source, assist in integration with open source projects, incentivize contributions).

Wallet Drain Example
Lighthouse is all about protecting transactions, take for example a malicious transaction that simulates as having no changes to the token account but actually steals every token from the signer's wallet.

{% figure src="/assets/slide_1.webp" alt="TODO" caption="The contents of a wallet draining transaction" /%}

A wallet provider injects a lighthouse assertion instruction into the transaction to check if the token account balance is equal to what was expected to happen in the simulation. If the lighthouse instruction fails, the transaction is rejected.

{% figure src="/assets/slide_2.png" alt="TODO" caption="Wallet draining transaction with injected lighthouse instruction" /%}

In the event that a malicious transaction is too large or is cosigned (and thus unalterable). The wallet provider creates a lighthouse transaction, signs, and sends the transactions in a bundle to the jito validators. Jito bundles X Lighthouse are the perfect silver bullet for mitigating fraudulent transfers in a transaction.

{% figure src="/assets/slide_3.png" alt="TODO" caption="Jito bundle of malicious transaction and lighthouse transaction combined. Jito bundles fail atomically meaning if the lighthouse transaction fails, the entire bundle fails." /%}

## Next steps

Whilst this provides a good overview of Lighthouse, there is a lot more to discover and learn about them. Head over to [Getting Started](/getting-started).
