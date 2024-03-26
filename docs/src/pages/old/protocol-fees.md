---
title: Protocol Fees
metaTitle: Metaplex — Protocol Fees
description: A details of the on-chain fees for Metaplex's products.
---

The Metaplex Foundation currently charges the following protocol fees:

## **Revised Fee Schedule (as of May 24th, 2023)**

Based on community feedback, Metaplex Foundation has announced an updated fee schedule with the following changes:

- The Update, Verify, Freeze and Thaw fees for Token Metadata have been removed

| Instruction     | Program         | Typical Payer | Amount (SOL) | Notes                                                                                                                                                                                                                                                                                    |
| --------------- | --------------- | ------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Create          | Token Metadata  | Collector     | 0.01         | The minter for most NFTs created on Solana are the individual collectors minting from new drops. Creators that mint many NFTs may consider using compression for radically lower mint costs. (Includes all instructions that "create" an NFT including ones that create print editions.) |
| Combine         | Fusion (Trifle) | Collector     | 0.002        |                                                                                                                                                                                                                                                                                          |
| Split           | Fusion (Trifle) | Collector     | 0.002        |                                                                                                                                                                                                                                                                                          |
| Edit constraint | Fusion (Trifle) | Creator       | 0.01         |                                                                                                                                                                                                                                                                                          |

## Fee Schedule (as of May 22nd, 2023)

| Instruction     | Program         | Typical Payer | Amount (SOL) | Notes                                                                                                                                                                                        |
| --------------- | --------------- | ------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Create          | Token Metadata  | Collector     | 0.01         | The minter for most NFTs created on Solana are the individual collectors minting from new drops. Creators that mint many NFTs may consider using compression for radically lower mint costs. |
| Update          | Token Metadata  | Creator       | 0.002\*      | This fee applies only to the update instructions that are NOT called during the initial mint of an NFT.                                                                                      |
| Verify          | Token Metadata  | Creator       | 0.001\*      |                                                                                                                                                                                              |
| Freeze delegate | Token Metadata  | Collector     | 0.001\*      | This is NOT the spl-token freeze and applies only to NonFungible tokens (regular NFTs). This fee is paid by the person granting the authority to freeze their NFT, e.g. for staking          |
| Thaw delegate   | Token Metadata  | Collector     | 0.001\*      | This is NOT the spl-token freeze and applies only to NonFungible tokens (regular NFTs). This fee is paid by the person granting the authority to thaw their NFT, e.g. for staking            |
| Combine         | Fusion (Trifle) | Collector     | 0.002        |                                                                                                                                                                                              |
| Split           | Fusion (Trifle) | Collector     | 0.002        |                                                                                                                                                                                              |
| Edit constraint | Fusion (Trifle) | Creator       | 0.01         |                                                                                                                                                                                              |

\*These fees are currently under review and may not be the final amounts at launch.

## FAQs

### Will the fee amounts change over time?

We are constantly monitoring community feedback related to the fees and may change the fee amounts over time. Our goal is for fees to be minimally disruptive and promote the growth and usage of the protocol.

### How much will it cost me, as a creator, in Token Metadata fees to launch a 10k NFT collection through Candy Machine?

Creators will incur 0 SOL in Token Metadata fees for a standard 10k NFT drop since the Create fees are spread amongst the collectors who are minting from the Candy Machine.

### Do the freeze and thaw fees impact pNFT transfers?

No.
