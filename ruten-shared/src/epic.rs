use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error as StdError;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstallationList {
    pub installation_list: Vec<Installation>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Installation {
    pub install_location: Box<Path>,
    pub namespace_id: String,
    pub item_id: String,
    pub artifact_id: String,
    pub app_version: String,
    pub app_name: String
}

pub fn get_installation_list() -> Result<InstallationList, Box<dyn StdError>> {
    let program_data = env::var("ProgramData")?;
    let program_data = Path::new(&program_data);

    let launcher_installed = program_data
        .join("Epic")
        .join("UnrealEngineLauncher")
        .join("LauncherInstalled.dat");
    
    Ok(serde_json::from_str(&fs::read_to_string(launcher_installed)?)?)
}

pub fn get_fortnite_dir() -> Result<Option<Box<Path>>, Box<dyn StdError>> {
    Ok(
        get_installation_list()?
        .installation_list
        .into_iter()
        .find(|i| i.app_name == "Fortnite")
        .map(|i| i.install_location)
    )
}