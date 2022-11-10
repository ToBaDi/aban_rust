mod satisfy_aban_dot_ab_file;
mod satisfy_project_root_dir_path;

use crate::intermediate_data_structure::IntermediateDataStructure;

pub fn start(intermediate_data_structure: &mut IntermediateDataStructure) -> Result<(), ()> {
    let ids = intermediate_data_structure;
    satisfy_project_root_dir_path::satisfy(ids)?;
    satisfy_aban_dot_ab_file::satisfy(ids)?;
    return Ok(());
}
