mod sanitizer;

use self::sanitizer::sanitize_root_dir_path;
use std::{
    fs::{read_dir, ReadDir},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoDirectory,
}

impl Into<super::Error> for Error {
    fn into(self) -> super::Error {
        super::Error::OnProjectRootDirPath(self)
    }
}

/// Responsible for reading file system and
/// providing file and directory information
/// to the rest of the application.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ProjectRootDirPath(PathBuf);

impl ProjectRootDirPath {
    /// Creates a new [`ProjectRootDirPath`].
    /// - cli_root is suppose fo came from [`crate::Cli`]
    /// - arg_path is suppose to came from [`std::env::args()`].
    pub fn new(cli_root: &Option<PathBuf>, arg_path: &str) -> Result<Self, Error> {
        let path = sanitize_root_dir_path(cli_root, arg_path)?;
        Ok(ProjectRootDirPath(path))
    }

    /// Returns a reference to the root of this [`ProjectRootDirPath`].
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }

    /// List of all the items in [`ProjectRootDirPath`] root directory.
    pub fn list(&self) -> io::Result<ReadDir> {
        read_dir(&self.0)
    }
}

#[cfg(test)]
mod test {
    use super::Error;
    use super::ProjectRootDirPath;
    use std::path::PathBuf;

    fn get_path_file() -> PathBuf {
        let args: Vec<String> = std::env::args().collect();
        let path_file = std::path::Path::new(&args[0]);
        return path_file.to_owned();
    }

    fn get_path_dir() -> PathBuf {
        let path_file = get_path_file();
        let mut path_file_ancestors = path_file.ancestors();
        path_file_ancestors.next();
        let path_dir = path_file_ancestors.next().unwrap();
        return path_dir.to_owned();
    }

    #[test]
    fn on_new_path_passed_correctly() {
        let path_dir = get_path_dir();
        assert_eq!(
            ProjectRootDirPath::new(&None, path_dir.to_str().unwrap()).unwrap(),
            ProjectRootDirPath(path_dir)
        );

        let path_dir = get_path_dir();
        let path_file = get_path_file();
        assert_eq!(
            ProjectRootDirPath::new(&None, path_file.to_str().unwrap()).unwrap(),
            ProjectRootDirPath(path_dir)
        );
    }

    #[test]
    fn on_new_should_fail_on_file_path_passed() {
        let path = "/aban.ab/aban.ab";
        assert_eq!(
            ProjectRootDirPath::new(&None, path),
            Err(Error::NoDirectory),
        )
    }
}
