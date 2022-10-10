// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A concurrent reader which acts over an owned vector of bytes.
//! 
//! Concurrency is achieved as a result of:
//! - Wrapping the provided vector of bytes within an [`Arc`] to allow shared ownership.
//! - Wrapping this [`Arc`] around a [`Cursor`] when reading (as the [`Arc`] can deref and coerce into a `&[u8]`).
//! 
//! ### Usage
//! An individual [`ZipEntryReader`] cannot be sent between thread boundaries as they hold a reference to the byte
//! slice (via the [`Arc`] pointer indirection).
//! 
//! ### Example
//! ```
//! use async_zip::read::mem::ZipFileReader;
//! 
//! # async fn run() -> Result<(), ZipError> {
//! let data = Vec::new();
//! let reader = ZipFileReader::new(data);
//! 
//! let mut local_reader = reader.clone();
//! let fut1 = async move {
//!     let mut entry_reader = local_reader.entry_reader(1).await.unwrap();
//!     let mut data = Vec::new();
//!     entry_reader.read_to_end(&mut data).await.unwrap();
//! };
//! 
//! let mut local_reader = reader.clone();
//! let fut2 = async move {
//!     let entry_reader = local_reader.entry_reader(1).await.unwrap();
//!     let mut data = Vec::new();
//!     entry_reader.read_to_end(&mut data).await.unwrap();
//! };
//! 
//!     tokio::join!(fut1, fut2);
//! #   Ok(())
//! # }
//! ```

use crate::read::io::entry::ZipEntryReader;
use crate::file::ZipFile;
use crate::spec::compression::Compression;
use crate::error::Result;

use std::sync::Arc;
use std::io::Cursor;

struct Inner {
    data: Vec<u8>,
    file: ZipFile,
}

/// A concurrent reader which acts over an owned vector of bytes.
#[derive(Clone)]
pub struct ZipFileReader {
    inner: Arc<Inner>,
}

impl ZipFileReader {
    pub async fn entry_reader(&self, index: usize) -> Result<ZipEntryReader<Cursor<&[u8]>>> {
        let entry = self.inner.file.entries.get(index).unwrap();
        let meta = self.inner.file.metas.get(index).unwrap();

        let cursor = Cursor::new(&self.inner.data[..]);
        Ok(ZipEntryReader::new_with_owned(cursor, Compression::Deflate, 0))
    }
}