---
title: Additional Signer
metaTitle: Token Auth Rules - Additional Signer
description: The Additional Signer primitive rule
---

## Additional Signer

An additional account must sign this transaction.

### Fields

- **address** - The address that must sign the Transaction

```js
// This Rule Set will only evaluate to true if the Public Key signs the transaction.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    deposit: {
      type: 'AdditionalSigner',
      publicKey: publicKey('DhYCi6pvfhJkPRpt5RjYwsE1hZw84iu6twbRt9B6dYLV'),
    },
  },
}
```
