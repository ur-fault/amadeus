use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use path_absolutize::Absolutize;

use crate::package::Package;

pub static MAIN_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("~/.run-that/").absolutize().unwrap().into());
pub static REPOS_PATH: Lazy<PathBuf> = Lazy::new(|| MAIN_PATH.join("repos"));

pub fn get_package_info(path: &Path) -> Result<Package, Box<dyn std::error::Error>> {
    let full_path = if path.is_file() {
        path.to_path_buf()
    } else {
        path.join("run.yml")
    };

    let file = std::fs::File::open(full_path)?;
    let package: Package = serde_yaml::from_reader(file)?;
    Ok(package)
}
