// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::spec::compression::Compression;

use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(any(feature = "deflate", feature = "bzip2", feature = "zstd", feature = "lzma", feature = "xz"))]
use async_compression::tokio::bufread;
use tokio::io::{AsyncRead, ReadBuf, BufReader};
use pin_project::pin_project;

#[pin_project(project = CompressionReaderProj)]
pub(crate) enum CompressionReader<R> where R: AsyncRead + Unpin {
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

impl<R> CompressionReader<R> where R: AsyncRead + Unpin {
    pub(crate) fn new(reader: R, compression: Compression) -> Self {
        match compression {
            Compression::Stored => CompressionReader::Stored(reader),
            #[cfg(feature = "deflate")]
            Compression::Deflate => CompressionReader::Deflate(bufread::DeflateDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "bzip2")]
            Compression::Bz => CompressionReader::Bz(bufread::BzDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "lzma")]
            Compression::Lzma => CompressionReader::Lzma(bufread::LzmaDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "zstd")]
            Compression::Zstd => CompressionReader::Zstd(bufread::ZstdDecoder::new(BufReader::new(reader))),
            #[cfg(feature = "xz")]
            Compression::Xz => CompressionReader::Xz(bufread::XzDecoder::new(BufReader::new(reader))),
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for CompressionReader<R> {
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut ReadBuf<'_>) -> Poll<tokio::io::Result<()>> {
        match self.project() {
            CompressionReaderProj::Stored(inner) => inner.poll_read(c, b),
            #[cfg(feature = "deflate")]
            CompressionReaderProj::Deflate(inner) => inner.poll_read(c, b),
            #[cfg(feature = "bzip2")]
            CompressionReaderProj::Bz(inner) => inner.poll_read(c, b),
            #[cfg(feature = "lzma")]
            CompressionReaderProj::Lzma(inner) => inner.poll_read(c, b),
            #[cfg(feature = "zstd")]
            CompressionReaderProj::Zstd(inner) => inner.poll_read(c, b),
            #[cfg(feature = "xz")]
            CompressionReaderProj::Xz(inner) => inner.poll_read(c, b),
        }
    }
}
