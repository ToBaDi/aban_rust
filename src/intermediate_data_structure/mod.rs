use clap::Parser;

use self::{aban_dot_ab_file::AbanDotAbFile, project_root_dir_path::ProjectRootDirPath};

pub mod aban_dot_ab_file;
pub mod cli;
pub mod project_root_dir_path;

#[derive(Debug)]
pub enum Error {
    OnProjectRootDirPath(project_root_dir_path::Error),
    OnAbanDotAbFile(aban_dot_ab_file::Error),
}

#[derive(Debug)]
pub struct IntermediateDataStructure {
    pub cli: cli::Cli,
    pub args: std::env::Args,
    pub project_root_dir_path: Option<ProjectRootDirPath>,
    pub aban_dot_ab: Option<AbanDotAbFile>,
    pub is_ready: bool,
    pub errors: Vec<Error>,
}

impl IntermediateDataStructure {
    pub fn new() -> Self {
        return IntermediateDataStructure::default();
    }
}

impl Default for IntermediateDataStructure {
    fn default() -> Self {
        Self {
            cli: cli::Cli::parse(),
            args: std::env::args(),
            project_root_dir_path: Default::default(),
            aban_dot_ab: Default::default(),
            is_ready: Default::default(),
            errors: Default::default(),
        }
    }
}
