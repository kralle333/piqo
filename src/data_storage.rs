use models::Project;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use owo_colors::OwoColorize;

use crate::models;

pub enum CrabdPath {
    NotFound(gix_discover::upwards::Error),
    FoundNotInit(PathBuf),
    Found(PathBuf),
}

pub fn check_crabd_dir() -> CrabdPath {
    let git_location = gix_discover::upwards(Path::new("."));

    let git_location = match git_location {
        Ok(git_path) => git_path.0,
        Err(err) => {
            return CrabdPath::NotFound(err);
        }
    };

    let (_, github_folder_dir) = git_location.into_repository_and_work_tree_directories();

    let crabd_json_path = github_folder_dir.unwrap().join(".crabd");
    if crabd_json_path.exists() {
        CrabdPath::Found(crabd_json_path)
    } else {
        CrabdPath::FoundNotInit(crabd_json_path)
    }
}

pub(crate) fn store_project(p: &Project) -> Result<(), std::io::Error> {
    let crabd = check_crabd_dir();

    let path = match crabd {
        CrabdPath::NotFound(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unable to store .cradb file: {}", err),
            ))
        }
        CrabdPath::FoundNotInit(path) | CrabdPath::Found(path) => Some(path),
    };
    let mut file = File::create(path.unwrap())?;
    let serialized = serde_json::to_string(p)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub(crate) fn load_project() -> Result<Project, std::io::Error> {
    let crabd = check_crabd_dir();

    let path = match crabd {
        CrabdPath::NotFound(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unable to load project, .cradb file could not be found: {}",
                    err
                ),
            ));
        }
        CrabdPath::Found(path) => Some(path),
        CrabdPath::FoundNotInit(path) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unable to load project, .cradb file could not be found: {:?} - {} ",
                    path,
                    "try running `crabd init`".green(),
                ),
            ));
        }
    };

    let file = File::open(path.unwrap())?;
    let rdr = BufReader::new(file);
    let p: Project = serde_json::from_reader(rdr)?;
    Ok(p)
}

pub(crate) fn get_user_email() -> Option<String> {
    None
}
