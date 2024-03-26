---
title: Getting Started using Rust
metaTitle: Token Metadata - Getting Started - Rust
description: Get started with NFTs using Rust
---

If you are a Rust developer, you can also use a Rust client SDK to interact with the Token Metadata program. Metaplex provides a dedicated Rust client crate, which is a lightweight crate with minimal dependencies.

To get started, you'll need to add the `mpl-token-metadata` dependency to your project. From a terminal on the root folder of your project:
```
cargo add mpl-token-metadata
```
This will all the latest version of the crate in your project's dependency list.

{% callout %}

If you are using a solana-program version prior to 1.16, first add the `solana-program` dependency to your project and then add `mpl-token-metadata`. This will make sure you only have a single copy of the `borsh` crate.

{% /callout %}

## 🧱 Structure

The client SDK is divided into several modules:

- `accounts`: structs representing the accounts of the program
- `errors`: enum representing program errors
- `instructions`: structs to facilitate the creation of instructions from client (off-chain) and programs (on-chain), and instruction arguments
- `types`: structs representing types used by the program

A good starting point to explore is the `instructions` module, which contains helpers to create instructions to interact with Token Metadata. These are designed to be flexible and easy-to-use. If an instruction requires additional types, these will be referenced from the `types` module. If you want to deserialize the content of a Token Metadata account, the `accounts` module has a struct representing each account with helpers methods to deserialize their content.

## 🏗️ Instruction Builders

One of the main features of the client SDK is to facilitate the creation of instructions. There are two _types_ of instruction builders depending on whether you are writing off-chain or on-chain code, and both support passing accounts by name and optional positional accounts.

### Client (off-chain)

These are intended to be used by off-chain client code. Each instruction is represented by a struct, where its fields are the `Pubkey`s of the required accounts.

{% totem %}
{% totem-prose %}

`CreateV1` instruction struct:

{% /totem-prose %}

```rust
pub struct CreateV1 {
    /// Unallocated metadata account with address as pda
    /// of ['metadata', program id, mint id]
    pub metadata: Pubkey,

    /// Unallocated edition account with address as pda
    /// of ['metadata', program id, mint, 'edition']
    pub master_edition: Option<Pubkey>,

    /// Mint of token asset
    pub mint: (Pubkey, bool),

    /// Mint authority
    pub authority: Pubkey,

    /// Payer
    pub payer: Pubkey,

    /// Update authority for the metadata account
    pub update_authority: (Pubkey, bool),

    /// System program
    pub system_program: Pubkey,

    /// Instructions sysvar account
    pub sysvar_instructions: Pubkey,

    /// SPL Token program
    pub spl_token_program: Pubkey,
}
```

{% /totem %}


After filling in the instruction account fields, you can use the `instruction(...)` method to generate the corresponding Solana `Instruction`:

{% totem %}
{% totem-prose %}

Creating an `Instruction` for `CreateV1`:

{% /totem-prose %}

```rust
// instruction args
let args = CreateV1InstructionArgs {
    name: String::from("My pNFT"),
    symbol: String::from("MY"),
    uri: String::from("https://my.pnft"),
    seller_fee_basis_points: 500,
    primary_sale_happened: false,
    is_mutable: true,
    token_standard: TokenStandard::ProgrammableNonFungible,
    collection: None,
    uses: None,
    collection_details: None,
    creators: None,
    rule_set: None,
    decimals: Some(0),
    print_supply: Some(PrintSupply::Zero),
};

// instruction accounts
let create_ix = CreateV1 {
    metadata,
    master_edition: Some(master_edition),
    mint: (mint_pubkey, true),
    authority: payer_pubkey,
    payer: payer_pubkey,
    update_authority: (payer_pubkey, true),
    system_program: system_program::ID,
    sysvar_instructions: solana_program::sysvar::instructions::ID,
    spl_token_program: spl_token::ID,
};

// creates the instruction
let create_ix = create_ix.instruction(args);
```

{% /totem %}

At this point, `create_ix` is an `Instruction` ready to be added to a transaction and sent for processing.

In the example above, you probably noticed that even when we do not need to provide a value for an optional argument, we still need to specify `None`. To facilitate the creation of instructions even further, you can use the `*Builder` _companion_ struct.

{% totem %}
{% totem-prose %}

Creating an `Instruction` using `CreateV1Builder`:

{% /totem-prose %}

```rust
let create_ix = CreateV1Builder::new()
    .metadata(metadata)
    .master_edition(Some(master_edition))
    .mint(mint_pubkey, true)
    .authority(payer_pubkey)
    .payer(payer_pubkey)
    .update_authority(payer_pubkey, true)
    .is_mutable(true)
    .primary_sale_happened(false)
    .name(String::from("My pNFT"))
    .uri(String::from("https://my.pnft"))
    .seller_fee_basis_points(500)
    .token_standard(TokenStandard::ProgrammableNonFungible)
    .print_supply(PrintSupply::Zero)
    .instruction();
```

{% /totem %}

The end result is the same `create_ix` instruction to be added to a transaction and sent for processing.

### Cross Program Invocation (on-chain)

When you are writing a program that needs to interact with Token Metadata, you can use the on-chain Cross Program Invocation (CPI) builder. They work similarly to off-chain builders, with the main difference being that they expect `AccountInfo` references instead of `Pubkey`s.

{% totem %}
{% totem-prose %}

`TransferV1Cpi` instruction struct:

{% /totem-prose %}

```rust
pub struct TransferV1Cpi<'a> {
    /// The program to invoke.
    pub __program: &'a AccountInfo<'a>,

    /// Token account
    pub token: &'a AccountInfo<'a>,

    /// Token account owner
    pub token_owner: &'a AccountInfo<'a>,

    /// Destination token account
    pub destination_token: &'a AccountInfo<'a>,

    /// Destination token account owner
    pub destination_owner: &'a AccountInfo<'a>,

    /// Mint of token asset
    pub mint: &'a AccountInfo<'a>,

    /// Metadata (pda of ['metadata', program id, mint id])
    pub metadata: &'a AccountInfo<'a>,

    /// Edition of token asset
    pub edition: Option<&'a AccountInfo<'a>>,

    /// Owner token record account
    pub token_record: Option<&'a AccountInfo<'a>>,

    /// Destination token record account
    pub destination_token_record: Option<&'a AccountInfo<'a>>,

    /// Transfer authority (token owner or delegate)
    pub authority: &'a AccountInfo<'a>,

    /// Payer
    pub payer: &'a AccountInfo<'a>,

    /// System Program
    pub system_program: &'a AccountInfo<'a>,

    /// Instructions sysvar account
    pub sysvar_instructions: &'a AccountInfo<'a>,

    /// SPL Token Program
    pub spl_token_program: &'a AccountInfo<'a>,

    /// SPL Associated Token Account program
    pub spl_ata_program: &'a AccountInfo<'a>,

    /// Token Authorization Rules Program
    pub authorization_rules_program: Option<&'a AccountInfo<'a>>,

    /// Token Authorization Rules account
    pub authorization_rules: Option<&'a AccountInfo<'a>>,

    /// The arguments for the instruction.
    pub __args: TransferV1InstructionArgs,
}
```

{% /totem %}

The instruction struct requires three different pieces of information: (1) the program to CPI into it – `__program` field; (2) a variable list of accounts represented by references to `AccountInfo`; (3) the instruction args – `__args` field. To simplify the creation of the struct, there is a `new(...)` factory method. After filling in the program, instruction accounts and argument fields, you can use the `invoke()` or `invoke_signed(...)` method to perform the CPI.

{% totem %}
{% totem-prose %}

Invoking the `TransferV1Cpi` instruction:

{% /totem-prose %}

```rust
// creates the instruction
let cpi_transfer = TransferV1Cpi::new(
    metadata_program_info,
    TransferV1CpiAccounts {
        token: owner_token_info,
        token_owner: owner_info,
        destination_token: destination_token_info,
        destination_owner: destination_info,
        mint: mint_info,
        metadata: metadata_info,
        authority: vault_info,
        payer: payer_info,
        system_program: system_program_info,
        sysvar_instructions: sysvar_instructions_info,
        spl_token_program: spl_token_program_info,
        spl_ata_program: spl_ata_program_info,
        edition: edition_info,
        token_record: None,
        destination_token_record: None,
        authorization_rules: None,
        authorization_rules_program: None,
    },
    TransferV1InstructionArgs {
        amount,
        authorization_data: None,
    },
);

// performs the CPI
cpi_transfer.invoke_signed(&[&signer_seeds])
```

{% /totem %}

You have probably noticed (again) that for every optional account/argument that we do not pass a value, we still need to set it to `None`. Similarly to the off-chain instructions, CPI instructions have a _companion_ `*Builder` struct.

{% totem %}
{% totem-prose %}

Invoking the `TransferV1Cpi` instruction using `TransferV1CpiBuilder`:

{% /totem-prose %}

```rust
// creates the instruction
let cpi_transfer = TransferV1CpiBuilder::new(metadata_program_info)
    .token(owner_token_info)
    .token_owner(owner_info)
    .destination_token(destination_token_info)
    .destination_owner(destination_info)
    .mint(mint_info)
    .metadata(metadata_info)
    .edition(edition_info)
    .authority(vault_info)
    .payer(payer_info)
    .system_program(system_program_info)
    .sysvar_instructions(sysvar_instructions_info)
    .spl_token_program(spl_token_program_info)
    .spl_ata_program(spl_ata_program_info)
    .amount(amount);

// performs the CPI
cpi_transfer.invoke_signed(&[&signer_seeds])
```

{% /totem %}

## 🔎 PDA helpers

Another set of useful helpers of the SDK are the PDA lookups. Account types representing PDAs (e.g., `Metadata`) have associated functions to find/create PDA `Pubkey`s.

{% totem %}
{% totem-prose %}

Implementation of `find_pda` and `create_pda` helper methods:

{% /totem-prose %}

```rust
impl Metadata {
    pub fn find_pda(mint: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                crate::MPL_TOKEN_METADATA_ID.as_ref(),
                mint.as_ref(),
            ],
            &crate::MPL_TOKEN_METADATA_ID,
        )
    }

    pub fn create_pda(
        mint: Pubkey,
        bump: u8,
    ) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(
            &[
                "metadata".as_bytes(),
                crate::MPL_TOKEN_METADATA_ID.as_ref(),
                mint.as_ref(),
                &[bump],
            ],
            &crate::MPL_TOKEN_METADATA_ID,
        )
    }
}
```

{% totem-prose %}

The `find_pda` method is usually used on off-chain clients:

```rust
let (metadata_pubkey, _) = Metadata::find_pda(mint);
```
{% /totem-prose %}
{% totem-prose %}

The `create_pda` method is recommended to be used on-chain, since it can save compute units in comparison to `find_pda`, but it does require storing the `bump` used to generate the PDA derivation:

```rust
let metadata_pubkey = Metadata::create_pda(mint, bump)?;
```

{% /totem-prose %}
{% /totem %}


## 🔗 Helpful links

- [GitHub repository](https://github.com/metaplex-foundation/mpl-token-metadata/blob/main/clients/rust)
- [Crate page](https://crates.io/crates/mpl-token-metadata)
- [API references](https://docs.rs/mpl-token-metadata/latest/mpl_token_metadata/index.html)
