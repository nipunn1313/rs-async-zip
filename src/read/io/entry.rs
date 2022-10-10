// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::read::io::{compressed::CompressedReader, hashed::HashedReader, owned::OwnedReader};
use crate::spec::compression::Compression;

use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncReadExt, Take, ReadBuf};
use pin_project::pin_project;

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

impl<'a, R> ZipEntryReaderExt for ZipEntryReader<'a, R> where R: AsyncRead + Unpin {

}

/// A trait that extends [`ZipEntryReaderExt`]'s functionality.
pub trait ZipEntryReaderExt {

}