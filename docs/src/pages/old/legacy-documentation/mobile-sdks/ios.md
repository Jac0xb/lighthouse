---
title: iOS SDK
metaTitle: iOS SDK
description: Metaplex iOS SDK
---

The [Metaplex iOS SDK][docs] is a library that allows you to:

- Load and Deserialize Accounts
- Create transactions
- Run Actions (mint NFT, create an auction, and so on)

It works both in iOS and other Apple platforms that support Swift.

## Stability

[Stability 1 - Experimental](/stability-index)

This project is in active development. **All** interfaces are _very likely_ to change very frequently. Please use caution when making use of this library. Bugs or behavior changes may surprise users when Experimental API modifications occur.

## References

- [API documentation][docs]
- [Source code][github]

## Getting started

### Installation
#### Requirements {#requirements}

- iOS 11.0+ / macOS 10.13+ / tvOS 11.0+ / watchOS 3.0+
- Swift 5.3+

From Xcode 11, you can use [Swift Package Manager](https://swift.org/package-manager/) to add Solana.swift to your project.

- File > Swift Packages > Add Package Dependency
- Add `https://github.com/metaplex-foundation/metaplex-ios`
- Select "branch" with "master"
- Select Metaplex

If you encounter any problems or have a question on adding the package to an Xcode project, I suggest reading the [Adding Package Dependencies to Your App](https://developer.apple.com/documentation/xcode/adding_package_dependencies_to_your_app) guide article from Apple.

### Setup
The entry point to the Swift SDK is a `Metaplex` instance that will give you access to its API.

Set the `SolanaConnectionDriver` and set up your environment. Provide a `StorageDriver` and `IdentityDriver`. You can also use the concrete implementations URLSharedStorageDriver for URLShared and GuestIdentityDriver for a guest Identity Driver. 

You can customise who the SDK should interact on behalf of and which storage provider to use when uploading assets. We might provide a default and simple implementation in the future.

```swift
let solanaDriver = SolanaConnectionDriver(endpoint: RPCEndpoint.mainnetBetaSolana)
let identityDriver = GuestIdentityDriver(solanaRPC: solana.solanaRPC)
let storageDriver = URLSharedStorageDriver(urlSession: URLSession.shared)
let metaplex Metaplex(connection: solana, identityDriver: identityDriver, storageDriver: storageDriver)
```

# Usage
Once properly configured, that `Metaplex` instance can be used to access modules providing different sets of features. Currently, there is only one NFT module that can be accessed via the `nfts()` method. From that module, you will be able to find, create and update NFTs with more features to come.

## NFTs
The NFT module can be accessed via `Metaplex.nfts()` and provide the following methods. Currently, we only support reading methods. Writing and creating NFTs will be supported in the future.

- findNftByMint(mint, callback)
- findNftByMintList(mints, callback)
- findNftsByOwner(owner, callback)
- findNftsByCreator(creator, position = 1, callback)
- findNftsByCandyMachine(candyMachine, version = 2, callback)

All the methods return a callback. It's also possible to wrap them inside either RX, and async Result or Combine. We only provide this interface since is the most compatible without forcing any specific framework. 

### Your first request

The following code snippet is a basic one you can use to get NFTs from a publicKey. This use case maybe very common for a wallet:

```swift
let ownerPublicKey = PublicKey(string: "5LeMDmNW6bQFWQjMhcTZnp6LVHTQQfUpY9jn6YH6RpyE")!
metaplex.nft.findNftsByOwner(publicKey: ownerPublicKey) { [weak self] result in
	switch result {
		case .success(let nftList):
			break
		case .failure:
			break
	}
}
```

This will return an array of NFTs owned by that specific public key.

### The `Nft` model

All the methods above either return or interact with an `Nft` object. The `Nft` object is a read-only data representation of your NFT that contains all the information you need at the top level.

You can see its full data representation by checking the code but here is an overview of the properties that are available on the `Nft` object.

```swift
// Always loaded.
public let metadataAccount: MetadataAccount
    
public let updateAuthority: PublicKey
public let mint: PublicKey
public let name: String
public let symbol: String
public let uri: String
public let sellerFeeBasisPoints: UInt16
public let creators: [MetaplexCreator]
public let primarySaleHappened: Bool
public let isMutable: Bool
public let editionNonce: UInt8?

// Sometimes loaded.
public let masterEditionAccount: MasterEditionAccount?
```

As you can see, some properties are loaded on demand. This is because they are not always needed and/or can be expensive to load.

In order to load these properties, you may run the `metadata` properties of the `Nft` object.

```swift
nft.metadata(metaplex: self.metaplex) { result in
    switch result {
    case .success(let metadata):
        ...
    case .failure:
        ...
    }
}
```

## Identity
The current identity of a `Metaplex` instance can be accessed via `metaplex.identity()` and provide information on the wallet we are acting on behalf of when interacting with the SDK.

This method returns an identity object with the following interface. All the methods required a solana api instance

```swift
public protocol IdentityDriver {
    var publicKey: PublicKey { get }
    func sendTransaction(serializedTransaction: String, onComplete: @escaping(Result<TransactionID, IdentityDriverError>) -> Void)
    func signTransaction(transaction: Transaction, onComplete: @escaping (Result<Transaction, IdentityDriverError>) -> Void)
    func signAllTransactions(transactions: [Transaction], onComplete: @escaping (Result<[Transaction?], IdentityDriverError>) -> Void)
}
```

The implementation of these methods depends on the concrete identity driver being used. For example, using a KeypairIdentity or a Guest (no publickey added).

Let’s have a quick look at the concrete identity drivers available to us.

### GuestIdentityDriver

The `GuestIdentityDriver` driver is the simplest identity driver. It is essentially a `null` driver that can be useful when we don’t need to send any signed transactions. It will return failure if you use `signTransaction` methods.


### KeypairIdentityDriver

The `KeypairIdentityDriver` driver accepts a `Account` object as a parameter.


### ReadOnlyIdentityDriver

The `KeypairIdentityDriver` driver accepts a `PublicKey` object as a parameter. It's a read only similar to the GuestIdentity, but it has a provided `PublicKey`. It will return failure if you use `signTransaction` methods.

## Storage

You may access the current storage driver using `metaplex.storage()` which will give you access to the following interface.

```swift
public protocol StorageDriver {
    func download(url: URL, onComplete: @escaping(Result<NetworkingResponse, StorageDriverError>) -> Void)
}
```

Currently, it's only used to retrieve json data off-chain. 

### URLSharedStorageDriver

This will use URLShared networking. Which is the default iOS networking implementation. This maybe the most useful call.

### MemoryStorageDriver

This will use return Empty Data object with 0 size. 

## Sample app

The SDK comes with a [sample app][sample]. Please clone it run it on your phone and take what is can help you. 

[github]: https://github.com/metaplex-foundation/metaplex-ios
[docs]: https://github.com/metaplex-foundation/metaplex-ios#metaplex-ios-sdk
[sample]: https://github.com/metaplex-foundation/metaplex-ios/tree/main/Sample


