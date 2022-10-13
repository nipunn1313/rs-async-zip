// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::file::{ZipFile, builder::ZipFileBuilder};
use crate::entry::ZipEntry;

/// A trait that extends [`ZipFile`]'s functionality.
pub trait ZipFileExt {
    /// Returns a list of entries where their filenames equal the one supplied.
    /// 
    /// If multiple entries with the same name are found, they're returned in the order they were found within the ZIP
    /// file.
    fn entries_with_filename(&self, filename: &str) -> Vec<&ZipEntry>;
}

impl ZipFileExt for ZipFile {
    fn entries_with_filename(&self, filename: &str) -> Vec<&ZipEntry> {
        self.entries.iter().filter(|entry| entry.filename() == filename).collect()
    }
}

/// A trait that extends [`ZipFileBuilder`]'s functionality.
pub trait ZipFileBuilderExt {

}

impl ZipFileBuilderExt for ZipFileBuilder {

}