// Copyright (c) 2021-2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! An asynchronous ZIP archive reading/writing crate.
//!
//! ## Features
//! - Asynchronous design powered by tokio.
//! - Support for Stored, Deflate, bzip2, LZMA, zstd, and xz compression methods.
//! - Various different reading approaches (seek, stream, filesystem, in-memory buffer).
//! - Support for writing complete data (u8 slices) or stream writing using data descriptors.
//! - Aims for reasonable [specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) compliance.
//!
//! ## Installation & Basic Usage
//! ```toml
//! [dependencies]
//! async_zip = { version = "0.1.0", features = ["full"] }
//! ```
//! 
//! ## Feature Flags
//! - `date` - Enables support for parsing dates/times stored in ZIPs via [`chrono`].
//! - `fs` - Enables support for the [`fs`] reading module.
//! - `deflate` - Enables support for the Deflate compression method.
//! - `bzip2` - Enables support for the Bzip2 compression method.
//! - `lzma` - Enables support for the Lzma compression method.
//! - `zstd` - Enables support for the zstd compression method.
//! - `xz` - Enables support for the xz compression method.
//! 
//! [Read more.](https://github.com/Majored/rs-async-zip)

pub mod error;
pub mod read;
pub(crate) mod spec;
pub(crate) mod entry;
pub(crate) mod file;
pub mod write;

#[cfg(doc)]
use crate::read::fs;

pub use crate::spec::compression::Compression;
pub use crate::spec::attribute::AttributeCompatibility;

pub use crate::entry::{ZipEntry, builder::ZipEntryBuilder};
pub use crate::entry::ext::{ZipEntryExt, ZipEntryBuilderExt};
pub use crate::file::{ZipFile, builder::ZipFileBuilder};
pub use crate::file::ext::{ZipFileExt, ZipFileBuilderExt};
pub use crate::read::io::entry::{ZipEntryReader, ZipEntryReaderExt};