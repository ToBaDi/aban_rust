mod cli;
mod file_system;
mod sanitize_root_dir_path;

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

fn load_aban_dot_ab(file_system: &FileSystem) {
    for item in file_system.list().unwrap() {
        println!("{:?}", item);
    }
}
