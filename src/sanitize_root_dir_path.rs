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
pub fn sanitize_root_dir_path(cli: &Cli, arg_path: &str) -> Result<PathBuf, Error> {
    // Get path from arguments.
    let arg_root = Path::new(arg_path);
    let cli_root = &cli.root;

    // Check if Cli input is usable as PathBuf.
    // Or turn arg_root path to PathBuf.
    let path = match cli_root {
        Some(path) => path.to_owned(),
        None => arg_root.to_owned(),
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
        let root = "/";
        let empty = "";
        let aban = "/Aban";
        let a = "a";

        assert_eq!(sanitize_root_dir_path(&cli, root), Ok(PathBuf::from(root)));
        assert_eq!(sanitize_root_dir_path(&cli, empty), Err(Error::NoDirectory));
        assert_eq!(sanitize_root_dir_path(&cli, aban), Ok(PathBuf::from(aban)));
        assert_eq!(sanitize_root_dir_path(&cli, a), Err(Error::NoDirectory));
    }
}
