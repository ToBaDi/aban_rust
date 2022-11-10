use crate::intermediate_data_structure::{
    aban_dot_ab_file::AbanDotAbFile, IntermediateDataStructure,
};

pub fn satisfy(ids: &mut IntermediateDataStructure) -> Result<(), ()> {
    if let Some(root) = &ids.project_root_dir_path {
        ids.aban_dot_ab = match AbanDotAbFile::new(root) {
            Ok(aban) => Some(aban),
            Err(error) => {
                println!("Error from opening Aban.ab file: {:?}\n", error);
                return Err(());
            }
        };
        return Ok(());
    } else {
        return Err(());
    }
}
