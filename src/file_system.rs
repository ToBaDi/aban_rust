use std::{
    fs::{read_dir, ReadDir},
    io,
    path::{Path, PathBuf},
};

/// Responsible for reading file system and
/// providing file and directory information
/// to the rest of the application.
///
///
///
#[derive(Debug, PartialEq)]
pub struct FileSystem {
    root: PathBuf,
}

impl FileSystem {
    /// Creates a new [`FileSystem`].
    ///
    /// # Panics
    ///
    /// Panics if root_path is NOT a directory.
    pub fn new(root_path: &Path) -> Self {
        assert!(
            root_path.is_dir(),
            "Path Passed to FileSystem::new() should be a Directory Path"
        );

        FileSystem {
            root: root_path.to_owned(),
        }
    }

    /// Returns a reference to the root of this [`FileSystem`].
    pub fn root(&self) -> &Path {
        self.root.as_path()
    }

    /// List of all the items in [`FileSystem`] root directory.
    pub fn list(&self) -> io::Result<ReadDir> {
        read_dir(&self.root)
    }
}

#[cfg(test)]
mod test {
    use crate::FileSystem;
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
            FileSystem::new(path_dir.as_path()),
            FileSystem { root: path_dir }
        );
    }

    #[test]
    #[should_panic]
    fn on_new_should_fail_on_file_path_passed() {
        let path_file = get_path_file();
        let _ = FileSystem::new(path_file.as_path());
    }
}
