use crate::intermediate_data_structure::{
    aban_dot_ab_file::AbanDotAbFile, IntermediateDataStructure,
};

pub fn satisfy(ids: &mut IntermediateDataStructure) -> Result<(), ()> {
    ids.aban_dot_ab = match AbanDotAbFile::new(ids.project_root_dir_path.as_ref().unwrap()) {
        Ok(aban) => Some(aban),
        Err(error) => {
            println!("Error from opening Aban.ab file: {:?}\n", error);
            return Err(());
        }
    };
    return Ok(());
}
