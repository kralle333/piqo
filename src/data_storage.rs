use models::Project;
use std::{
    fs::File,
    io::{BufReader, Write},
};

use crate::models;

pub(crate) fn store_project(p: &Project) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(p)?;

    let mut file = File::create(".crabd")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub(crate) fn load_project() -> Result<Project, std::io::Error> {
    let file = File::open(".crabd")?;
    let rdr = BufReader::new(file);
    let p: Project = serde_json::from_reader(rdr)?;
    Ok(p)
}
