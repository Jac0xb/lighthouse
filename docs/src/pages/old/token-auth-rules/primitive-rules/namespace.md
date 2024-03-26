---
title: Namespace
metaTitle: Token Auth Rules - Namespace
description: The Namespace primitive rule
---

## Namespace
The **Namespace** rule is an advanced rule used to reduce the size of a Rule Set account and compute units used during deserialization. It can also be used for common rules across multiple [Scenarios](/token-auth-rules/#scenario). The **Namespace** rule is used for a **Operation**:**Scenario** pair and will indicate that evaluation should use the rule under the **Operation**. For example, if a token has `Transfer:Owner`, `Transfer:Delegate`, and `Transfer:Authority` scenarios, but only `Transfer:Delegate` needs a special rule, the **Namespace** rule can be used to indicate that a common rule under `Transfer` should be used for both `Transfer:Owner` and `Transfer:Authority`.

```js
// This Rule Set will evaluate the Pass rule under 'Transfer' and be true for both 'Transfer:Owner' and 'Transfer:Authority' but it will only evaluate to true if the additional signer is present for a 'Delegate' transfer.
const revision: RuleSetRevisionV2 = JSON.parse({
  'libVersion': 2,
  'name': 'My Rule Set',
  owner,
  'operations': {
    'Transfer': {
      'type': 'Pass',
    },
    'Transfer:Owner': {
      'type': 'Namespace',
    },
    'Transfer:Authority': {
      'type': 'Namespace',
    },
    'Transfer:Delegate': {
      'type': 'AdditionalSigner',
      'publicKey': publicKey('DhYCi6pvfhJkPRpt5RjYwsE1hZw84iu6twbRt9B6dYLV'),
    },
  },
});
```