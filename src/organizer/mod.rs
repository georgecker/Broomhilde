use crate::config::{self, BroomhildeConfig};
use chrono::Local;
use std::{
    fs::{self},
    path::Path,
};

pub fn sweap(config: BroomhildeConfig) {
    let defualt = config.default_stratergy;
    config.folder_configs.iter().for_each(|fc| {
        let stratergy = fc.stratergy.unwrap_or(defualt);
        match stratergy {
            config::Stratergy::Folderize => folderize(&fc.path),
            config::Stratergy::Clear => clear(&fc.path),
            config::Stratergy::Reduce => todo!(),
        }
    });
}

fn folderize(path: &str) {
    let dir = fs::read_dir(path).unwrap();
    let today = Local::now()
        .naive_local()
        .date()
        .format("%Y-%m-%d")
        .to_string();
    let todays_folder = {
        let new_path = Path::new(path).join(&today);

        if !fs::exists(&new_path).unwrap() {
            fs::create_dir_all(&new_path).unwrap();
        }

        new_path
    };

    let files = dir.filter(|dir_e| {
        dir_e
            .as_ref()
            .map(|e| e.file_name().to_str().unwrap() != &today)
            .is_ok()
    });

    files.for_each(|f| {
        let entry = f.as_ref().unwrap();
        let name = entry.file_name();
        let dest = todays_folder.clone().join(name);
        fs::rename(entry.path(), dest).unwrap();
    });
}

fn clear(path: &str) {
    let dir = fs::read_dir(path).unwrap();
    println!("CLEARING {}", path);
    dir.for_each(|e| {
        let dir_entry = e.unwrap();
        let path = dir_entry.path();
        println!("DELETING File {:?}", path);
        fs::remove_file(path).unwrap();
        println!("DELETING Succesful");
    });
}
