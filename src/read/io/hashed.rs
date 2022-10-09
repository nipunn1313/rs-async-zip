// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::read::io::poll_result_ok;

use std::pin::Pin;
use std::task::{Context, Poll, ready};

use tokio::io::{AsyncRead, ReadBuf};
use crc32fast::Hasher;
use pin_project::pin_project;

/// A wrapping reader which computes the CRC32 hash of data read via [`AsyncRead`].
#[pin_project]
pub(crate) struct HashedReader<R> where R: AsyncRead + Unpin {
    #[pin]
    pub(crate) reader: R,
    pub(crate) hasher: Hasher,
}

impl<R> HashedReader<R> where R: AsyncRead + Unpin {
    /// Constructs a new wrapping reader from a generic [`AsyncRead`] implementer.
    pub(crate) fn new(reader: R) -> Self {
        Self { reader, hasher: Hasher::default() }
    }

    /// Consumes this reader and returns the computed CRC32 hash.
    /// 
    /// This method is consuming as the internal hasher also requires consuming in order to compute the hash. See the
    /// non-consuming counterpart, swap_and_compute_hash(), as an alternative.
    pub(crate) fn compute_hash(self) -> u32 {
        self.hasher.finalize()
    }

    /// Swaps the internal hasher and returns the computed CRC32 hash.
    /// 
    /// The internal hasher is taken and replaced with a newly-constructed one. As a result, this method should only be
    /// called once EOF has been reached and it's known that no more data will be read, else the computed hash(s) won't
    /// accurately represent the data read in.
    pub(crate) fn swap_and_compute_hash(&mut self) -> u32 {
        std::mem::take(&mut self.hasher).finalize()
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for HashedReader <R> {
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut ReadBuf<'_>) -> Poll<tokio::io::Result<()>> {
        let project = self.project();
        let prev_len = b.filled().len();

        poll_result_ok!(ready!(project.reader.poll_read(c, b)));
        project.hasher.update(&b.filled()[prev_len..b.filled().len()]);

        Poll::Ready(Ok(()))
    }
}

#[tokio::test]
async fn hashed_reader_test() {
    use std::io::Cursor;
    use tokio::io::AsyncReadExt;

    let data = "foo bar";
    let data_crc32 = 0xbe460134u32;

    let cursor = Cursor::new(data.as_bytes());
    let mut reader = HashedReader::new(cursor);
    
    let mut read_data = String::new();
    reader.read_to_string(&mut read_data).await.expect("read into HashedReader failed");

    assert_eq!(data, read_data);
    assert_eq!(data_crc32, reader.compute_hash());
}