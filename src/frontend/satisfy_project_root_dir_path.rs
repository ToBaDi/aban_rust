use crate::intermediate_data_structure::{
    project_root_dir_path::ProjectRootDirPath, IntermediateDataStructure,
};

pub fn satisfy(ids: &mut IntermediateDataStructure) -> Result<(), ()> {
    let cli_root = &ids.cli.root;
    let arg_path = &ids.args.next().unwrap();
    ids.project_root_dir_path = match ProjectRootDirPath::new(&cli_root, arg_path) {
        Ok(root) => Some(root),
        Err(error) => {
            ids.errors.push(error.into());
            return Err(());
        }
    };
    return Ok(());
}
