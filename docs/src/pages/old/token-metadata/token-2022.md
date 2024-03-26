---
title: SPL Token-2022 on Token Metadata
metaTitle: Token Metadata - SPL Token-2022
description: Learn about how SPL Token-2022 is integrated with Token Metadata
---

SPL Token-2022 is the latest token program on the Solana blockchain that can be used to create fungible and non-fungible tokens. It supports the same functionalities and structures of the SPL Token program, but also includes a set of extensions to add new functionalities.

In order to support adding metadata information to Token-2022 mint accounts, a set of Token Metadata instructions have been updated to allow specified the desired token program. For example, Token Metadata can initialize a Token-2022 mint, create metadata and mint tokens using the `Create` and `Mint` instructions and specifying the SPL Token-2022 as the token program to use.

{% totem %}

{% dialect-switcher title="Specifying token program on Create and Mint" %}
{% dialect title="JavaScript" id="js" %}

{% totem-accordion title="Create Metadata" %}

```ts
import {
  generateSigner,
  percentAmount,
  publicKey,
  PublicKey
} from '@metaplex-foundation/umi'
import {
  createV1,
  TokenStandard,
} from '@metaplex-foundation/mpl-token-metadata'

const SPL_TOKEN_2022_PROGRAM_ID: PublicKey = publicKey(
  'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
);

const mint = generateSigner(umi)
await createV1(umi, {
  mint,
  authority,
  name: 'My NFT',
  uri,
  sellerFeeBasisPoints: percentAmount(5.5),
  splTokenProgram: SPL_TOKEN_2022_PROGRAM_ID,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion  %}

{% totem-accordion title="Mint a token" %}

```ts
import { mintV1, TokenStandard } from '@metaplex-foundation/mpl-token-metadata'

const SPL_TOKEN_2022_PROGRAM_ID: PublicKey = publicKey(
  'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb'
);

await mintV1(umi, {
  mint: mint.publicKey,
  authority,
  amount: 1,
  tokenOwner,
  splTokenProgram: SPL_TOKEN_2022_PROGRAM_ID,
  tokenStandard: TokenStandard.NonFungible,
}).sendAndConfirm(umi)
```

{% /totem-accordion  %}

{% /dialect %}

{% dialect title="Rust" id="rust" %}

{% totem-accordion title="Create Metadata" %}

```rust
use mpl_token_metadata::{
    instructions::CreateV1Builder,
    types::{PrintSupply, TokenStandard},
};
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
     message::Message,
     transaction::Transaction,
};

// 1. client is a reference to the initialized RpcClient
// 2. every account is specified by their pubkey

let client = ...;

let create_ix = CreateV1Builder::new()
    .metadata(metadata)
    .master_edition(Some(master_edition))
    .mint(mint.pubkey(), true)
    .authority(payer.pubkey())
    .payer(payer.pubkey())
    .update_authority(payer.pubkey(), false)
    .spl_token_program(spl_token_2022::id())
    .name(String::from("My NFT"))
    .uri(uri)
    .seller_fee_basis_points(550)
    .token_standard(TokenStandard::NonFungible)
    .print_supply(PrintSupply::Zero)
    .instruction();

let message = Message::new(
    &[create_ix],
    Some(&payer.pubkey()),
);

let blockhash = client.get_latest_blockhash()?;
let mut tx = Transaction::new(&[mint, payer], message, blockhash);
client.send_and_confirm_transaction(&tx)?;
```

{% /totem-accordion  %}

{% totem-accordion title="Mint a token" %}

```rust
use mpl_token_metadata::instructions::MintV1Builder;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
     message::Message,
     transaction::Transaction,
};

// 1. client is a reference to the initialized RpcClient
// 2. every account is specified by their pubkey

let client = ...;

let mint_ix = MintV1Builder::new()
    .token(token)
    .token_owner(Some(token_owner))
    .metadata(metadata)
    .master_edition(Some(master_edition))
    .mint(mint)
    .authority(update_authority)
    .payer(payer)
    .spl_token_program(spl_token_2022::id())
    .amount(1)
    .instruction();

let message = Message::new(
    &[mint_ix],
    Some(&payer.pubkey()),
);

let blockhash = client.get_latest_blockhash()?;
let mut tx = Transaction::new(&[update_authority, payer], message, blockhash);
client.send_and_confirm_transaction(&tx)?;
```

{% /totem-accordion  %}

{% /dialect %}

{% /dialect-switcher %}
{% totem-prose %}

The token program of a mint account can be determined by checking the `owner` property of the account.

{% /totem-prose %}

{% /totem %}

A similar approach can be used for other instructions, such as `Burn`, `Delegate`, `Lock`, `Print`, `Revoke`, `Transfer`, `Unlock`, `Unverify`, `Update`and `Verify`. These instruction can validate mint and token accounts from SPL Token-2022. The corresponding token program must be used in any instruction requiring a token program (e.g., `Delegate`): if the mint and token account are from Token-2022, then the `Delegate` instruction will validate that the correct token program has been specified.

{% callout %}
By default, `Create` and `Mint` will create SPL Token mint and token accounts if these accounts do not exist. To use Token-2022 accounts, you need to specify SPL Token-2022 as the token program to use.
{% /callout %}

## Supported Extensions

While Token-2022 provides several extensions, the majority of extensions focus on fungible tokens. For example, the `confidential transfer` can be used to hide the amount of tokens transferred. While this is relevant for fungibles, since the amount can vary across different transfers, it is not applicable to non-fungible tokens since their supply is always `1` and decimals is always `0`. Hence, the transfer amount of a non-fungible token will always be `1`.

Token Metadata enforces restrictions on the type of extensions that can be present on mint and token accounts based on the `Token Standard`. For fungible assets (`Fungible` and `FungibleAsset` standards), no restrictions are placed – the only restriction is on the program providing the metadata information. For non-fungible assets (`NonFungible` and `ProgrammableNonFungible` standards), Token Metadata validates which extensions are enabled and restricts the set of extensions that can be used.

### Mint account extensions

These are extensions that can be enabled on mint accounts of SPL Token-2022.

- `confidential transfers`: hides the amount during transfers.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * Not applicable since non-fungibles have supply of `1` {% align="center" %}
  {% /table %}

- `transfer fees`: allow to configure a transfer fee derived from the amount being transferred.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * Not applicable since non-fungibles have supply of `1` {% align="center" %}
  {% /table %}

- `closing mint`: allows closing mint accounts when supply reaches `0`.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * Must specify the `Metadata` account as the close authority {% align="center" %}
  * Potential for a creator to recreate the same group of mint and metadata accounts {% align="center" %}
  {% /table %}

- `interest-bearing tokens`: allows to change how the UI amount of tokens are represented.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * Not applicable since non-fungibles have supply of `1` {% align="center" %}
  {% /table %}

- `non-transferable tokens`: allows for "soul-bound" tokens that cannot be moved to any other address.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ✅ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * -- {% align="center" %}
  {% /table %}

- `permanent delegate`: allows to specify a permanent account delegate for any token account of a mint.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * This changes the concept of ownership {% align="center" %}
  {% /table %}

- `transfer hook`: allows call into third-party programs during transfer.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * Token Metadata specifies the logic for transfer {% align="center" %}
  {% /table %}

- `metadata pointer`: allows adding an address that describes the canonical metadata.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ✅ {% align="center" %}
  ---
  * Details 
  * Must point to the `Metadata` address {% align="center" %}
  * Must point to the `Metadata` address {% align="center" %}
  {% /table %}

- `metadata`: allow adding metadata directly to mint accounts.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ❌ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * Metadata information is added by Token Metadata {% align="center" %}
  * Metadata information is added by Token Metadata {% align="center" %}
  {% /table %}

### Token account extensions

These are extensions that can be enabled on token accounts of SPL Token-2022.

- `memo required`: requires memo on transfers.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * Not applicable {% align="center" %}
  {% /table %}

- `immutable ownership`: disables the ability to change the ownership of token accounts.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ✅ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * -- {% align="center" %}
  {% /table %}

- `default account state`: allows to configure default token account states.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * Token Metadata validates the account state {% align="center" %}
  {% /table %}

- `CPI guard`: prevent certain actions (e.g., transfer) inside cross-program invocations.

  {% table %}
  * Asset {% width="20%" %}
  * Fungible {% width="40%" %} {% align="center" %}
  * Non-Fungible {% width="40%" %} {% align="center" %}
  ---
  * Allowed 
  * ✅ {% align="center" %}
  * ❌ {% align="center" %}
  ---
  * Details 
  * -- {% align="center" %}
  * Token Metadata specifies the logic for transfer {% align="center" %}
  {% /table %}

{% callout %}
A comprehensibe overview of each extension can be found on SPL Token-2022 program [documentation](https://spl.solana.com/token-2022).
{% /callout %}

### Default extensions

When a mint account does not exist, the `Create` instruction will initialize one. If the token program being used is SPL Token-2022, the mint will be initialized with both `closing mint` and `metadata pointer` extensions.

Associated Token Accounts (ATAs) by default are always initialized with the `immutable ownership` extension.