// Copyright (c) 2021-2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! An asynchronous ZIP archive reading/writing crate with a heavy focus on streaming support.
//!
//! ## Features
//! - Asynchronous design powered by tokio.
//! - Support for Stored, Deflate, bzip2, LZMA, zstd, and xz compression methods.
//! - Various different reading approaches (seek, stream, filesystem, in-memory buffer).
//! - Support for writing complete data (u8 slices) or stream writing using data descriptors.
//! - Aims for reasonable [specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) compliance.
//!
//! [Read more.](https://github.com/Majored/rs-async-zip)

pub mod error;
pub mod read;
pub(crate) mod spec;
pub(crate) mod entry;
pub mod write;

pub use crate::spec::compression::Compression;
pub use crate::spec::attribute::AttributeCompatibility;

pub use crate::entry::{ZipEntry, builder::ZipEntryBuilder};
pub use crate::entry::ext::{ZipEntryExt, ZipEntryBuilderExt};
pub use crate::read::io::entry::{ZipEntryReader, ZipEntryReaderExt};