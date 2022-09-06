use std::{
    ffi::OsStr,
    fs::{DirEntry, File},
    io::{self, Read},
};

use crate::ProjectRootDirPath;

/// If [`AbanDotAbFile`].new() Failed.
#[derive(Debug)]
pub enum Error {
    FileNotFind,
    MultipleFile,
    OnOpenFile(io::Error),
    OnReadFile(io::Error),
}

/// Responsible to load and keep Aban.ab file data.
pub struct AbanDotAbFile {
    /// Buffer were keep the Aban.ab file data.
    data: String,
    /// Number of bytes were read to data.
    len: usize,
}

impl AbanDotAbFile {
    /// Creates a new [`AbanDotAbFile`].
    pub fn new(root: &ProjectRootDirPath) -> Result<Self, Error> {
        let list = Self::aban_dot_ab_list(root);

        if list.is_empty() {
            return Err(Error::FileNotFind);
        }

        if list.len() > 1 {
            return Err(Error::MultipleFile);
        }

        let mut file = match File::open(&list[0].path()) {
            Ok(file) => file,
            Err(error) => return Err(Error::OnOpenFile(error)),
        };

        let mut data = String::new();
        let len = 0;

        match file.read_to_string(&mut data) {
            Ok(len) => len,
            Err(error) => return Err(Error::OnReadFile(error)),
        };

        Ok(AbanDotAbFile { data, len })
    }

    /// Returns a reference to the data of this [`AbanDotAbFile`].
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Returns the length of this [`AbanDotAbFile`].
    pub fn len(&self) -> usize {
        self.len
    }

    /// List of entries in project root directory with Aban.ab name.
    /// - case insensitive.
    fn aban_dot_ab_list(root: &ProjectRootDirPath) -> Vec<DirEntry> {
        let mut aban_entries = Vec::<DirEntry>::with_capacity(2);
        let name = OsStr::new("Aban.ab").to_ascii_lowercase();

        for item in root.list().unwrap() {
            let item = item.unwrap();
            if item.file_name().to_ascii_lowercase() == name {
                aban_entries.push(item);
            }
        }

        aban_entries
    }
}
