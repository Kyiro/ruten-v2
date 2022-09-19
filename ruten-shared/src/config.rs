use crate::epic::get_fortnite_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct GameVersion {
    pub name: String,
    pub path: Box<Path>
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub selected_version: Option<Uuid>,
    pub versions: HashMap<Uuid, GameVersion>
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::default();

        if let Ok(Some(dir)) = get_fortnite_dir() {
            let uuid = Uuid::new_v4();

            config.selected_version = Some(uuid.clone());
            config.versions.insert(uuid, GameVersion {
                name: String::from("Latest"),
                path: dir
            });
        }

        config
    }

    pub fn get_current_version(&self) -> Option<&GameVersion> {
        self.versions.get(&self.selected_version?)
    }
}