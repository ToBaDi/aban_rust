use super::Error;
use std::path::PathBuf;

/// Try to make a proper directory path from command-line argument.
/// - cli_root is suppose fo came from [`crate::Cli`]
/// - arg_path is suppose to came from [`std::env::args()`].
pub fn sanitize_root_dir_path(
    cli_root: &Option<PathBuf>,
    arg_path: &str,
) -> Result<PathBuf, Error> {
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
    use std::path::PathBuf;

    use super::Error;

    use super::sanitize_root_dir_path;

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
