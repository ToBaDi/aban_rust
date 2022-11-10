use std::{
    ffi::OsStr,
    fs::{DirEntry, File},
    io::{self, Read},
};

use super::project_root_dir_path::ProjectRootDirPath;

/// If [`AbanDotAbFile`].new() Failed.
#[derive(Debug)]
pub enum Error {
    FileNotFind,
    MultipleFile,
    OnOpenFile(io::Error),
    OnReadFile(io::Error),
}

impl Into<super::Error> for Error {
    fn into(self) -> super::Error {
        super::Error::OnAbanDotAbFile(self)
    }
}

/// Responsible to load and keep Aban.ab file data.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct AbanDotAbFile {
    /// Buffer were keep the Aban.ab file data.
    data: String,
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

        match file.read_to_string(&mut data) {
            Ok(_) => {}
            Err(error) => return Err(Error::OnReadFile(error)),
        };

        Ok(AbanDotAbFile { data })
    }

    /// Returns a reference to the data of this [`AbanDotAbFile`].
    pub fn data(&self) -> &str {
        &self.data
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

#[cfg(test)]
mod test {
    use super::{AbanDotAbFile, ProjectRootDirPath};
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    const TEST_TEXT: &str = "Purpose is Test\nThis File is for testing AbanDotAbFile object.\n";

    fn project_root_dir_path() -> ProjectRootDirPath {
        ProjectRootDirPath::new(&None, std::env::args().next().unwrap().as_str()).unwrap()
    }

    /// Aban.ab path.
    fn get_path() -> PathBuf {
        let root = project_root_dir_path();
        let path = {
            let mut path = root.path().to_owned();
            path.push("Aban.ab");
            path
        };
        path
    }

    fn create_aban_dot_ab() {
        let file_path = get_path();
        let mut file = File::create(&file_path).unwrap();
        file.write_fmt(format_args!("{TEST_TEXT}")).unwrap();
    }

    fn remove_aban_dot_ab() {
        let file_path = get_path();
        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn aban_dot_ab_file_not_find() {
        let root = project_root_dir_path();
        let result = AbanDotAbFile::new(&root);
        let error = result.unwrap_err();
        match error {
            super::Error::FileNotFind => (),
            super::Error::MultipleFile => panic!("Wrong error!"),
            super::Error::OnOpenFile(_) => panic!("Wrong error!"),
            super::Error::OnReadFile(_) => panic!("Wrong error!"),
        }
    }

    #[test]
    fn aban_dot_ab_file() {
        create_aban_dot_ab();
        let root = project_root_dir_path();
        let result = AbanDotAbFile::new(&root);
        let aban = result.unwrap();
        assert_eq!(aban.data, TEST_TEXT);
        remove_aban_dot_ab();
        aban_dot_ab_file_not_find();
    }
}
