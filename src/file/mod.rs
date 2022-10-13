// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

pub(crate) mod builder;
pub(crate) mod ext;

use crate::entry::{ZipEntry, ZipEntryMeta};

/// An immutable store of data about a ZIP file.
pub struct ZipFile {
    pub(crate) entries: Vec<ZipEntry>,
    pub(crate) metas: Vec<ZipEntryMeta>,
    pub(crate) comment: String,
}

impl ZipFile {
    /// Returns a list of this ZIP file's entries.
    pub fn entries(&self) -> &[ZipEntry] {
        &self.entries
    }

    /// Returns this ZIP file's trailing comment.
    pub fn comment(&self) -> &str {
        &self.comment
    }
}

