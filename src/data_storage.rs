use models::Project;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use owo_colors::OwoColorize;

use crate::models;

pub enum PiqoPath {
    NotFound(gix_discover::upwards::Error),
    FoundNotInit(PathBuf),
    Found(PathBuf),
}

pub(crate)fn check_piqo_dir() -> PiqoPath {
    let git_location = gix_discover::upwards(Path::new("."));

    let git_location = match git_location {
        Ok(git_path) => git_path.0,
        Err(err) => {
            return PiqoPath::NotFound(err);
        }
    };

    let (_, github_folder_dir) = git_location.into_repository_and_work_tree_directories();

    let piqo_json_path = github_folder_dir.unwrap().join(".piqo");
    if piqo_json_path.exists() {
        PiqoPath::Found(piqo_json_path)
    } else {
        PiqoPath::FoundNotInit(piqo_json_path)
    }
}

pub(crate) fn store_project(p: &Project) -> Result<(), std::io::Error> {
    let piqo = check_piqo_dir();

    let path = match piqo {
        PiqoPath::NotFound(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unable to store .piqo file: {}", err),
            ))
        }
        PiqoPath::FoundNotInit(path) | PiqoPath::Found(path) => Some(path),
    };
    let mut file = File::create(path.unwrap())?;
    let serialized = serde_json::to_string(p)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub(crate) fn load_project() -> Result<Project, std::io::Error> {
    let piqo = check_piqo_dir();

    let path = match piqo {
        PiqoPath::NotFound(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unable to load project, .piqo file could not be found: {}",
                    err
                ),
            ));
        }
        PiqoPath::Found(path) => Some(path),
        PiqoPath::FoundNotInit(path) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unable to load project, .piqo file could not be found: {:?} - {} ",
                    path,
                    "try running `piqo init`".green(),
                ),
            ));
        }
    };

    let file = File::open(path.unwrap())?;
    let rdr = BufReader::new(file);
    let p: Project = serde_json::from_reader(rdr)?;
    Ok(p)
}
