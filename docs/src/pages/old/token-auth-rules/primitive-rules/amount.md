---
title: Amount
metaTitle: Token Auth Rules - Amount
description: The Amount primitive rule
---

## Amount
The amount of tokens being transferred is compared (greater than, less than, or equal to) against an amount.

### Fields
* **amount** - The amount to be compared against
* **operator** - The comparison operation to use: greater than, less than, equal to
* **field** - The payload field to compare against

```js
// This Rule Set will only evaluate to true if more than 5 tokens are being transferred.
const revision: RuleSetRevisionV2 = {
  libVersion: 2,
  name: 'My Rule Set',
  owner,
  operations: {
    transfer: {
      type: 'Amount',
      field: 'Amount',
      operator: '>'
      amount: 5,
    },
  },
}
```