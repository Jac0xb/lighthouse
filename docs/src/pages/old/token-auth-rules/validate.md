---
title: Validating with a Rule Set
metaTitle: Token Auth Rules - Validate
description: How to run validation using a Rule Set
---

## Introduction
The Validate instruction on Token Authorization Rules takes an operation, token, and payload and uses that data to evaluate the rule set to determine if the operation is allowed. The most common use of this is royalty enforcement on Metaplex programmable NFTs. On pNFTs, the Delegate and Transfer instructions are **Operations**, the NFT that is being delegated or transferred is the **Token**, and the **Payload** is constructed using the different attributes of the Operation.

## Constructing the Payload
Most [Primitive Rules](/token-auth-rules/primitive-rules/) include a **field**. This is meant to indicate a field in the Payload passed in to the validate call. The values stored in these fields are fetched from the Payload HashMap and used for evaluation of the various rules. Below is an example of how Token Metadata constructs a payload for transferring an NFT.

```rust
// Transfer Amount
auth_data
    .payload
    .insert("Amount", PayloadType::Number(amount));

// Transfer Authority
auth_data.payload.insert(
    "Authority",
    PayloadType::Pubkey(*authority_info.key),
);

// Transfer Source
auth_data.payload.insert(
    "Source",
    PayloadType::Pubkey(*source_info.key),
);

// Transfer Destination
auth_data.payload.insert(
    "Destination",
    PayloadType::Pubkey(*destination_info.key),
);
```

## Calling Validate
- **rule_set_pda** - The PDA that contains the Rule Set that will be evaluated.
- **mint** - The mint of the token that is being operated on.
- **system_program** - The System Program
- **payer** (optional) - The user that will pay for any state changes.
- **rule_authority** (optional) - The authority that must sign to modify the state.
- **rule_set_state_pda** (optional) - The PDA that contains any state for the Rule Set (currently unused).
- **operation** - The operation that is being performed on the token (e.g. Transfer, Delegate).
- **payload** - A HashMap containing detailed information for what is happening to the token.
- **update_rule_state** - A boolean that indicates whether or not the Rule Set state is being updated.
- **rule_set_revision** (optional) - The revision of the Rule Set to be evaluated. If None, the latest revision is used.

```rust
let validate_ix = ValidateBuilder::new()
    .rule_set_pda(*ruleset.key)
    .mint(*mint_info.key)
    .additional_rule_accounts(account_metas)
    .build(ValidateArgs::V1 {
        operation: operation.to_string(),
        payload: auth_data.payload.clone(),
        update_rule_state: false,
        rule_set_revision,
    })
    .map_err(|_error| ErrorCode::InvalidAuthorizationRules)?
    .instruction();

let mut account_infos = vec![ruleset.clone(), mint_info.clone()];
account_infos.extend(additional_rule_accounts.into_iter().cloned());
invoke_signed(&validate_ix, account_infos.as_slice(), &[])
```

## Resources

- [Token Auth Rule GitHub repository](https://github.com/metaplex-foundation/mpl-token-auth-rules)
- [TypeScript references for the JS client](https://mpl-token-auth-rules-js-docs.vercel.app/)
