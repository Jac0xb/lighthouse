---
title: Overview
metaTitle: Token Auth Rules - Overview
description: Provides a high-level overview of NFT permissions.
---
Token Authorization Rules (aka Token Auth Rules) is an advanced metaprogramming tool meant to evaluate permissions of an instruction occurring on an SPL Token. The program itself can be used to create and update Rule Sets, which are collections of Rules which represent specific criteria.

## Introduction
When a token operation is performed, the program can be called with instruction details (e.g. destination address for a token transfer), and those details are validated against the Rule Set. If the Rules are evaluated to be invalid, the instruction will fail and the whole transaction will be reverted. This enables whole transactions to be built that couple token operations with the Token Auth Rules program so any transactions, and therefore the contained token operation, will be reverted if the Rules in the associated Rule Set are violated.

## Features

[Create or Update Rule Sets](/token-auth-rules/create-or-update) - An instruction used to both initialize and update Rule Set contents.

[Rule Set Buffers](/token-auth-rules/buffers) - How large Rule Sets are handled.

[Validate Rule Sets](/token-auth-rules/validate) - How a Rule Set is validated.

## Rule Types
Authorization rules are variants of a `Rule` enum that implements a `validate()` method.

There are **Primitive Rules** and **Composite Rules** that are created by combining of one or more primitive rules.

**Primitive Rules** store any accounts or data needed for evaluation, and at runtime will produce a true or false output based on accounts and a well-defined `Payload` that are passed into the `validate()` function.

**Composite Rules** return a true or false based on whether any or all of the primitive rules return true.  Composite rules can then be combined into higher-level composite rules that implement more complex boolean logic.  Because of the recursive definition of the `Rule` enum, calling `validate()` on a top-level composite rule will start at the top and validate at every level, down to the component primitive rules.

## Operation
A Rule Set is built upon the `HashMap` data structure and is meant to store various sets of rules for different instruction types that could be used with a token (e.g. transfer, delegate, burn, etc.). Token Auth Rules uses the term **Operation** for these various instructions and **Operations** are used as keys in the `HashMap` data structure. Each **Operation** can have a different set of associated rules.

### Scenario
**Scenarios** are an optional addition to **Operations** and are used to handle more specific circumstances under which an instruction can be called. From a data format perspective, an **Operation** and **Scenario** combination is just two strings separated by a colon `<Operation>:<Scenario>`. For example, Token Metadata uses the authority type as a **Scenario** for calls to Token Auth Rules from Token Metadata. A Transfer **Operation** may be triggered on a token by either the token's owner or delegate, and the Rule Set manager may want these different scenarios to be governed by different rules. To handle this specific use case a **Scenario** can be used to manage the distinction. The the two `HashMap` keys used for the prior example would be `Transfer:Owner` and `Transfer:Delegate`.

Please see the [Namespace](/token-auth-rules/primitive-rules/namespace) for how to manage identical rules across multiple scenarios.

## Payload
The Token Auth Rules program relies on payload data received from the program requesting evaluation from a Rule Set. The underlying data structure of the `Payload` is a `HashMap`, with `Payload` fields being represented as `HashMap` keys. Most Rules store a pre-defined `Payload` field so a lookup can be performed at validation time.

See the [Validate](/token-auth-rules/validate) instruction for more details on how `Payload` is used.

## Resources

- [Token Auth Rule GitHub repository](https://github.com/metaplex-foundation/mpl-token-auth-rules)
- [TypeScript references for the JS client](https://mpl-token-auth-rules-js-docs.vercel.app/)
