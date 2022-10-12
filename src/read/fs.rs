// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A concurrent reader which acts on a file system path.
//! 
//! Concurrency is achieved as a result of:
//! - Wrapping the provided path within an [`Arc`] to allow shared ownership.
//! - Constructing a new [`File`] from the path when reading.
//! 
//! ### Usage
//! Unlike the [`seek`] module, we no longer hold a mutable reference to any inner reader which in turn, allows the
//! construction of concurrent [`ZipEntryReader`]s. Though, note that each individual [`ZipEntryReader`] cannot be sent
//! between thread boundaries due to the masked lifetime requirement. Therefore, the overarching [`ZipFileReader`]
//! should be cloned and moved into those contexts when needed.
//! 
//! ### Concurrent Example
//! ```
//! # use async_zip::read::fs::ZipFileReader;
//! # use async_zip::error::Result;
//! #
//! # async fn run() -> Result<()> {
//! let reader = ZipFileReader::new("./foo.zip").await?;
//! 
//! let fut_gen = |index| { async {
//!     let mut entry_reader = local_reader.entry_reader(index).await?;
//!     let mut data = Vec::new();
//!     entry_reader.read_to_end(&mut data).await?;
//! }};
//! 
//! tokio::join!(fut_gen(0), fut_gen(1)).map(|res| res?);
//! #   Ok(())
//! # }
//! ```
//! 
//! ### Parallel Example
//! ```
//! # use async_zip::read::fs::ZipFileReader;
//! # use async_zip::error::Result;
//! #
//! # async fn run() -> Result<()> {
//! let reader = ZipFileReader::new("./foo.zip").await?;
//! 
//! let fut_gen |index| {
//!     let local_reader = reader.clone();
//! 
//!     tokio::spawn(async move {
//!         let mut entry_reader = local_reader.entry_reader(index).await?;
//!         let mut data = Vec::new();
//!         entry_reader.read_to_end(&mut data).await.unwrap();
//!     })
//! };
//! 
//! tokio::join!(fut_gen(0), fut_gen(1)).map(|res| res?);
//! #   Ok(())
//! # }
//! ```

#[cfg(doc)]
use crate::read::seek;

use crate::read::io::entry::ZipEntryReader;
use crate::file::ZipFile;
use crate::spec::compression::Compression;
use crate::error::Result;

use std::sync::Arc;
use std::path::{Path, PathBuf};

use tokio::fs::File;

struct Inner {
    path: PathBuf,
    file: ZipFile,
}

/// A concurrent reader which acts over an owned vector of bytes.
#[derive(Clone)]
pub struct ZipFileReader {
    inner: Arc<Inner>,
}

impl ZipFileReader {
    pub async fn new<P>(path: P) -> Result<ZipFileReader> where P: AsRef<Path> {
        let path = path.as_ref().to_owned();
        let file = crate::read::file(File::open(&path).await?).await?;
        
        Ok(ZipFileReader { inner: Arc::new(Inner { path, file }) })
    }

    pub async fn entry_reader(&self, index: usize) -> Result<ZipEntryReader<File>> {
        let entry = self.inner.file.entries.get(index).unwrap();
        let meta = self.inner.file.metas.get(index).unwrap();

        let fs_file = File::open(&self.inner.path).await?;
        Ok(ZipEntryReader::new_with_owned(fs_file, Compression::Deflate, 0))
    }
}