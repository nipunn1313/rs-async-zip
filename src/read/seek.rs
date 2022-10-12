// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::error::Result;
use crate::file::ZipFile;

use tokio::io::{AsyncRead, AsyncSeek};

pub struct ZipFileReader<R> where R: AsyncRead + AsyncSeek + Unpin {
    reader: R,
    file: ZipFile,
}

impl<R> ZipFileReader<R> where R: AsyncRead + AsyncSeek + Unpin {
    pub async fn new(mut reader: R) -> Result<ZipFileReader<R>> {
        let file = crate::read::file(&mut reader).await?;
        Ok(ZipFileReader { reader, file })
    }
}