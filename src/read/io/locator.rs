// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4316
//! 
//! As with other ZIP libraries, we face the predicament that the end of central directory record may contain a
//! variable-length file comment. As a result, we cannot just make the assumption that the start of this record is
//! 18 bytes (the length of the EOCDR) offset from the end of the data - we must locate it ourselves.
//!
//! The `zip-rs` crate handles this by reading in reverse from the end of the data. This involes seeking backwards
//! by a single byte each iteration and reading 4 bytes into a u32. Whether this is performant/acceptable within a
//! a non-async context, I'm unsure, but it isn't desirable within an async context. Especially since we cannot just
//! place a [`BufReader`] infront of the upstream reader (as its internal buffer is invalidated on each seek).
//!
//! Reading in reverse is still desirable as the use of file comments is limited and they're unlikely to be large.
//!
//! The below method is one that compromises on these two contention points. Please submit an issue or PR if you know
//! of a better algorithm for this (and have tested/verified its performance).

#[cfg(doc)]
use tokio::io::BufReader;

use crate::error::{Result, ZipError};
use crate::spec::consts::{EOCDR_SIGNATURE, EOCDR_LENGTH, SIGNATURE_LENGTH};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, SeekFrom};

/// The buffer size used when locating the EOCDR, equal to 2KiB.
const BUFFER_SIZE: usize = 2048;

/// The upper bound of where the EOCDR signature cannot be located.
const EOCDR_UPPER_BOUND: u64 = EOCDR_LENGTH as u64;

/// The lower bound of where the EOCDR signature cannot be located.
const EOCDR_LOWER_BOUND: u64 = EOCDR_UPPER_BOUND + SIGNATURE_LENGTH as u64 + u16::MAX as u64;

/// Locate the `end of central directory record` offset, if one exists.
/// 
/// This method involves buffered reading in reverse and revese linear searching along those buffers for the EOCDR
/// signature. As a result of this buffered approach, we reduce seeks when compared to `zip-rs`'s by a factor of
/// the buffer size. We also then don't have to do individual u32 reads against the upstream reader.
/// 
/// Whilst I haven't done any in-depth benchmarks, when reading a ZIP file with the maximum length comment this method
/// saw a reduction in location time by a factor of 500 when compared with the `zip-rs` method.
pub(crate) async fn eocdr<R>(mut reader: R) -> Result<u64>
where 
    R: AsyncRead + AsyncSeek + Unpin,
{
    let length = reader.seek(SeekFrom::End(0)).await?;
    let signature = &EOCDR_SIGNATURE.to_le_bytes();
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut position = reader.seek(SeekFrom::Start(length.saturating_sub((EOCDR_LENGTH + BUFFER_SIZE) as u64))).await?;
    let mut search: Option<SignatureMatch> = None;

    loop {
        let read = reader.read(&mut buffer).await?;
        let slice = &buffer[..read];

        // If we have a previous partial match, we need to check whether the end of the newly read buffer would now
        // result in a full match.
        if let Some(matched) = search.as_ref() {
            let partial_buffer = &slice[slice.len() - (matched.match_index + 1)..];
            let partial_signature = &signature[..(matched.match_index + 1)];

            if let Some(new_matched) = reverse_search_buffer(partial_buffer, partial_signature) {
                // We should never have another partial match as we've calculated the partial bounds above.
                assert!(new_matched.full_match);
                return Ok(position + (slice.len() - (matched.match_index + 1)) as u64);
            }
        }

        if let Some(matched) = reverse_search_buffer(slice, signature) {
            if matched.full_match {
                return Ok(position + ((matched.match_index + 1) - signature.len()) as u64);
            } else {
                search = Some(matched);
            }
        }

        // If we hit the start of the data or the lower bound, we're unable to locate the EOCDR.
        if position == 0 || position <= length.saturating_sub(EOCDR_LOWER_BOUND) {
            return Err(ZipError::UnableToLocateEOCDR);
        }

        position = reader.seek(SeekFrom::Start(position.saturating_sub(BUFFER_SIZE as u64))).await?;
    }
}

/// A type which holds data about a match within 'reverse_search_buffer()'.
/// 
/// The 'match_index' field indicates where the match starts with respects to the reverse order (ie. the match occurs
/// at indexes <= match_index).
struct SignatureMatch {
    full_match: bool,
    match_index: usize,
}

/// A naive reverse linear search along the buffer for the specified signature bytes.
/// 
/// This is already surprisingly performant. For instance, using memchr::memchr() to match for the first byte of the
/// signature, and then manual byte comparisons for the remaining signature bytes was actually slower by a factor of
/// 2.25. This method was explored as tokio's `read_until()` implementation uses memchr::memchr().
fn reverse_search_buffer(buffer: &[u8], signature: &[u8]) -> Option<SignatureMatch> {
    'outer: for index in (0..buffer.len()).rev() {
        for (signature_index, signature_byte) in signature.iter().rev().enumerate() {
            let index_to_check = index.checked_sub(signature_index);

            // We have a partial match but have hit the start of the buffer.
            if index_to_check.is_none() && signature_index != 0 { 
                return Some(SignatureMatch { full_match: false, match_index: index });
            }

            if buffer[index_to_check.unwrap()] != *signature_byte {
                continue 'outer;
            }
        }

        return Some(SignatureMatch { full_match: true, match_index: index });
    }

    None
}

#[cfg(test)]
#[test]
fn search_one_byte_test() {
    let buffer: &[u8] = &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
    let signature: &[u8] = &[0x1];

    let matched = reverse_search_buffer(buffer, signature);
    assert!(matched.is_none());

    let buffer: &[u8] = &[0x2, 0x1, 0x0, 0x0, 0x0, 0x0];
    let signature: &[u8] = &[0x1];

    let matched = reverse_search_buffer(buffer, signature);
    assert!(matched.is_some());
    assert!(matched.as_ref().unwrap().full_match);
    assert_eq!(1, matched.as_ref().unwrap().match_index);
}

#[cfg(test)]
#[test]
fn search_two_byte_test() {
    let buffer: &[u8] = &[0x2, 0x1, 0x0, 0x0, 0x0, 0x0];
    let signature: &[u8] = &[0x2, 0x1];

    let matched = reverse_search_buffer(buffer, signature);
    assert!(matched.is_some());
    assert!(matched.as_ref().unwrap().full_match);
    assert_eq!(1, matched.as_ref().unwrap().match_index);

    let buffer: &[u8] = &[0x1, 0x0, 0x0, 0x0, 0x0, 0x0];
    let signature: &[u8] = &[0x2, 0x1];

    let matched = reverse_search_buffer(buffer, signature);
    assert!(matched.is_some());
    assert!(!matched.as_ref().unwrap().full_match);
    assert_eq!(0, matched.as_ref().unwrap().match_index);
}