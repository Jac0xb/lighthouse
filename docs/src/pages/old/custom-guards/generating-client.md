---
title: Generating Custom Guard Client
metaTitle: Candy Machine - Generating Custom Guard Client
description: How to generate a Umi compatible client for your custom guards.
---

Once you have written your custom guard for the Candy Machine Guard program you'll need to generate a Kinobi client that works with the Umi SDK to for example be able to use your guard in a frontend.

## Generate IDL and Initial Client

### Configuring Shankjs

Shankjs is a IDL generator that works on both Anchor and non Anchor programs. You want to configure this with your new custom Candy Guard deployment key to properly generate a working client. Edit the file located at `/configs/shank.cjs` in the mpl-candy-machine repo.

```js
/configs/shank.cjs

generateIdl({
  generator: "anchor",
  programName: "candy_guard",
  programId: "Guard1JwRhJkVH6XZhzoYxeBVQe872VH6QggF4BWmS9g", // Your custom Candy Guard deployed program key.
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "candy-guard", "program"),
});

```

{% callout %}
If you are generating using anchor 28 you will need to add a fallback in to Shankjs idl generator to anchor 27 due to a missing crates.io crate.
{% /callout %}

```js
/configs/shank.cjs

generateIdl({
  generator: "anchor",
  programName: "candy_guard",
  programId: "Guard1JwRhJkVH6XZhzoYxeBVQe872VH6QggF4BWmS9g", // Your custom Candy Guard deployed program key.
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "candy-guard", "program"),
  rustbin: {
    locked: true,
    versionRangeFallback: "0.27.0",
  },
});

```

### Generate IDL and Client

Now you should be able to generate the IDL and the initial client. From the root of the project run

```shell
pnpm run generate
```

this will in turn execute both scripts `pnpm generate:idls` and `pnpm generate:clients` and build out the intial clients.
If you need to run these seperately for what ever reason you are able to do so.

## Adding Guard(s) to the client

### Create Guard File

Once a sucessful generation of the intial client is made navigate to `/clients/js/src`.

The first step would be to add you new guard into the `/clients/js/src/defaultGuards` folder.

Below is a template you could use and adjust to your needs based on the type of guard you have created.
You can name your guard what ever you want but I'm going to name my example `customGuard.ts`

```ts
import { PublicKey } from '@metaplex-foundation/umi'
import {
  getCustomGuardSerializer,
  CustomGuard,
  CustomGuardArgs,
} from '../generated'
import { GuardManifest, noopParser } from '../guards'

export const customGuardManifest: GuardManifest<
  CustomGuardArgs,
  CustomGuard,
  CustomGuardMintArgs
> = {
  name: 'customGuard',
  serializer: getCustomGuardSerializer,
  mintParser: (context, mintContext, args) => {
    const { publicKeyArg1, arg1 } = args
    return {
      data: new Uint8Array(),
      // Pass in any accounts needed for your custom guard from your mint args.
      // Your guard may or may not need remaining accounts.
      remainingAccounts: [
        { publicKey: publicKeyArg1, isWritable: true },
        { publicKey: publicKeyArg2, isWritable: false },
      ],
    }
  },
  routeParser: noopParser,
}

// Here you would fill out any custom Mint args needed for your guard to operate.
// Your guard may or may not need MintArgs.

export type CustomGuardMintArgs = {
  /**
   * Custom Guard Mint Arg 1
   */
  publicKeyArg1: PublicKey

  /**
   * Custom Guard Mint Arg 2
   */
  publicKeyArg2: PublicKey

  /**
   * Custom Guard Mint Arg 3.
   */
  arg3: Number
}
```

### Add Guard to Existing Files

From here you need to add your new guard to some existing files.

Export your new guard from `/clients/js/src/defaultGuards.index.ts`

```ts
...
export * from './tokenGate';
export * from './tokenPayment';
export * from './token2022Payment';
// add your guard to the list
export * from './customGuard';
```

Within `/clients/js/src/defaultGuards.defaults.ts` add your guard to these locations;

```ts
import { CustomGuardArgs } from "../generated"

export type DefaultGuardSetArgs = GuardSetArgs & {
    ...
     // add your guard to the list
    customGuard: OptionOrNullable<CustomGuardArgs>;
}
```

```ts
import { customGuard } from "../generated"

export type DefaultGuardSet = GuardSet & {
    ...
     // add your guard to the list
    customGuard: Option<CustomGuard>
}
```

```ts
import { CustomGuardMintArgs } from "./defaultGuards/customGuard.ts"
export type DefaultGuardSetMintArgs = GuardSetMintArgs & {
    ...
    // add your guard to the list
    customGuard: OptionOrNullable<CustomGuardMintArgs>
}
```

```ts
export const defaultCandyGuardNames: string[] = [
  ...// add your guard to the list
  'customGuard',
]
```

Finally you need to add the exported customGuardManifest to the plugin file located at `/clients/js/src/plugin.ts`

```ts
import {customGuardManifest} from "./defaultGuards"

umi.guards.add(
  ...// add your guard manifest to the list
  customGuardManifest
)
```

From this point you can build and upload your client package to npm or link/move it to your project folder where you would like to access the new guard client.

It is worth using the built in testing suite of AVA to write some tests that fully test your guard in multiple scenarios. Examples of tests can be found in `/clients/js/tests`.
