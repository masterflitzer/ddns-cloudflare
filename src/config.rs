use crate::structs::config::Config;
use directories::ProjectDirs;
use std::{
    fs::File,
    io::{Error as IOError, ErrorKind, Read},
    path::{Path, PathBuf},
};

fn cargo_name() -> String {
    env!("CARGO_PKG_NAME").replace('_', "-")
}

pub(crate) fn path() -> Result<PathBuf, IOError> {
    let name = cargo_name();

    let project_dirs =
        ProjectDirs::from("", "", &name).ok_or_else(|| IOError::from(ErrorKind::NotFound))?;
    let config_dir = ProjectDirs::config_dir(&project_dirs);

    let mut path = PathBuf::from(config_dir);
    path.push(format!("{}.toml", name));
    Ok(path)
}

pub(crate) fn get(path: impl AsRef<Path>) -> Result<Config, IOError> {
    std::fs::create_dir_all(
        path.as_ref()
            .parent()
            .ok_or_else(|| IOError::from(ErrorKind::NotFound))?,
    )?;

    let mut file = File::options()
        .read(true)
        .write(true)
        .append(false)
        .truncate(false)
        .create(true)
        .open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
