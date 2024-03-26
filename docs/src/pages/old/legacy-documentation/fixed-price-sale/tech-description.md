---
title: Technical Description
metaTitle: Fixed-Price-Sale - Technical Description
description: Technical Description of the Fixed Price Sale Program
---

## Creators to sell something have to:

1. Create store

    - It's necessary because we have to filter Markets somehow

    - It will contain name, admin key, description

2. Initialise Selling resource. It can be either created one or our platform will create it.

    - Once user initialise selling resource we have an object with resource which we can sell

3. Create a Market

    - Create object with info about items selling, all apart from max supply such as we defined it in Selling resource

## Users to buy tokens have to:

1. Go to store. 

2. Choose token and click "Buy"

    - Under the hood next things will happen:

        - TradeHistory account will be created where we track how many tokens this user already bought

        - Debit and credit operations

        - New NFT created(create mint, mint token, create Metadata, create MasterEdition)

3. Token will be shown in their wallets

# Accounts

## Store

| Field      | Type |Description|
| ----------- | ----------- | ------ |
| admin      | `Pubkey`       | Admin key who can create selling resources and markets in specific store       |
|  name  |  `String`  |   |
|  description  |  `String`  |   |

## Selling resource

| Field      | Type |Description|
| ----------- | ----------- | ------ |
|  store  |  `Pubkey`  |    |
|  owner  |  `Pubkey`  |  Owner of resource. This account can receive back resource once sail is ended  |
|  resource  |  `Pubkey`  |  Mint account Metadata attached to. We don’t need store Metadata key because it’s PDA and we can calculate it knowing the mint key  |
|  vault  |  `Pubkey`  |  Token account which holds MasterEdition  |
|  vault_owner  |  `Pubkey`  |  PDA with seeds [“mt_vault“, resource.key(), store.key()]  |
|  supply  |  `u64`  |  Amount of tokens already sold  |
|  max_supply  |  `Option<u64>`  |  Max amount of token can be sold  |
|  state  |  `Enum{Uninitialised, Created, InUse, Exhausted, Stoped,}`  |  State of resource  |

## Market

| Field      | Type |Description|
| ----------- | ----------- | ------ |
|  store  |  `Pubkey`  |    |
|  selling_resource  |  `Pubkey`  |    |
|  treasury_mint  |  `Pubkey`  |  Mint account of tokens which market will accept as a payment  |
|  treasury_holder  |  `Pubkey`  |  Token account buyers will send tokens to. Only market owner can withdraw assets  |
|  treasury_owner  |  `Pubkey`  |  PDA[“holder“, treasury_mint.key(), selling_resource.key()]  |
|  owner  |  `Pubkey`  |  Market owner  |
|  name  |  `String`  |    |
|  description  |  `String`  |    |
|  mutable  |  `bool`  |    |
|  price  |  `u64`  |    |
|  pieces_in_one_wallet  |  `Option<u64>`  |  How many tokens we can sell to one wallet  |
|  start_date  |  `u64`  |    |
|  end_date  |  `Option<u64>`  |    |
|  state  |  `Enum {Uninitialised, Created, Active, Ended,}`  |    |
|  funds_collected  |  `u64`  |    |


## TradeHistory

### PDA [“history“, wallet.key(), market.key()]

| Field      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  `Pubkey`  |    |
|  wallet  |  `Pubkey`  |    |
|  already_bought  |  `u64`  |  How many tokens user already bought from specific Market  |

## PrimaryMetadataCreators

### PDA [“primary_creators“, metadata.key()]

| Field      | Type |Description|
| ----------- | ----------- | ------ |
|  creators  |  `Vec<mpl_token_metadata::state::Creator>`  |  List of creators to receive primary sales royalties  |

# Instructions

## CreateStore

Creates new Store account.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  admin  |  Key, Signer, Writable  |    |
|  store  |  Key, Signer, Writable  |  Uninitialized account  |
|  name  |  `String`  |    |
|  description  |  `String`  |    |

## InitSellingResource

Initialize SellingResource account which will be used by Market.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  store  |  Key  |    |
|  store_admin  |  Key, Signer, Writable  |  Holds resource_token and pays for selling_resource account creating  |
|  selling_resource  |  Key, Signer, Writable  |  Uninitialized account  |
|  selling_resource_owner  |  Key  |  Key which can withdraw MasterEdition once sale is ended  |
|  resource_mint  |  Key  |  Mint account Metadata attached to  |
|  master_edition  |  Key  |  PDA with seeds [“metadata”, tokenMetadataProgramID, resource_mint, “edition”]  |
|  metadata  |  Key  |  Master edition’s metadata  |
|  vault  |  Key, Writable  |  Token account to hold resource  |
|  vault_owner  |  PDA [“mt_vault“, resource_mint.key(), store.key()]  |  Owner of vault token account  |
|  resource_token  |  Key, Writable  |  User’s token account which holds token from resource_mint  |
|  max_supply  |  `Option<u64>`  |  Max amount of tokens to sell  |

## CreateMarket

Initialize Market account. Set state to Created, it means that owner can change some data before it will be activated, off course if Market marked as mutable.

:::warning

If user want sell art for native SOL as `treasury_mint` should be set `11111111111111111111111111111111` also treasury_holder and treasury_owner should be the same accounts PDA. It’s necessary for security reasons so only program will be able to spend that SOL.

:::

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key, Signer, Writable  |  Uninitialized account  |
|  store  |  Key  |    |
|  selling_resource_owner  |  Key, Signer, Writable  |    |
|  selling_resource  |  Key, Writable  |    |
|  treasury_mint  |  Key  |  Mint of assets which we will take as a payment  |
|  treasury_holder  |  Key  |  Token account  |
|  treasury_owner  |  PDA [“holder“, treasury_mint.key(), selling_resource.key()]  |    |
|  name  |  `String`  |    |
|  description  |  `String`  |    |
|  mutable  |  `bool`  |    |
|  price  |  `u64`  |    |
|  pieces_in_one_wallet  |  `Option<u64>`  |    |
|  start_date  |  `u64`  |    |
|  end_date  |  `Option<u64>`  |    |
|  gating_config  |  `Option<GatingConfig{collection: Pubkey, expire_on_use: bool, gating_time: Option<u64>}>`  |  Gating token. If this value set only users with NFT from pointed collection can buy new NFTs from market.  |

## ChangeMarket

Available only if Market::mutable == true. Can change: name, description, mutable, price, pieces_in_one_wallet.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key, Writable  |    |
|  market_owner  |  Key, Signer  |    |
|  new_name  |  `Option<String>`  |    |
|  new_description  |  `Option<String>`  |    |
|  mutable  |  `Option<bool>`  |    |
|  new_price  |  `Option<u64>`  |    |
|  new_pieces_in_one_wallet  |  `Option<u64>`  |    |

## Buy

User can call only if current date > Market::start_date.

:::warning

If user buy art for native SOL user_token_acc and user_wallet accounts should be the same.

:::

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key, Writable  |    |
|  selling_resource  |  Key, Writable  |    |
|  user_token_acc  |  Key, Writable  |  Token account to pay for the member token. Mint of this token acc should be == treasury_mint  |
|  user_wallet  |  Key, Signer, Writable  |    |
|  trade_history  |  Key, Writable  |  Account to track how many NFTs user already bought  |
|  treasury_holder  |  Key, Writable  |    |
|  new_metadata_acc  |  Key, Writable  |    |
|  new_edition_acc  |  Key, Writable  |    |
|  master_edition_acc  |  Key, Writable  |    |
|  new_mint  |  Key, Writable  |    |
|  edition_marker  |  Key, Writable  |  PDA, seeds can be found in token-metadata program  |
|  vault  |  Key  |    |
|  vault_owner  |  PDA [“mt_vault“, resource.key(), store.key()]  |    |
|  master_edition_metadata  |  Key  |    |
|    |  Below accounts are optional and should be passed only if gating feature is enabled ↓  |    |
|  user_collection_token_account  |  Key, Writable  |  User’s token account from collection  |
|  token_account_mint  |  Key, Writable  |  Token’s mint account  |
|  metadata_account  |  Key  |  Metadata account for the mint mentioned above  |

## SuspendMarket

Suspend Market so nobody can buy items and market owner can change data. Instruction should be available only if Market::mutable == true because in other case there is no reason to suspend it.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key, Writable  |    |
|  market_owner  |  Key, Signer  |    |
|  clock  |  Key  |    |

## ResumeMarket

Instruction to resume the market after it was suspended. Can be called only if market is in suspended state.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key, Writable  |    |
|  market_owner  |  Key, Signer  |    |
|  clock  |  Key  |    |

## CloseMarket

This instruction can be called only if Market was created with unlimited duration.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key, Writable  |    |
|  market_owner  |  Key, Signer  |    |
|  clock  |  Key  |    |

## Withdraw

Called by Market owner to withdraw collected treasury funds. Available only if Market::state == Ended.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key  |    |
|  selling_resource  |  Key  |    |
|  metadata  |  Key  |    |
|  treasury_holder  |  Key, Writable  |  Market::treasury_holder. Token account which holds all the tokens received from users during selling  |
|  treasury_mint  |  Key  |    |
|  funder  |  Key  |    |
|  payer  |  Key, Signer  |    |
|  payout_ticket  |  Key, Writable  |  PDA[“payout_ticket“, market.key(), funder.key()]  |
|  treasury_owner  |  Key  |  PDA[“holder“, treasury_mint.key(), selling_resource.key()]  |
|  destination  |  Key, Writable  |  Token account transfer tokens to  |
|    |  Below account is optional and should be passed only during primary sale ↓  |    |
|  primary_metadata_creators_data  |  Key  |  List of creators who should receive royalties from primary sale  |

## ClaimResource

Called by Resource owner. Available only if SellingResource::state == Exhausted of Market::state == Ended.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  market  |  Key  |    |
|  treasury_holder  |  Key  |    |
|  selling_resource  |  Key  |    |
|  selling_resource_owner  |  Key, Signer  |    |
|  source  |  Key, Writable  |  SellingResource::vault. Token account which holds master edition  |
|  metadata  |  Key  |  Metadata for token which was sold  |
|  vault_owner  |  Key  |  PDA with seeds [“mt_vault“, resource.key(), store.key()]  |
|  secondary_metadata_creators  |  Key  |    |
|  destination  |  Key, Writable  |  Token account transfer master edition to  |

## SavePrimaryMetadataCreators

Called before market is created. This list of creators will be used in withdraw instruction to distribute royalties. Take a note that if you are going to sell NFTs from master edition with `primary_sale_happen = true` you don't need to call this instruction.

| Parameter      | Type |Description|
| ----------- | ----------- | ------ |
|  admin  |  Key, Signer, Writable  |  Metadata’s update authority  |
|  metadata  |  Key, Writable  |    |
|  primary_metadata_creators  |  Key, Writable  |  PDA with seeds [“primary_creators“, metadata.key()]  |
|  system_program  |  Key  |    |
|  primary_metadata_creators  |  `u8`  |  primary_metadata_creators key bump  |
|  creators  |  `Vec<mpl_token_metadata::state::Creator>`  |  List of creators who will receive primary royalties  |
