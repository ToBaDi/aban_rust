use crate::Cli;
use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug)]
pub enum Error {
    // NoAbsolute,
    NoDirectory,
}

/// Try to make a proper directory path from [`Cli`]
/// or command-line argument.
/// arg_path is suppose to came from [`std::env::args()`].
///
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

    // // Some helper booleans.
    // let b_is_arg_root_relative = arg_root.is_relative();
    // let b_is_path_relative = path.is_relative();
    // let b_is_cli_path = cli_root.is_some();
    //
    // // Check if path is relative.
    // // If CLi path is relative, combine it with arg_root to make it absolute.
    // // If path and arg_path are relative, return None.
    // let path = if b_is_path_relative == false {
    // path.to_owned()
    // } else if b_is_cli_path && b_is_arg_root_relative == false {
    // let mut res = arg_root.to_owned();
    // res.push(path);
    // res
    // } else {
    // return Err(Error::NoAbsolute);
    // };

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
mod tests {
    use std::path::PathBuf;

    use crate::{sanitize_root_dir_path, sanitize_root_dir_path::Error, Cli};
    use clap::Parser;

    #[test]
    fn sanitize_root_dir_path_test() {
        let cli = Cli::parse();
        let cli_root = cli.root;
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

        assert_eq!(sanitize_root_dir_path(&cli_root, root), res_root);
        assert_eq!(sanitize_root_dir_path(&cli_root, empty), error);
        assert_eq!(sanitize_root_dir_path(&cli_root, aban), res_aban);
        assert_eq!(sanitize_root_dir_path(&cli_root, abs_a), res_root);
        assert_eq!(sanitize_root_dir_path(&cli_root, rel_a), error);
        assert_eq!(sanitize_root_dir_path(&cli_root, abs_a_b), error);
        assert_eq!(sanitize_root_dir_path(&cli_root, rel_a_b), error);
    }
}
