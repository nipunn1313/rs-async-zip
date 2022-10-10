// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::spec::compression::Compression;

use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(any(feature = "deflate", feature = "bzip2", feature = "zstd", feature = "lzma", feature = "xz"))]
use async_compression::tokio::bufread;
use tokio::io::{AsyncRead, ReadBuf, BufReader};
use pin_project::pin_project;

/// A wrapping reader which holds concrete types for all respective compression method readers.
#[pin_project(project = CompressedReaderProj)]
pub(crate) enum CompressedReader<R> where R: AsyncRead + Unpin {
    Stored(#[pin] R),
    #[cfg(feature = "deflate")]
    Deflate(#[pin] bufread::DeflateDecoder<BufReader<R>>),
    #[cfg(feature = "bzip2")]
    Bz(#[pin] bufread::BzDecoder<BufReader<R>>),
    #[cfg(feature = "lzma")]
    Lzma(#[pin] bufread::LzmaDecoder<BufReader<R>>),
    #[cfg(feature = "zstd")]
    Zstd(#[pin] bufread::ZstdDecoder<BufReader<R>>),
    #[cfg(feature = "xz")]
    Xz(#[pin] bufread::XzDecoder<BufReader<R>>),
}

impl<R> CompressedReader<R> where R: AsyncRead + Unpin {
    /// Constructs a new wrapping reader from a generic [`AsyncRead`] implementer.
    pub(crate) fn new(reader: R, compression: Compression) -> Self {
        match compression {
            Compression::Stored => CompressedReader::Stored(reader),
            #[cfg(feature = "deflate")]
            Compression::Deflate => CompressedReader::Deflate(bufread::DeflateDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "bzip2")]
            Compression::Bz => CompressedReader::Bz(bufread::BzDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "lzma")]
            Compression::Lzma => CompressedReader::Lzma(bufread::LzmaDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "zstd")]
            Compression::Zstd => CompressedReader::Zstd(bufread::ZstdDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "xz")]
            Compression::Xz => CompressedReader::Xz(bufread::XzDecoder::new(BufReader::new(reader))),
        }
    }
}

impl<R> AsyncRead for CompressedReader<R> where R: AsyncRead + Unpin {
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut ReadBuf<'_>) -> Poll<tokio::io::Result<()>> {
        match self.project() {
            CompressedReaderProj::Stored(inner) => inner.poll_read(c, b),
            #[cfg(feature = "deflate")]
            CompressedReaderProj::Deflate(inner) => inner.poll_read(c, b),
            #[cfg(feature = "bzip2")]
            CompressedReaderProj::Bz(inner) => inner.poll_read(c, b),
            #[cfg(feature = "lzma")]
            CompressedReaderProj::Lzma(inner) => inner.poll_read(c, b),
            #[cfg(feature = "zstd")]
            CompressedReaderProj::Zstd(inner) => inner.poll_read(c, b),
            #[cfg(feature = "xz")]
            CompressedReaderProj::Xz(inner) => inner.poll_read(c, b),
        }
    }
}

compressed_test_helper!(stored_test, Compression::Stored, "foo bar", "foo bar");

compressed_test_helper!(
    deflate_test, Compression::Deflate, "foo bar", [0x4b, 0xcb, 0xcf, 0x57, 0x48, 0x4a, 0x2c, 0x02, 0x00]
);

compressed_test_helper!(
    bz_test, Compression::Bz, "foo bar", [
        0x42, 0x5a, 0x68, 0x36, 0x31, 0x41, 0x59, 0x26, 0x53, 0x59, 0xe4, 0x33, 0xbe, 0x6e, 0x00, 0x00, 0x01, 0x91, 0x80,
        0x40, 0x00, 0x31, 0x00, 0x90, 0x00, 0x20, 0x00, 0x22, 0x18, 0x68, 0x30, 0x0b, 0x19, 0x03, 0x0b, 0xb9, 0x22, 0x9c,
        0x28, 0x48, 0x72, 0x19, 0xdf, 0x37, 0x00
    ]
);

compressed_test_helper!(
    lzma_test, Compression::Lzma, "foo bar", [
        0x5d, 0x00, 0x00, 0x80, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x33, 0x1b, 0xec, 0x40, 0x62,
        0x31, 0x20, 0x25, 0x2f, 0xf9, 0xff, 0xff, 0xed, 0x88, 0x80, 0x00
    ]
);

compressed_test_helper!(
    zstd_test, Compression::Zstd, "foo bar", [
        0x28, 0xb5, 0x2f, 0xfd, 0x00, 0x58, 0x39, 0x00, 0x00, 0x66, 0x6f, 0x6f, 0x20, 0x62, 0x61, 0x72
    ]
);

compressed_test_helper!(
    xz_test, Compression::Xz, "foo bar", [
        0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00, 0x00, 0x04, 0xe6, 0xd6, 0xb4, 0x46, 0x02, 0x00, 0x21, 0x01, 0x16, 0x00,
        0x00, 0x00, 0x74, 0x2f, 0xe5, 0xa3, 0x01, 0x00, 0x06, 0x66, 0x6f, 0x6f, 0x20, 0x62, 0x61, 0x72, 0x00, 0x00,
        0x22, 0x50, 0xf4, 0xe4, 0x02, 0x84, 0xa8, 0x09, 0x00, 0x01, 0x1f, 0x07, 0x16, 0x2e, 0xb8, 0x73, 0x1f, 0xb6,
        0xf3, 0x7d, 0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x59, 0x5a
    ]
);

/// A helper macro for generating a CompressedReader test using a specific compression method.
macro_rules! compressed_test_helper {
    ($name:ident, $typ:expr, $data_raw:expr, $data:expr) => {
        #[cfg(test)]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::AsyncReadExt;
        
            let data = $data;
            let data_raw = $data_raw;
        
            let cursor = Cursor::new(data);
            let mut reader = CompressedReader::new(cursor, $typ);

            let mut read_data = String::new();
            reader.read_to_string(&mut read_data).await.expect("read into CompressedReader failed");
        
            assert_eq!(read_data, data_raw);
        }
    };
}

use compressed_test_helper;
