---
title: Connecting to Umi
metaTitle: Umi - Connecting to Umi
description: Learn how hook up Umi to Node and front end clients such as React.
---

## Connecting to an RPC

Connecting Umi to an RPC is as simple as creating an umi instance and passing through your rpc end point as the first argument. It is recommended to use at least a free RPC end point from one of the many Solana RPC providers and not use the public endpoint of `https://api.mainnet-beta.solana.com` due to it's restrictions and limitations.

Connecting Umi to devnet is as simple as swapping out the RPC end point for that of a Devnet endpoint.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'

const umi = createUmi('https://api.mainnet-beta.solana.com')
```

## Registering Programs and Clients

Sometimes Umi may require you to register programs/clients directly to it. To do this you can call the `.use()` method from your umi instance and pass in the client as the argument. In the below example we'll register the `mpl-token-metdata` client to UMI.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata'

const umi = createUmi('https://api.mainnet-beta.solana.com').use(
  mplTokenMetadata()
)
```

You can chain `.use()` together to register multiple clients.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata'
import { mplCandyMachine } from '@metaplex-foundation/mpl-candy-machine'

const umi = createUmi('https://api.mainnet-beta.solana.com')
  .use(mplTokenMetadata())
  .use(mplCandyMachine())
```

## Connecting w/ a Secret Key

To use Umi you'll need to register a wallet in order to send transactions. To use a file system wallet you can import the json stored private key and convert it to a keypair for use with Umi.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata'
import { mplCandyMachine } from '@metaplex-foundation/mpl-candy-machine'

// Create Umi Instance
const umi = createUmi('https://api.mainnet-beta.solana.com')

// Import your private key file and parse it.
const wallet = './my-wallet.json'
const secretKey = JSON.parse(fs.readFileSync(wallet, 'utf-8'))

// Create a keypair from your private key
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(secretKey))

// Register it to the Umi client.
umi.use(keypairIdentity(keypair))
```

## Connecting w/ Wallet Adapter

Umi can connect to `solana-labs/wallet-adapter` directly to provide a seemless experiance for users on your front end. This prebuilt wallet UI is a great starting place for websites that are looking for user transactions and interactions. For this example we'll create a simple `useUmi` hook in React.

```ts
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { walletAdapterIdentity } from '@metaplex-foundation/umi-signer-wallet-adapters'
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata'
import { mplCandyMachine } from '@metaplex-foundation/mpl-candy-machine'
import { useWallet } from '@solana/wallet-adapter-react'

const useUmi = () => {
  // Import useWallet hook
  const wallet = useWallet()

  // Create Umi instance
  const umi = createUmi('https://api.mainnet-beta.solana.com')
    .use(mplTokenMetadata())
    .use(mplCandyMachine())
    // Register Wallet Adapter to Umi
    .use(walletAdapterIdentity(wallet))

  return umi
}

export default useUmi
```

From here on you can import your `useUmi` hook into your components and use as needed.

```ts
// Import hook from where you saved it
import { useUmi } from '@hooks/useUmi'

// Your component
const MyComponent = () => {
  // Assign you hook to a const within component ready for use
  const umi = useUmi()

  return <div>...</div>
}

export default MyComponent
```
