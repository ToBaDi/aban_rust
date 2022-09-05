use clap::Parser;
use std::path::PathBuf;

/// Responsible for command-line argument parsing.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Project root directory path.
    #[clap(short, long, value_parser, value_name = "Root Directory")]
    pub root: Option<PathBuf>,
}
