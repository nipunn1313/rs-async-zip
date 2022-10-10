// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use tokio::io::AsyncRead;

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4316
// 
// As with other ZIP libraries, we face the predicament that the end of central directory record may contain a
// variable-length file comment. As a result, we cannot just make the assumption that the start of this record is
// 18 bytes (the length of the EOCDR) offset from the end of the data - we must locate it ourselves.
//
// The 'zip-rs' crate handles this by reading in reverse from the end of the data. This involes seeking backwards
// by a single byte each iteration and reading 4 bytes into a u32. Whether this is performant/acceptable within a
// a non-async context, I'm unsure, but it isn't desirable within an async context. Especially since we cannot just
// place a 'BufReader' infront of the upstream reader (as its internal buffer is invalidated on each seek).
//
// Reading in reverse is still desirable as the use of file comments is limited and they're unlikely to be large.
//
// The below solution is one that compromises on these two contention points. Please submit an issue or PR if you know
// of a better algorithm for this (and have tested/verified its performance).

pub(crate) async fn signature_locator<R>(reader: R, signature: &[u8]) -> std::io::Result<Vec<usize>>
where 
    R: AsyncRead + Unpin,
{
    let locations: Vec<usize> = Vec::new();

    loop {
        
    }

    Ok(locations)
}