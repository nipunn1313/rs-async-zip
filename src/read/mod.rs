// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A module which supports reading ZIP files.

pub(crate) mod io;
pub mod seek;
pub mod mem;

use crate::error::{ZipError, Result};
use crate::entry::{ZipEntry, ZipEntryMeta};
use crate::spec::compression::Compression;
use crate::spec::header::CentralDirectoryHeader;
use crate::spec::attribute::AttributeCompatibility;

use tokio::io::{AsyncRead, Take};

pub(crate) async fn read_cd<R>(reader: Take<R>, num_of_entries: u64) -> Result<(Vec<ZipEntry>, Vec<ZipEntryMeta>)>
where 
    R: AsyncRead + Unpin
{
    let num_of_entries: usize = num_of_entries.try_into().map_err(|_| ZipError::TargetZip64Unsupported)?;
    let mut entries = Vec::with_capacity(num_of_entries);
    let mut metas = Vec::with_capacity(num_of_entries);

    for _ in 0..num_of_entries {

    }

    Ok((entries, metas))
}

pub(crate) async fn read_cd_entry<R>(mut reader: R) -> Result<(ZipEntry, ZipEntryMeta)>
where 
    R: AsyncRead + Unpin
{
    let header = CentralDirectoryHeader::from_reader(&mut reader).await?;
    let filename = crate::read::io::read_string(&mut reader, header.file_name_length.into()).await?;
    let compression = Compression::try_from(header.compression)?;
    let extra_field = crate::read::io::read_bytes(&mut reader, header.extra_field_length.into()).await?;
    let comment = crate::read::io::read_string(reader, header.file_comment_length.into()).await?;
    let last_modification_date = crate::spec::date::zip_date_to_chrono(header.mod_date, header.mod_time);

    let entry = ZipEntry {
        filename,
        compression,
        attribute_compatibility: AttributeCompatibility::Unix, /// FIXME: Default to Unix for the moment
        crc32: header.crc,
        uncompressed_size: header.uncompressed_size,
        compressed_size: header.compressed_size,
        last_modification_date,
        internal_file_attribute: header.inter_attr,
        external_file_attribute: header.exter_attr,
        extra_field,
        comment
    };

    let meta = ZipEntryMeta {
        general_purpose_flag: header.flags,
        file_offset: Some(header.lh_offset),
    };

    Ok((entry, meta))
}