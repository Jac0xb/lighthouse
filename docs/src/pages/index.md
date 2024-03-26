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

<!-- {% quick-links %}
{% quick-link title="Creator Studio" icon="Star" href="https://studio.metaplex.com" description="Don't want to code? Launch your next drop by pressing buttons!" /%}

{% quick-link title="Getting Started" icon="InboxArrowDown" href="/candy-machine/getting-started" description="Find the language or library of your choice and get started with Candy Machines." /%}
{% quick-link title="Recipes" icon="RectangleStack" href="/candy-machine/recipes" description="Learn various scenarios by reading concrete code examples." /%}
{% quick-link title="API reference" icon="CodeBracketSquare" href="/candy-machine/references" description="Looking for something specific? We've got you." /%}
{% /quick-links %} -->

<!-- {% callout %}
This documentation refers to the latest iteration of Candy Machine known as Candy Machine V3. If you’re looking for Candy Machine V2, [please refer to this documentation instead](https://docs.metaplex.com/deprecated/candy-machine-v2/).
{% /callout %} -->

## Next steps

Whilst this provides a good overview of Lighthouse, there is a lot more to discover and learn about them. Here’s what you can expect in the other pages of this Lighthouse documentation.

- [Getting Started](/candy-machine/getting-started). Lists the various ways lighthouse can be used.
<!-- - [Candy Machine Settings](/candy-machine/settings). Explains Candy Machine settings in great detail.
- [Managing Candy Machines](/candy-machine/manage). Explains how to manage Candy Machines.
- [Inserting Items](/candy-machine/insert-items). Explains how to load items into Candy Machines.
- [Candy Guards](/candy-machine/guards). Explains how guards work and how to enable them.
- [Guard Groups](/candy-machine/guard-groups). Explains how to configure multiple groups of guards.
- [Special Guard Instructions](/candy-machine/guard-route). Explains how to execute guard-specific instructions.
- [Minting](/candy-machine/mint). Explains how to mint from Candy Machines and how to handle pre-mint requirements.
- [How-To Guides](/candy-machine/how-to-guides). Lists practical articles relevant to Candy Machines.
- [Conceptual Guides](/candy-machine/conceptual-guides). Lists theoretical articles relevant to Candy Machines.
- [References](/candy-machine/references). Lists API References relevant to Candy Machines.
- [Updates](/candy-machine/updates). Documents the latest changes. -->
