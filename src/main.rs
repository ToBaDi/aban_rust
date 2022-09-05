mod file_system;
use clap;

pub use file_system::FileSystem;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_file = std::path::Path::new(&args[0]);
    let mut path_file_ancestors = path_file.ancestors();
    path_file_ancestors.next();
    let path_dir = path_file_ancestors.next().unwrap();

    let file_system = FileSystem::new(path_dir);

    println!("{}", file_system.root().display());

    // find_aban_dot_ab(&file_system);
}

pub fn find_aban_dot_ab(file_system: &FileSystem) {}
