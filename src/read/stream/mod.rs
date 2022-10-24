// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::entry::ZipEntry;
use tokio::io::AsyncRead;

pub struct ZipFileReader<R> where R: AsyncRead + Unpin {
    reader: R,
    state: State,
    entry: Option<ZipEntry>,
}

impl<R> ZipFileReader<R> where R: AsyncRead + Unpin {
    pub fn new(reader: R) -> Self {
        Self { reader, state: State::PositionedLFH, entry: None }
    }


}

enum State {
    PositionedLFH,
    ReadData,
    ReadDataDescriptor,
}