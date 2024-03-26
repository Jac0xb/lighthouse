---
title: Android SDK
metaTitle: Android SDK
description: Metaplex Android SDK
---

The Metaplex Android SDK is a library that allows you to:

- Load and Deserialize Accounts
- Create transactions
- Run Actions (mint NFT, create an auction, and so on)

It works both in Android and other platforms that support kotlin.

## Stability

[Stability 1 - Experimental](/stability-index)

This project is in development. **All** interfaces are _very likely_ to change very frequently. Please use caution when making use of this library. Bugs or behavior changes may surprise users when Experimental API modifications occur.

## References

- [Source code][github]

## Getting started

### Installation
#### Requirements {#requirements}

- Android 21+

We recommend using the GitHub recommended way to load Artifacts. First get a GitHub Token from your [account settings](https://github.com/settings/tokens).

Inside settings.gradle add a maven repository:

```
repositories {
	...
	maven {
       name = "GitHubPackages"
       url = "https://github.com/metaplex-foundation/metaplex-android"
       credentials {
		   username = "<YOUR_GITHUB_USERNAME>"
		   password = "<YOUR_GITHUB_TOKENS>"
       }
	}
}
 
```

Then at your build.gradle:

```
dependencies {
	...
	implementation 'com.metaplex:metaplex:+' // Set version
}
```

After that gradle sync.

## JitPack Release

The library is now is available through JitPack.io

First, add the JitPack repository to your build:
```

repositories {
	...
	maven { url 'https://jitpack.io' }
}

```
Then add the dependency to the 'build.gradle' file for your app/module:
```
dependencies {
	...
	implementation 'com.github.metaplex-foundation:metaplex-android:{version}'
}
```

## Setup

The entry point to the Android SDK is a `Metaplex` instance that will give you access to its API.

Set the `SolanaConnectionDriver` and set up your environment. Provide a `StorageDriver` and `IdentityDriver`. You can also use the concrete implementations OkHttpSharedStorageDriver for OKHttp and ReadOnlyIdentityDriver for a read only Identity Driver. 

You can customize who the SDK should interact on behalf of and which storage provider to use when uploading assets. We might provide a default and simple implementation in the future.

```kotlin
val ownerPublicKey = PublicKey("<Any PublicKey>")
val solanaConnection = SolanaConnectionDriver(RPCEndpoint.mainnetBetaSolana)
val solanaIdentityDriver = ReadOnlyIdentityDriver(ownerPublicKey, solanaConnection.solanaRPC)
val storageDriver = OkHttpSharedStorageDriver()
val metaplex = Metaplex(solanaConnection, solanaIdentityDriver, storageDriver)
```

# Usage
Once properly configured, that `Metaplex` instance can be used to access modules providing different sets of features. Currently, there is only one NFT module that can be accessed via the `nft` property. From that module, you will be able to find, create and update NFTs with more features to come.

## NFTs
The NFT module can be accessed via `Metaplex.nft` and provide the following methods. Currently, we only support reading methods. Writing and creating NFTs will be supported on the future.

- findByMint(mint, callback)
- findAllByMintList(mints, callback)
- findAllByOwner(owner, callback)
- findAllByCreator(creator, position = 1, callback)
- findAllByCandyMachine(candyMachine, version = 2, callback)

All the methods return a callback. It's also possible to wrap them inside either RX or Async Result. We only provide this interface since is the most compatible without forcing any specific framework. 

### Your first request

The following code snippet is a basic one you can use to get NFTs from a publicKey. This use case maybe very common for a Wallet:

```kotlin
metaplex.nft.findByMint(mintPublicKey){
	it.onSuccess { 
		...
	}.onFailure { 
		...
	}
}
```

This will return an array of NFTs owned by that specific public key.

### The `Nft` model

All of the methods above either return or interact with an `Nft` object. The `Nft` object is a read-only data representation of your NFT that contains all the information you need at the top level.

You can see its full data representation by checking the code but here is an overview of the properties that are available on the `Nft` object.

```kotlin
class NFT(
    val metadataAccount: MetadataAccount,
    val masterEditionAccount: MasterEditionAccount?
) {

    val updateAuthority: PublicKey = metadataAccount.update_authority
    val mint: PublicKey = metadataAccount.mint
    val name: String = metadataAccount.data.name
    val symbol: String = metadataAccount.data.symbol
    val uri: String = metadataAccount.data.uri
    val sellerFeeBasisPoints: Int = metadataAccount.data.sellerFeeBasisPoints
    val creators: Array<MetaplexCreator> = metadataAccount.data.creators
    val primarySaleHappened: Boolean = metadataAccount.primarySaleHappened
    val isMutable: Boolean = metadataAccount.isMutable
    val editionNonce: Int? = metadataAccount.editionNonce
    val tokenStandard: MetaplexTokenStandard? = metadataAccount.tokenStandard
    val collection: MetaplexCollection? = metadataAccount.collection
	...
}
```

As you can see, some of the properties are loaded on demand. This is because they are not always needed and/or can be expensive to load.

In order to load these properties, you may run the `metadata` properties of the `Nft` object.

```kotlin
nft..metadata(metaplex) { result -> 
	it.onSuccess { 
		...
	}.onFailure { 
		...
	}
}
```

## Auctions
The Metaplex Auction House protocol allows anyone to implement a decentralized sales contract and accept ay SPL token they desire. 

The Auctions module can be accessed via `Metaplex.auctions` and provide the following methods. Currently we only support read methods. Auction House creation, and the ability to interact with and create bids and listings will be supported in the future.

- [`findAuctionHouseByAddress(address)`](#findAuctionHouseByAddress)
- [`findAuctionHouseByCreatorAndMint(creator, treasuryMint)`](#findAllByMintList)
- more coming soon!

All methods are provided as composable [suspending functions](https://kotlinlang.org/docs/composing-suspending-functions.html) to provide more flexibility and compatibility in your application.   

**Note:** These suspend functions provided by the Auctions API are an architectural change for the library. We have previously only provided async-callback methods. We highly recommend that everyone migrate to the new suspending functions, however we have also provided async-callback implementations of the available methods. Note that these methods are provided as a interim and may be deprecated in the future:

- [`findAuctionHouseByAddress(address, callback)`](#findAuctionHouseByAddress)
- [`findAuctionHouseByCreatorAndMint(creator, treasuryMint, callback)`](#findAllByMintList)

### findAuctionHouseByAddress

The `findAuctionHouseByAddress` method accepts a public key and returns an AuctionHouse object, or an error if no AuctionHouse was found for the given address.

```kotlin
val theAuctionHouse: AuctionHouse? = metaplex.auctions.findAuctionHouseByAddress(addressPublicKey).getOrNull()
```

### findAuctionHouseByCreatorAndMint

The `findAuctionHouseByCreatorAndMint` method accepts a public key and returns an AuctionHouse object, or an error if no AuctionHouse was found for the given address.

```kotlin
val theAuctionHouse: AuctionHouse? = metaplex.auctions.findAuctionHouseByCreatorAndMint(creatorPublicKey, mintPublicKey).getOrNull()
```

The returned `AuctionHouse` model will contain details about the Auction House account on chain. In the future, this model will be used to construct an `AuctionHouseClient` instance to interact with the auction and perform trades. 

## Identity
The current identity of a `Metaplex` instance can be accessed via `metaplex.identity()` and provide information on the wallet we are acting on behalf of when interacting with the SDK.

This method returns an identity object with the following interface. All the methods required a solana api instance

```kotlin
interface IdentityDriver {
    val publicKey: PublicKey
    fun sendTransaction(transaction: Transaction, recentBlockHash: String? = null, onComplete: ((Result<String>) -> Unit))
    fun signTransaction(transaction: Transaction, onComplete: (Result<Transaction>) -> Unit)
    fun signAllTransactions(transactions: List<Transaction>, onComplete: (Result<List<Transaction?>>) -> Unit)
}
```

The implementation of these methods depends on the concrete identity driver being used. For example use a KeypairIdentity or a Guest(no publickey added)

Let’s have a quick look at the concrete identity drivers available to us.

### GuestIdentityDriver

The `GuestIdentityDriver` driver is the simplest identity driver. It is essentially a `null` driver that can be useful when we don’t need to send any signed transactions. It will return failure if you use `signTransaction` methods.


### KeypairIdentityDriver

The `KeypairIdentityDriver` driver accepts a `Account` object as a parameter.


### ReadOnlyIdentityDriver

The `KeypairIdentityDriver` driver accepts a `PublicKey` object as a parameter. It's a read only similar to the GUestIdentity, but it has a provided `PublicKey`. It will return failure if you use `signTransaction` methods.

## Storage

You may access the current storage driver using `metaplex.storage()` which will give you access to the following interface.

```kotlin
interface StorageDriver {
    fun download(url: URL, onComplete: (ResultWithCustomError<NetworkingResponse, StorageDriverError>) -> Unit)
}
```

Currently its only used to retrieve json data off-chain. 

### OkHttpSharedStorageDriver

This will use OkHttp networking. Which is the most popular Android networking implementation library. This maybe the most useful implementation.

### MemoryStorageDriver

This will use return Empty Data object with 0 size. 

## Sample app

The SDK comes with a [sample app](https://github.com/metaplex-foundation/metaplex-android/tree/main/sample). Please clone it run it on your phone and take what is can help you. 

[github]: https://github.com/metaplex-foundation/metaplex-android
[sample]: https://github.com/metaplex-foundation/metaplex-android/tree/main/sample


