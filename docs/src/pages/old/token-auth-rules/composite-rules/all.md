---
title: All
metaTitle: Token Auth Rules - All
description: The All composite rule
---

## All

This Rule operates as a logical AND on all Rules contained by an **All** Rule. All contained Rules must evaluate to true in order for the **All** Rule to evaluate to true.

### Fields

- **rules** - A list of contained Rules

```js
// This Rule Set will only evaluate to true if both Public Keys sign the transaction.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    deposit: {
      type: 'All',
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
