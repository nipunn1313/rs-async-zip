// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::read::io::{compressed::CompressedReader, hashed::HashedReader, owned::OwnedReader};
use crate::spec::compression::Compression;
use crate::entry::ZipEntry;
use crate::error::{Result, ZipError};

use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncReadExt, Take, ReadBuf};
use pin_project::pin_project;
use async_trait::async_trait;

#[pin_project]
pub struct ZipEntryReader<'a, R> where R: AsyncRead + Unpin {
    #[pin]
    reader: HashedReader<CompressedReader<Take<OwnedReader<'a, R>>>>,
}

impl<'a, R> ZipEntryReader<'a, R> where R: AsyncRead + Unpin {
    /// Constructs a new entry reader from its required parameters (incl. an owned R).
    pub(crate) fn new_with_owned(reader: R, compression: Compression, size: u64) -> Self {
        Self { reader: HashedReader::new(CompressedReader::new(OwnedReader::Owned(reader).take(size), compression)) }
    }

    /// Constructs a new entry reader from its required parameters (incl. a mutable borrow of an R).
    pub(crate) fn new_with_borrow(reader: &'a mut R, compression: Compression, size: u64) -> Self {
        Self { reader: HashedReader::new(CompressedReader::new(OwnedReader::Borrow(reader).take(size), compression)) }
    }
}

impl<'a, R> AsyncRead for ZipEntryReader<'a, R> where R: AsyncRead + Unpin {
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut ReadBuf<'_>) -> Poll<tokio::io::Result<()>> {
        self.project().reader.poll_read(c, b)
    }
}

/// A trait that extends [`ZipEntryReader`]'s functionality.
#[async_trait(?Send)]
pub trait ZipEntryReaderExt {
    /// Computes and returns the CRC32 hash of bytes read by this reader so far.
    /// 
    /// This hash should only be computed once EOF has been reached. 
    fn compute_hash(&mut self) -> u32;

    /// Reads all bytes until EOF has been reached, appending them to buf, and verifies the CRC32 values.
    /// 
    /// This is a helper function synonymous to [`AsyncReadExt::read_to_end()`].
    /// 
    /// Equivalent to:
    /// ```no_run
    /// async fn read_to_end_checked(&mut self, buf: &mut Vec<u8>, entry: &ZipEntry) -> Result<usize>
    /// ```
    async fn read_to_end_checked(&mut self, buf: &mut Vec<u8>, entry: &ZipEntry) -> Result<usize>;

    /// Reads all bytes until EOF has been reached, placing them into buf, and verifies the CRC32 values.
    /// 
    /// This is a helper function synonymous to [`AsyncReadExt::read_to_string()`].
    /// 
    /// Equivalent to:
    /// ```no_run
    /// async fn read_to_string_checked(&mut self, buf: &mut String, entry: &ZipEntry) -> Result<usize>
    /// ```
    async fn read_to_string_checked(&mut self, buf: &mut String, entry: &ZipEntry) -> Result<usize>;
}

#[async_trait(?Send)]
impl<'a, R> ZipEntryReaderExt for ZipEntryReader<'a, R> where R: AsyncRead + Unpin {
    fn compute_hash(&mut self) -> u32 {
        self.reader.swap_and_compute_hash()
    }

    async fn read_to_end_checked(&mut self, buf: &mut Vec<u8>, entry: &ZipEntry) -> Result<usize> {
        let read = self.read_to_end(buf).await?;

        if self.compute_hash() == entry.crc32() {
            Ok(read)
        } else {
            Err(ZipError::CRC32CheckError)
        }
    }

    async fn read_to_string_checked(&mut self, buf: &mut String, entry: &ZipEntry) -> Result<usize> {
        let read = self.read_to_string(buf).await?;

        if self.compute_hash() == entry.crc32() {
            Ok(read)
        } else {
            Err(ZipError::CRC32CheckError)
        }
    }
}