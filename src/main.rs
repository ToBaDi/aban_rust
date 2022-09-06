mod cli;
mod file_system;
mod sanitize_root_dir_path;

use std::{ffi::OsStr, fs::DirEntry};

use clap::Parser;
pub use cli::Cli;
pub use file_system::FileSystem;
pub use sanitize_root_dir_path::sanitize_root_dir_path;

fn main() {
    let arg_path = std::env::args().next().unwrap();
    let cli = Cli::parse();
    let root = sanitize_root_dir_path(&cli.root, arg_path.as_str());
    if root.is_ok() == false {
        println!("Project Root Directory Path is InValid! Try Again.");
    }
    let root = root.unwrap();
    let file_system = FileSystem::new(root.as_path());

    println!("{}\n", file_system.root().display());

    load_aban_dot_ab(&file_system);
}

fn find_aban_dot_ab(file_system: &FileSystem) {
    let mut aban_entries = Vec::<DirEntry>::with_capacity(2);
    let name = OsStr::new("Aban.ab").to_ascii_lowercase();

    for item in file_system.list().unwrap() {
        let item = item.unwrap();
        println!("{:?}", item.file_name());
        if item.file_name().to_ascii_lowercase() == name {
            aban_entries.push(item);
        }
    }

    println!();
    for item in aban_entries {
        println!("{:?}", item);
    }
}
