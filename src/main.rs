mod aban_dot_ab_file;
mod cli;
mod project_root_dir_path;

pub use aban_dot_ab_file::AbanDotAbFile;
use clap::Parser;
pub use cli::Cli;
pub use project_root_dir_path::ProjectRootDirPath;

/// For now, main serve as a playground and test-field.
/// It probably needs rewrite and clarification for final, real use.
fn main() {
    /*
    Project Raeder.
    Package Reader.
    Module Reader.
    File Reader.
    Line Raeder.
    Section Reader.
    */

    /*
    Front end that reads Aban files.
        Read Aban.ab.
        Search all the directories specifid in that file.
        Read al the source files in those directories.
        Find spectations based on those files.
        Satisfy those expectation with those files.
        Collect errors.
        Collect project data.
        Build up AST with those data.
        Generate C files base on AST.
    */
}

/*
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

    for item in root.list().unwrap() {
        println!("{:?}", item);
    }

    let aban = match AbanDotAbFile::new(&root) {
        Ok(aban) => aban,
        Err(error) => {
            println!("Error from opening Aban.ab file: {:?}\n", error);
            return;
        }
    };

    println!("{:?}", aban.data());

    for _item in aban.data().chars() {}
}
*/
