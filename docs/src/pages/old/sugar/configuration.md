---
title: Configuration File
metaTitle: Candy Machine - Sugar - Configuration File
description: A detailed overview of Sugar configuration file.
---

Sugar uses a JSON configuration file to upload assets and configure a Candy Machine – in most cases, the file will be named `config.json`. The configuration includes the settings that are used to initialize and update the Candy Machine, as well as upload the assets to be minted. It will also include the configuration of guards that will provide access control to minting.

A basic configuration file is shown below:

```json
{
  "tokenStandard": "pnft",
  "number": 10,
  "symbol": "TEST",
  "sellerFeeBasisPoints": 500,
  "isMutable": true,
  "isSequential": false,
  "ruleSet": "eBJLFYPxJmMGKuFwpDWkzxZeUrad92kZRC5BJLpzyT9",
  "creators": [
    {
      "address": "PanbgtcTiZ2PveV96t2FHSffiLHXXjMuhvoabUUKKm8",
      "share": 50
    },
    {
      "address": "PanbgtcTiZ2PveV96t2FHSffiLHXXjMuhvoabUUKKm8",
      "share": 50
    }
  ],
  "hiddenSettings": null,
  "uploadMethod": "bundlr",
  "awsConfig": null,
  "nftStorageAuthToken": null,
  "shdwStorageAccount": null,
  "pinataConfig": null,
  "sdriveApiKey": null,
  "guards": {
    "default": {
      "botTax": {
        "value": 0.01,
        "lastInstruction": true
      }
    },
    "groups": [
      {
        "label": "OGs",
        "guards": {
          "startDate": {
            "date": "2022-10-20 12:00:00 +0000"
          },
          "tokenGate": {
            "amount": 1,
            "mint": "7nE1GmnMmDKiycFkpHF7mKtxt356FQzVonZqBWsTWZNf"
          },
          "solPayment": {
            "value": 1,
            "destination": "PanbgtcTiZ2PveV96t2FHSffiLHXXjMuhvoabUUKKm8"
          }
        }
      },
      {
        "label": "Public",
        "guards": {
          "startDate": {
            "date": "2022-10-20 18:00:00 +0000"
          },
          "solPayment": {
            "value": 2,
            "destination": "PanbgtcTiZ2PveV96t2FHSffiLHXXjMuhvoabUUKKm8"
          }
        }
      }
    ]
  }
}
```

The configuration file can be viewed as having three main parts: Candy Machine settings (`"tokenStandard"` to `"hiddenSettings"`), upload settings (`"uploadMethod"` to `"sdriveApiKey"`) and guard settings (`"guards"`).

## Candy Machine settings

Candy Machine settings determine the type of the asset, number of assets available and their metadata information.

| Setting | Options | Value/Type | Description               |
| ------- | ------- | --------------- | ------------------------- |
| tokenStandard |   |                 |                           |
|         |         | "nft"           | Non-Fungible asset (`NFT`)        |
|         |         | "pnft"           | Programmable Non-Fungible asset (`pNFT`) |
| number  |         | Integer         | Number of available items |
| symbol  |         | String          | String representing the symbol of the NFT |
| sellerFeeBasisPoint  |         | Integer          | The royalties shared by the creators in basis points (i.e., 550 means 5.5%)  |
| isMutable |       | Boolean         | A boolean indicating if the NFTs Metadata Account can be updated |
| isSequential |    | Boolean         | A boolean indicating whether a sequential index generation should be used during mint or not |
| ruleSet  |        | Public Key | *(optional)* The rule set used by minted `pNFT`s |

The `creators` setting allows you to specify up to 4 addresses and their percentage share. 

| Setting | Options | Value/Type | Description               |
| ------- | ------- | --------------- | ------------------------- |
| creators |        | Up to 4 creators | List of creators and their percentage share of the royalties |
|          | address | Public Key | A creator public key |
|          | share | Integer | A value between `0` and `100` |

{% callout %}

While the metadata on-chain stores up to 5 creators, the Candy Machine is set as one of the creators. As a result, there is a limit of 4 creators at most.

The sum of the share values must add up to 100, otherwise an error will be generated.

{% /callout %}

The Candy Machine can be configured to not have the final metadata when an NFT is minted. This is useful when you are planning a revel step once mint is completed. In this case, you can specify the "placeholder" metadata values for the *hidden* NFT:

| Setting | Options | Value/Type | Description               |
| ------- | ------- | --------------- | ------------------------- |
| hiddenSettings | | | |
| | name | String | Name of the mint (can use `$ID$` or `$ID+1$` mint index replacement variables) |
| | uri | String | URI of the mint (can use `$ID$` or `$ID+1$` mint index replacement variables) |
| | hash | String | A 32 character hash (in most cases this is the hash of the cache file with the mapping between mint number and metadata so that the order can be verified when the mint is complete)

## Upload settings

Sugar supports a variety of storage providers – the one to be used is define by the `uploadMethod` setting. Depending of the provider, there would be additional configuration needed.

The table below provides an overview of the settings available:

| Setting | Options | Accepted Values | Description               |
| ------- | ------- | --------------- | ------------------------- |
| uploadMethod |   |  | Configure the storage to upload images and metadata |  
|  |   | "bundlr" |  Uploads to Arweave using [Bundlr](https://bundlr.network) and payments are made in SOL (works on both mainnet and devnet; files are only stored for 7 days on devnet)
|  |   | "aws" | Uploads to Amazon Web Services (AWS) |
|  |   | "nftStorage" | Uploads to [NFT.Storage](https://nft.storage) (works on all networks; no payment required) |
|  |   | "shdw" | Uploads to the GenesysGo [Shadow Drive](https://docs.shadow.cloud) (works on mainnet only)
|  |   | "pinata" | Uploads to [Pinata](https://www.pinata.cloud) (works on all networks; free and tiered subscriptions) |
|  |   | "sdrive" | Uploads to Shador Drive using [SDrive Cloud Storage](https://sdrive.app) |
|awsConfig | | | *(required when "aws" is used)* |
| | bucket | String | AWS bucket name
| | profile | String | AWS profile to use from the credentials file name |
| | directory | String | The directory within the bucket to upload the items to. An empty string means uploading files to the bucket root directory. |
| nftStorageAuthToken | | String | NFT.Storage API Key *(required when "nftStorage" is used)* |
| pinataConfig | | | *(required when "pinata" is used)* |
| | JWT | String | JWT authentication token |
| | apiGateway | String | URL to connect to Pinata API |
| | apiContent | String | URL to use as the base for creating the asset links |
| | parallelLimit | Integer | Number of concurrent uploads; use this setting to avoid rate limits |
| shadowStorageAccount | | String | Shadow Drive storage pubkey *(required when "shdw" is used)* |
| sdriveApiKey | | String | SDrive API key *(required when "sdrive" is used)* |

Specific upload method settings:

{% totem %}
{% totem-accordion title="Bundlr" %}

The `"bundlr"` upload method does not require extra configuration. Any fee associated with the upload will be payed in `SOL` using the configured keypair.

{% /totem-accordion %}
{% totem-accordion title="AWS" %}

The `"aws"` method uploads files to Amazon S3 storage. This requires additional configuration, you need to specify the `bucket`, `profile`, `directory` and `domain` values in the configuration file under `"awsConfig"` and set up the credentials in your system. In most cases, this will involve creating a file `~/.aws/credentials` with the following properties:

```
[default]
aws_access_key_id=<ACCESS KEY ID>
aws_secret_access_key=<SECRET ACCESS KEY>
region=<REGION>
```

It is also important to set up the ACL permission of the bucket correctly to enable `"public-read"`` and apply Cross-Origin Resource Sharing (CORS) rules to enable content access requested from a different origin (necessary to enable wallets and blockchain explorers load the metadata/media files). More information about these configurations can be found at:

* [Bucket policy examples](https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-bucket-policies.html)
* [CORS configuration](https://aws.amazon.com/premiumsupport/knowledge-center/s3-configure-cors/)

The `profile` value allows you to specify which profile to read from your credentials file. The `directory` value is the name of the directory in the bucket where the files will be uploaded, allowing you to have multiple candy machine or collections in a single bucket separated by different directories. Leaving this as an empty string will upload the files to the root of the bucket. The (optional) `domain` allows you to specify a custom domain to serve the data from AWS — e.g., using the domain as `https://mydomain.com` will create links to files in the format `https://mydomain.com/0.json`. If you do not specify a domain, the default AWS S3 domain will be used (`https://<BUCKET_NAME>.s3.amazonaws.com`).

{% /totem-accordion %}
{% totem-accordion title="NFT.Storage" %}

NFT.Storage is a popular free service that uploads data on the public IPFS network. You will need to register an account to obtain an API key (token), which need to be specified by `"nftStorageAuthToken"` in the configuration file.

{% /totem-accordion %}
{% totem-accordion title="Pinata" %}

The `"pinata"` method uploads files to Pinata storage. You need to specify the `jwt`, `apiGateway`, `contentGateway` and `parallelLimit` values in the configuration file under `"pinataConfig"`:

* `jwt``: JWT authentication token
* `apiGateway`: URL to connect to Pinata API (use `https://api.pinata.cloud` for the public API endpoint)
* `contentGateway`: URL to use as the base for creating the asset links (use `https://gateway.pinata.cloud` for the public gateway)
* `parallelLimit`: (optional) number of concurrent upload, adjust this value to avoid rate limits

{% callout %}

The public gateways are not intended to be used in production — they are good to be used for testing as they are heavily rate limited and not designed for speed.

{% /callout %}

{% /totem-accordion %}
{% totem-accordion title="Shadow Drive" %}

Shadow Drive is a decentralized storage network built specifically for the Solana blockchain. In order to upload data to the Shadow Drive you will need to first create a storage account. This can be done using the [Shadow Drive CLI](https://docs.shadow.cloud/build). After creating a storage account, specify its pubkey address in the configuration file using the property `"shdwStorageAccount"`.

{% callout %}

The Shadow Drive upload method is only available on `mainnet-beta`.

{% /callout %}

{% /totem-accordion %}
{% totem-accordion title="SDrive" %}

SDrive is a storage app built on top of GenesysGo Shadow Drive. You will need to register an account to obtain an API key (token), which need to be specified by `"sdriveApiKey"` in the configuration file.

{% /totem-accordion %}
{% /totem %}

## Guard settings

The `guards` settings allows you to specify which guards will be enabled on the Candy Machine.

Candy Machine support a number of guards that provide access control to minting. [Guards](#/candy-machine/guards) can be configured into a "default" [guard group](#/candy-machine/guard-groups) or appear in multiple guard groups.
