// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::read::io::{compressed::CompressedReader, hashed::HashedReader, owned::OwnedReader};
use crate::spec::compression::Compression;
use crate::error::Result;
use crate::file::ZipFile;

use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncReadExt, Take, ReadBuf};
use pin_project::pin_project;

pub struct ZipFileReader<R> where R: AsyncRead + Unpin {
    reader: R,
    file: ZipFile,
}

impl<R> ZipFileReader<R> where R: AsyncRead + Unpin {
    pub async fn new(mut reader: R) -> Result<ZipFileReader<R>> {
        let (entries, metas) = crate::read::read_cd((&mut reader).take(0), 0).await?;
        let comment = String::new();

        let file = ZipFile { entries, metas, comment };
        Ok(ZipFileReader { reader, file })
    }
}