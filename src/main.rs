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
    let root = sanitize_root_dir_path(&cli, arg_path.as_str()).unwrap();

    let file_system = FileSystem::new(root.as_path());

    println!("{}", file_system.root().display());
}
