---
title: Bring Your Own Uploader
metaTitle: Candy Machine - Sugar - Bring Your Own Uploader
description: How to bring your own uploader to Sugar.
---

Sugar has an extensible architecture to easily allow the implementation of new upload methods with minimal effort. The upload logic is decoupled from the `upload` command and new methods can be plug-in into the upload flow by implementing a Rust trait, supporting both *free-form* and parallel upload methods:

- `Uploader`: this trait should be implemented directly by upload methods that require full control on how the upload is performed.
- `ParallelUploader`: this trait abstracts the threading logic and allows methods to focus on the logic of uploading a single asset (file).

The use of the different traits is illustrated in the upload architecture overview below:

![Uploader architecture](https://docs.metaplex.com/assets/images/UploaderOverview-c455b1d9ac30e0c664d77d49e5935b3d.png#radius#shadow)

To implement your uploader, the first step is to decide whether you need full control of the upload process or your method support parallel upload. This will inform which trait to implement. Independently of the trait that you implement, assets (files) requiring upload are represented by a `AssetInfo` struct:

```rust
pub struct AssetInfo {
    /// Id of the asset in the cache.
    pub asset_id: String,
    /// Name (file name) of the asset.
    pub name: String,
    /// Content of the asset - either a file path or the string
    /// representation of its content.
    pub content: String,
    /// Type of the asset.
    pub data_type: DataType,
    /// MIME content type.
    pub content_type: String,
}
```

An `AssetInfo` can represent a physical file, in which case the `content` will correspond to the name of the file; or an in-memory asset, in which case the `content` will correspond to the content of the asset.

For example, for image files, the `content` contains the path of the file on the file system. In the case of json metadata files, the `content` contains the string representation of the json metadata.

## Traits

> More details of the traits' implementations can be found on Sugar's [source code](https://github.com/metaplex-foundation/sugar/blob/main/src/upload/uploader.rs).

### Uploader

The `Uploader` trait gives you full control on how the assets (files) are uploaded. It defines a single function:

```rust
async fn upload(
    &self,
    sugar_config: &SugarConfig,
    cache: &mut Cache,
    data_type: DataType,
    assets: &mut Vec<AssetInfo>,
    progress: &ProgressBar,
    interrupted: Arc<AtomicBool>,
) -> Result<Vec<UploadError>>;
```

where:

* `sugar_config` - The current sugar configuration
* `cache` - Asset cache object (mutable)
* `data_type` - Type of the asset being uploaded
* `assets` - Vector of assets to upload (mutable)
* `progress` - Reference to the progress bar to provide feedback to the console
* `interrupted` - Reference to the shared interruption handler flag to receive notifications

This function will be called to upload each type of asset separately&mdash;e.g., once for your images, once for your metadata and, if present, once for your animation assets. After uploading an asset, its information needs to be updated in the `cache` object and the cache saved to the file system using the `sync_file` function. Syncing the cache to the file system might be slow for large collections, therefore it should be done as frequent as practical to avoid slowing down the upload process and, at the same time, minimizing the chances of information loss in case the user aborts the upload.

Implementations are expected to use the `interrupted` parameter to control when the user aborts the upload process by pressing `Ctrl+C`&mdash;this is useful for large uploads. Any information saved in the cache will not be re-uploaded. The `upload` command will filter out the assets already uploaded, and they will not be included in the vector of assets. The `progress` is a reference to the progress bar displayed on the console and should be used to provide a visual feedback of the progress of the upload by calling its `progress.inc(1)` function to indicate that `1` asset was uploaded.

When all files are uploaded successfully, the `upload` method will return an empty `Vec`; in case of errors, the `Vec` will contain a list of `UploadError`s that will be displayed to the user.

### ParallelUploader

The `ParallelUpload` provides a thread-enabled implementation of the `Uploader` trait's `upload` function to support concurrent uploads, abstracting the threading logic to focus on the logic of uploading a single asset (file). Therefore, methods that can upload assets in parallel need to implement a simplified `upload_asset` function:

```rust
fn upload_asset(
    &self,
    asset: AssetInfo
) -> JoinHandle<Result<(String, String)>>;
```

The `upload_asset` function must return a `JoinHandle` object. In most cases, the function will return the value from `tokio::spawn`. This function should only include the logic to upload the asset&mdash;the interruption control and cache synchronization is done automatically by the `ParallelUpload` trait.

### Prepare

All upload methods need to implement an additional trait `Prepare`. The rationale is to prepare the method for the upload of the specified media/metadata files, e.g.:
- check if any file exceeds a size limit;
- check if there is storage space for the upload;
- check/add funds for the upload.

The trait defines a single function:

```rust
async fn prepare(
    &self,
    sugar_config: &SugarConfig,
    asset_pairs: &HashMap<isize, AssetPair>,
    asset_indices: Vec<(DataType, &[isize])>,
) -> Result<()>;
```
where:
* `sugar_config` - The current sugar configuration
* `asset_pairs` - Mapping of `index` to an `AssetPair`
* `asset_indices` - Vector with the information of which asset pair indices will be uploaded, grouped by type.

The `asset_pairs` contain the complete information of the assets, but only the assets specified in the `asset_indices` will be uploaded&mdash;e.g., if index `1` is only present in the `DataType::Image` indices' array, only the image of asset `1` will the uploaded.

## Configuration

After implementing the logic of the upload method, you need to integrate your method in Sugar's configuration file. Firstly, you will need to add a new value to the `UploadMethod` [enum](https://github.com/metaplex-foundation/sugar/blob/main/src/config/data.rs#L220-L231) to identify your upload method. Secondly, you need to modify the `initialize` [factory method](https://github.com/metaplex-foundation/sugar/blob/main/src/upload/uploader.rs#L274-L296) to create the `Uploader` object when it is specified in the configuration file.

In case your upload method requires additional parameters, you will need to modify the `ConfigData` [struct](https://github.com/metaplex-foundation/sugar/blob/main/src/config/data.rs#L30-L86). For example, the `aws` upload method requires the user to specify a bucket name for the upload. In the `ConfigData` struct, you will find an `aws_s3_bucket` field, which corresponds to the `awsS3Bucket` property in the configuration file.

Once you completed the upload method trait implementation and added its details to Sugar's configuration file, it is ready to be used to upload assets.

{% callout %}

Do not forget to submit a PR to Sugar's repository to have your implementation added to Sugar's code base.

{% /callout %}

## Next steps

Sugar currently has six [upload methods](https://github.com/metaplex-foundation/sugar/tree/main/src/upload/methods) available&mdash;check their source code for more details about how the upload of assets works and design ideas to implement your own upload method.
