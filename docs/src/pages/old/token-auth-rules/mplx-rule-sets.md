---
title: Overview
metaTitle: Token Auth Rules - Metaplex Rule Sets
description: The pNFT Rule Sets that Metaplex Foundation manages.
---
The Metaplex Foundation manages two Rule Sets for pNFTs.

## Metaplex Foundation Rule Set
This rule set represents the best efforts of finding and blocking programs that do not enforce marketplace royalties. This status is periodically checked and any updates are performed to continue comprehensive creator royalty enforcement.

### Address:
**Devnet:** eBJLFYPxJmMGKuFwpDWkzxZeUrad92kZRC5BJLpzyT9

**Mainnet:** eBJLFYPxJmMGKuFwpDWkzxZeUrad92kZRC5BJLpzyT9

### Contents
Note that at the moment there are no programs bypassing creator royalties, therefore the rule set does not currently block any programs.

```json
{
  "libVersion": 1,
  "owner": "ELskdHjzTQ6F4bBibhk4iqy63gSPs8ELec9HbfAaSDJk",
  "ruleSetName": "Metaplex Foundation Rule Set",
  "operations": {
    "Transfer:WalletToWallet": "Pass",
    "Transfer:Owner": "Pass",
    "Transfer:MigrationDelegate": "Pass",
    "Transfer:SaleDelegate": "Pass",
    "Transfer:TransferDelegate": "Pass",
    "Delegate:LockedTransfer": "Pass",
    "Delegate:Update": "Pass",
    "Delegate:Transfer": "Pass",
    "Delegate:Utility": "Pass",
    "Delegate:Staking": "Pass",
    "Delegate:Authority": "Pass",
    "Delegate:Collection": "Pass",
    "Delegate:Use": "Pass",
    "Delegate:Sale": "Pass"
  }
}
```

## Compatibility Rule Set
This rule set doesn't block any programs and serves to allow the same level of transfer restrictions (i.e. None) as standard NFTs.

### Address:
**Devnet:** AdH2Utn6Fus15ZhtenW4hZBQnvtLgM1YCW2MfVp7pYS5

**Mainnet:** AdH2Utn6Fus15ZhtenW4hZBQnvtLgM1YCW2MfVp7pYS5

### Contents
```json
{
  "libVersion": 1,
  "owner": "ELskdHjzTQ6F4bBibhk4iqy63gSPs8ELec9HbfAaSDJk",
  "ruleSetName": "Compatibility Rule Set",
  "operations": {
    "Transfer:WalletToWallet": "Pass",
    "Transfer:Owner": "Pass",
    "Transfer:MigrationDelegate": "Pass",
    "Transfer:SaleDelegate": "Pass",
    "Transfer:TransferDelegate": "Pass",
    "Delegate:LockedTransfer": "Pass",
    "Delegate:Update": "Pass",
    "Delegate:Transfer": "Pass",
    "Delegate:Utility": "Pass",
    "Delegate:Staking": "Pass",
    "Delegate:Authority": "Pass",
    "Delegate:Collection": "Pass",
    "Delegate:Use": "Pass",
    "Delegate:Sale": "Pass"
  }
}

```