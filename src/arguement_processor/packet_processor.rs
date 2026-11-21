use std::{fs::File, io::Error, path::PathBuf};

pub fn get_file(file: &PathBuf) -> Result<File, Error> {
    let file = File::open(file)?;
    Ok(file)
}
