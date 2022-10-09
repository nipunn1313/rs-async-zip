// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, ReadBuf};
use pin_project::pin_project;

/// A wrapping reader which holds an owned R or a mutable borrow to R.
/// 
/// This is used to represent whether the supplied reader can be acted on concurrently or not.
#[pin_project(project = OwnedReaderProj)]
pub(crate) enum OwnedReader<'a, R> where R: AsyncRead + Unpin {
    Owned(#[pin] R),
    MutBorrow(#[pin] &'a mut R),
}

impl<'a, R> AsyncRead for OwnedReader<'a, R> where R: AsyncRead + Unpin {
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut ReadBuf<'_>) -> Poll<tokio::io::Result<()>> {
        match self.project() {
            OwnedReaderProj::Owned(inner) => inner.poll_read(c, b),
            OwnedReaderProj::MutBorrow(inner) => inner.poll_read(c, b),
        }
    }
}
