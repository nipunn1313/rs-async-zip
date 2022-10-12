// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::error::{Result, ZipError};
use crate::spec::consts::{EOCDR_SIGNATURE, EOCDR_LENGTH};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, SeekFrom};

/// 
const BUFFER_SIZE: usize = 2048;

/// 
const MIN_DATA_LENGTH: usize = EOCDR_LENGTH + EOCDR_SIGNATURE.to_le_bytes().len();

/// 
const EOCDR_UPPER_BOUND: u64 = EOCDR_LENGTH as u64;

/// 
const EOCDR_LOWER_BOUND: u64 = u16::MAX as u64;

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

/// Locate the 'end of central directory record' offset, if one exists.
/// 
/// 
pub(crate) async fn eocdr<R>(mut reader: R) -> Result<u64>
where 
    R: AsyncRead + AsyncSeek + Unpin,
{

    let length = reader.seek(SeekFrom::End(0)).await?;
    let position = reverse_seek(&mut reader, length, (EOCDR_LENGTH + BUFFER_SIZE) as u64).await?;

    while position >= EOCDR_LOWER_BOUND {

    }

    todo!();
}

pub(crate) async fn reverse_seek<R>(mut reader: R, length: u64, position: u64) -> std::io::Result<u64>
where 
    R: AsyncRead + AsyncSeek + Unpin
{
    reader.seek(SeekFrom::Start(length - std::cmp::min(length, position))).await
}

/// A type which holds data about a match within 'reverse_search_buffer()'.
/// 
/// The 'match_index' field indicates where the match starts with respects to the reverse order.
/// Ie. the match occurs at indexes <= match_index.
struct SignatureMatch {
    full_match: bool,
    match_index: usize,
}

// A naive reverse linear search along the buffer for the specified signature bytes.
fn reverse_search_buffer(buffer: &[u8], signature: &[u8]) -> Option<SignatureMatch> {
    // This is already surprisingly performant. For instance, using memchr::memchr() to match for the first byte of the
    // signature, and then manual byte comparisons for the remaining signature bytes was actually slower by a factor of
    // 2.25. This method was explored as tokio's `read_until()` implementation uses memchr::memchr().

    'outer: for index in (0..buffer.len()).rev() {
        for (signature_index, signature_byte) in signature.iter().rev().enumerate() {
            let index_to_check = index.checked_sub(signature_index);

            if index_to_check.is_none() && signature_index != 0 { 
                // We have a partial match but have hit the start of the buffer.
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