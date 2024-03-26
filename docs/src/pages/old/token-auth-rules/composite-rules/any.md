---
title: Any
metaTitle: Token Auth Rules - Any
description: The Any composite rule
---

## Any
This Rule operates as a logical OR on all Rules contained by an **Any** Rule. Only one contained Rule must evaluate to true in order for the **Any** Rule to evaluate to true.

### Fields
* **rules** - A list of contained Rules

```js
// This Rule Set will evaluate to true if one of the Public Keys sign the transaction.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    deposit: {
      type: 'Any',
      rules: [
        {
          type: 'AdditionalSigner',
          publicKey: publicKey('DhYCi6pvfhJkPRpt5RjYwsE1hZw84iu6twbRt9B6dYLV'),
        },
        {
          type: 'AdditionalSigner',
          publicKey: publicKey('6twkdkDaF3xANuvpUQvENSLhtNmPxzYAEu8qUKcVkWwy'),
        },
      ],
    },
  },
}
```