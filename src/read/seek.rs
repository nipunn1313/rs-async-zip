// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::read::io::{compressed::CompressedReader, hashed::HashedReader, owned::OwnedReader};
use crate::spec::compression::Compression;

use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncReadExt, Take, ReadBuf};
use pin_project::pin_project;

pub struct ZipFileReader<R> where R: AsyncRead + Unpin {
    reader: R
}
