# Overview

{% callout type="warning" %}

Please note that this program is marked as deprecated and is no longer actively maintained by the Metaplex Foundation team. New features, security fixes and backward compatibility are not guaranteed. Please use with caution.

{% /callout %}

## Introduction

Metaplex's Token Entangler program is pulled straight out of quantum mechanics! It enables you to entangle two NFTs together and make it so only one can exist in the wild at a time (and can always be exchanged with the entangled NFT). This could be useful to **'de-rug'** projects by replacing all the rugged NFTs with a new non-rug set. That's also the reason for Token Entanglers creation: To help the Exiled Apes community after the Degen Ape Academy’s erroneous mint. You can find more about the back story on the exiled apes website.

The idea behind the program was that the initially minted NFTs with damaged metadata could be swapped to new NFTs containing proper metadata. It could also be used for derugging projects or more creative use cases.

These swaps are possible back and forth at any given time, even if the NFT that is not in the escrow currently is sold to a different wallet the new wallet will be able to swap it back again.

## Opportunities

The Token Entangler Program is very simple. It takes a NFT A and returns the NFT B which has already been assigned to NFT A on token entangler creation. Nevertheless there are some opportunities which might be interesting for you:

- **Swap back and Forth**: If the user swaps NFT A for NFT B he can always reverse that swap again.
- **Swapping Fee**: You can introduce a swapping fee that is either paid every time a token is swapped or only once per NFT pair.
- **SPL token fee**: The swapping fee can either be paid with SPL tokens or SOL.

## How it works

The user facing process is easy. They pay the Token Entangler with NFT A and (if configured SOL or SPL tokens) and receive the entangled mint B:

![Image showing the general Token Entangler process. It shows a Wallet and the Token Entangler Program as a box. The boxes are connected with two arrows. One from Wallet to Entangler with annotation "NFT A + SOL" and another one from Entangler to Wallet with annotation "NFT B"](https://github.com/metaplex-foundation/docs/blob/main/static/assets/programs/token-entangler/Token-Entangler-Overview-Process.png?raw=true)

This is a very reduced picture that just shows the user facing process. There are additional Accounts etc. that are not shown in this image.

## Create your own!

The start to end from a general point of view looks like this:

1. Mint the new Tokens
2. Entangle the old and new NFTs
3. Host a customer facing website. There is a [sample UI implementation](https://github.com/metaplex-foundation/token-entangler-ui)
4. Have your users swap their NFTs!

## Further Information

More general information about the Token Entangler Program can be found here in the documentation:

- Getting Started
- Accounts
- Instructions
- CLI
- FAQ
- Changelog

If you want to use the Token Entangler you can e.g. use

- [JS CLI](https://github.com/metaplex-foundation/deprecated-clis/blob/main/src/token-entangler-cli.ts)
- [Token Entangler UI](https://github.com/metaplex-foundation/token-entangler-ui)

Also feel free to Checkout the [GitHub Repository](https://github.com/metaplex-foundation/metaplex-program-library/tree/master/token-entangler/) if you want to look into Token Entangler code.
