// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

pub(crate) mod compression;
#[cfg(feature = "date")]
pub(crate) mod date;
pub(crate) mod header;
pub(crate) mod parse;
pub(crate) mod consts;
pub(crate) mod version;
pub(crate) mod attribute;