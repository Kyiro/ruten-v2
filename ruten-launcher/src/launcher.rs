use derive_more::{Display, Error, From};
use include_crypt::{include_crypt, EncryptedFile};
use ruten_shared::caldera::Claims;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fs;
use std::path::Path;
use std::process;

static WAITER: EncryptedFile = include_crypt!("./resources/waiter.exe");

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;

        let file_type = entry.file_type()?;
        let copy_path = dst.as_ref().join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(entry.path(), copy_path)?;
        } else {
            fs::copy(entry.path(), copy_path)?;
        }
    }

    Ok(())
}

fn use_waiter(file: impl AsRef<Path>) -> std::io::Result<process::Child> {
    let file = file.as_ref();

    fs::remove_file(file)?;
    fs::write(file, WAITER.decrypt())?;

    Ok(
        process::Command::new(file)
        .spawn()?
    )
}

#[derive(Clone, Default)]
pub struct ArgsBuilder {
    args: HashMap<String, Option<String>>
}

impl ArgsBuilder {
    pub fn new() -> Self {
        Self::default()
            .insert("epicapp", Some("Fortnite"))
            .insert("epicenv", Some("Prod"))
            .insert("epicportal", None::<String>)
            .insert("skippatchcheck", None::<String>)
            .insert("caldera", Some(Claims::default().encode_default().unwrap_or(String::new())))
            .clone()
    }

    pub fn auth<T: ToString>(&mut self, auth_login: T, auth_password: T, auth_type: T) -> &mut Self {
        self
            .insert("AUTH_LOGIN", Some(auth_login))
            .insert("AUTH_PASSWORD", Some(auth_password))
            .insert("AUTH_TYPE", Some(auth_type))
    }

    pub fn insert<K: ToString, V: ToString>(&mut self, key: K, val: Option<V>) -> &mut Self {
        self.args.insert(key.to_string(), val.map(|i| i.to_string()));

        self
    }

    pub fn build(&self) -> Vec<String> {
        let mut args = Vec::new();

        for (key, val) in self.args.clone() {
            let mut formatted = format!("-{}", key);

            if let Some(val) = val {
                formatted.push_str(&format!("={}", val));
            }

            args.push(formatted);
        }

        args
    }
}

#[derive(Debug, Display, Error, From)]
pub enum LaunchError {
    #[display(fmt = "Cannot find the current version")]
    NoCurrentVersion,
    #[display(fmt = "Invalid Game Folder")]
    InvalidGameFolder,
    #[display(fmt = "Cannot find the FortniteGame and Engine folders")]
    NotFortnite,
    #[display(fmt = "Incomplete Installation")]
    IncompleteInstall
}

#[non_exhaustive]
#[derive(Clone, Deserialize, Serialize)]
pub struct LaunchSettings {
    pub method: LaunchMethod,
    pub arguments: Vec<String>,
    pub injectables: Vec<Vec<u8>>
}

#[non_exhaustive]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum LaunchMethod {
    NoLauncher, // Before season 4 or when using a RequestEngineExit hook
    UsingLauncher, // Before 18.10
    Caldera, // Using Caldera, after 18.10
    Auto // Automatically select the launching method
}

impl Default for LaunchSettings {
    fn default() -> Self {
        Self {
            method: LaunchMethod::Auto,
            arguments: ArgsBuilder::default().build(),
            injectables: Vec::new()
        }
    }
}

impl Default for LaunchMethod {
    fn default() -> Self { Self::Auto }
}

pub fn launch(path: impl AsRef<Path>, settings: LaunchSettings) -> Result<(), Box<dyn StdError>> {
    let game_path = path.as_ref();
    let mut launch_method = settings.method;

    if !game_path.is_dir() { return Err(Box::new(LaunchError::InvalidGameFolder)); }

    if !game_path.join("FortniteGame").is_dir() || !game_path.join("Engine").is_dir() {
        return Err(Box::new(LaunchError::NotFortnite));
    }

    let binaries_folder = game_path.join("FortniteGame").join("Binaries");

    let win64_folder = binaries_folder.join("Win64");
    let mut work_folder = binaries_folder.join("Ruten");

    if !win64_folder.is_dir() { return Err(Box::new(LaunchError::IncompleteInstall)) }
    if win64_folder.join("FortniteLauncher.exe").is_file() {
        launch_method = LaunchMethod::NoLauncher;
    }

    let mut children: Vec<process::Child> = Vec::new();
    
    if launch_method != LaunchMethod::NoLauncher {
        copy_dir_all(win64_folder.clone(), work_folder.clone())?;

        for i in [
            "FortniteClient-Win64-Shipping_EAC.exe",
            "FortniteLauncher.exe"
        ] {
            children.push(use_waiter(work_folder.join(i))?);
        }
    } else {
        work_folder = win64_folder.clone();
    }

    if launch_method != LaunchMethod::UsingLauncher {
        let mut game_client = process::Command::new(work_folder.join("FortniteClient-Win64-Shipping.exe"))
            .args(settings.arguments)
            .spawn()?;

        println!("{}", game_client.id());

        game_client.wait()?;
    } else {
        // IMPLEMENT LATER
    }

    for child in children.iter_mut() {
        let _ = child.kill();
    }

    Ok(())
}