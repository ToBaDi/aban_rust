mod aban_dot_ab_file;
mod cli;
mod project_root_dir_path;

pub use aban_dot_ab_file::AbanDotAbFile;
use clap::Parser;
pub use cli::Cli;
pub use project_root_dir_path::ProjectRootDirPath;

fn main() {
    let root = match ProjectRootDirPath::new(
        &Cli::parse().root,
        std::env::args().next().unwrap().as_str(),
    ) {
        Ok(root) => root,
        Err(error) => match error {
            project_root_dir_path::Error::NoDirectory => {
                println!("Project Root Directory Path is InValid! Try Again.");
                return;
            }
        },
    };

    println!("Project Root Directory: {}\n", root.path().display());

    let aban_dot_ab = AbanDotAbFile::new(&root);

    if let Err(error) = aban_dot_ab {
        println!("Error from Aban.ab File: {:?}\n", error);
    }
}
