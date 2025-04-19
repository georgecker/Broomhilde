use std::{
    fs::{self, File, OpenOptions},
    io::{self},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BroomhildeConfig {
    pub default_stratergy: Stratergy,
    pub folder_configs: Vec<FolderConfig>,
}

impl BroomhildeConfig {
    pub fn test_data() -> Self {
        Self {
            default_stratergy: Stratergy::Folderize,
            folder_configs: vec![
                FolderConfig {
                    path: "/Users/georgecker/Downloads".to_string(),
                    stratergy: Some(Stratergy::Folderize),
                },
                FolderConfig {
                    path: "/Users/georgecker/Desktop".to_string(),
                    stratergy: Some(Stratergy::Clear),
                },
            ],
        }
    }
}

impl Default for BroomhildeConfig {
    fn default() -> Self {
        Self {
            default_stratergy: Stratergy::Folderize,
            folder_configs: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FolderConfig {
    pub path: String,
    pub stratergy: Option<Stratergy>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Stratergy {
    Folderize = 1, // Collects all files into a Daily folder
    Clear = 2,     // Clears folder
    Reduce = 3,    // Reduces files with mutual token in name into a folder
}

impl Stratergy {
    pub fn to_string(&self) -> String {
        match self {
            Self::Folderize => "Folderize".to_string(),
            Self::Clear => "Clear".to_string(),
            Self::Reduce => "Reduce".to_string(),
        }
    }
}

/// Saves config to OS specified path
pub fn save_config(config: BroomhildeConfig) -> Result<(), io::Error> {
    let path = get_os_config_path();
    let (index_last_sub, _) = path
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '/' || *c == '\\')
        .last()
        .unwrap();
    let (dir_path, _) = path.split_at(index_last_sub);

    fs::create_dir_all(dir_path)?;

    let file = OpenOptions::new().write(true).create(true).open(&path)?;
    serde_json::to_writer(file, &config).map_err(|e| io::Error::from(e.io_error_kind().unwrap()))
}

/// Retrieves config from OS specified path
pub fn get_config() -> Result<BroomhildeConfig, io::Error> {
    let path = get_os_config_path();
    if fs::exists(&path)? {
        let file = File::open(&path)?;
        let config: BroomhildeConfig = serde_json::from_reader(file)?;
        return Ok(config);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("No file found in path '{}'", &path),
    ))
}

fn get_os_config_path() -> String {
    "~/.config/broomhilde/config".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread::sleep, time::Duration};

    #[test]
    fn save_config_assert_success() {
        reset();
        wait_ready();

        let folder_config_one = FolderConfig {
            path: "~/Downloads".to_string(),
            stratergy: Some(Stratergy::Folderize),
        };
        let folder_config_two = FolderConfig {
            path: "~/Desktop".to_string(),
            stratergy: Some(Stratergy::Reduce),
        };
        let config = BroomhildeConfig {
            default_stratergy: Stratergy::Folderize,
            folder_configs: vec![folder_config_one, folder_config_two],
        };

        save_config(config).unwrap();
    }

    fn reset() {
        let path = get_os_config_path();
        if fs::exists(&path).unwrap() {
            fs::remove_file(&path).unwrap();
        }
    }

    fn wait_ready() {
        let path = get_os_config_path();
        loop {
            if fs::exists(&path).unwrap() {
                sleep(Duration::from_millis(200));
            } else {
                return;
            }
        }
    }
}
