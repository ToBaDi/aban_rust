use std::{
    fs::{read_dir, ReadDir},
    io,
    path::{Path, PathBuf},
};

#[derive(PartialEq, Debug)]
pub enum Error {
    NoDirectory,
}

/// Responsible for reading file system and
/// providing file and directory information
/// to the rest of the application.
#[derive(Debug, PartialEq)]
pub struct ProjectRootDirPath(PathBuf);

impl ProjectRootDirPath {
    /// Creates a new [`FileSystem`].
    /// - cli_root is suppose fo came from [`crate::Cli`]
    /// - arg_path is suppose to came from [`std::env::args()`].
    pub fn new(cli_root: &Option<PathBuf>, arg_path: &str) -> Result<Self, Error> {
        let path = sanitize_root_dir_path(cli_root, arg_path)?;
        Ok(ProjectRootDirPath(path))
    }

    /// Returns a reference to the root of this [`FileSystem`].
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }

    /// List of all the items in [`FileSystem`] root directory.
    pub fn list(&self) -> io::Result<ReadDir> {
        read_dir(&self.0)
    }
}

/// Try to make a proper directory path from command-line argument.
/// - cli_root is suppose fo came from [`crate::Cli`]
/// - arg_path is suppose to came from [`std::env::args()`].
fn sanitize_root_dir_path(cli_root: &Option<PathBuf>, arg_path: &str) -> Result<PathBuf, Error> {
    // Check if Cli input is usable as PathBuf.
    // Or turn arg_root path to PathBuf.
    let path = match cli_root {
        Some(path) => path.to_owned(),
        None => PathBuf::from(arg_path),
    };

    // If path is not a directory, pop it!
    let path = {
        let mut res = path;
        if res.is_dir() == false {
            res.pop();
        }
        res
    };

    // Return path if it is a directory.
    match path.is_dir() {
        true => Ok(path),
        false => Err(Error::NoDirectory),
    }
}

#[cfg(test)]
mod test {
    use super::sanitize_root_dir_path;
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

    #[test]
    fn sanitize_root_dir_path_test() {
        let cli = Option::None;
        let root = "/";
        let empty = "";
        let aban = "/Aban";
        let abs_a = "/a";
        let rel_a = "a";
        let abs_a_b = "/a/b";
        let rel_a_b = "a/b";
        let error = Err(Error::NoDirectory);
        let res_root = Ok(PathBuf::from(root));
        let res_aban = Ok(PathBuf::from(aban));

        assert_eq!(sanitize_root_dir_path(&cli, root), res_root);
        assert_eq!(sanitize_root_dir_path(&cli, empty), error);
        assert_eq!(sanitize_root_dir_path(&cli, aban), res_aban);
        assert_eq!(sanitize_root_dir_path(&cli, abs_a), res_root);
        assert_eq!(sanitize_root_dir_path(&cli, rel_a), error);
        assert_eq!(sanitize_root_dir_path(&cli, abs_a_b), error);
        assert_eq!(sanitize_root_dir_path(&cli, rel_a_b), error);

        let cli_root = Some(PathBuf::from(root));
        let cli_empty = Some(PathBuf::from(empty));
        let cli_aban = Some(PathBuf::from(aban));
        let cli_abs_a = Some(PathBuf::from(abs_a));
        let cli_rel_a = Some(PathBuf::from(rel_a));
        let cli_abs_a_b = Some(PathBuf::from(abs_a_b));
        let cli_rel_a_b = Some(PathBuf::from(rel_a_b));

        assert_eq!(sanitize_root_dir_path(&cli_root, empty), res_root);
        assert_eq!(sanitize_root_dir_path(&cli_empty, empty), error);
        assert_eq!(sanitize_root_dir_path(&cli_aban, empty), res_aban);
        assert_eq!(sanitize_root_dir_path(&cli_abs_a, empty), res_root);
        assert_eq!(sanitize_root_dir_path(&cli_rel_a, empty), error);
        assert_eq!(sanitize_root_dir_path(&cli_abs_a_b, empty), error);
        assert_eq!(sanitize_root_dir_path(&cli_rel_a_b, empty), error);
    }
}
