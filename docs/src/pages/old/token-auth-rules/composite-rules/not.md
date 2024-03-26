---
title: Not
metaTitle: Token Auth Rules - Not
description: The Not composite rule
---

## Not
A **Not** Rule operates as a negation on the contained Rule. If the contained Rule evaluates to true then the **Not** will evaluate to false, and vice versa.

### Fields
* **rule** - The Rule to negate

```js
// This Rule Set will only evaluate to true if the Public Key does NOT sign the transaction.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    deposit: {
      type: 'Not',
      rules: [
        {
          type: 'AdditionalSigner',
          publicKey: publicKey('DhYCi6pvfhJkPRpt5RjYwsE1hZw84iu6twbRt9B6dYLV'),
        },
      ],
    },
  },
}
```