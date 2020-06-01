use crate::range;
use directories::ProjectDirs;
use std::fs;
use std::io::Write;
use std::error::Error;
use serde_json;

pub fn load_ranges() -> Vec<range::Range> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Marble Devices", "Range Trainer") {
        let data_dir = proj_dirs.data_dir().join("ranges.json");
        let path = data_dir.as_path();
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            } else {
                dbg!("Couldn't parse json");
            }
        } else {
            dbg!("Couldnt read contents");
        }
    } else {
        dbg!("couldnt get project dir");
    }
    return vec![];
}


pub fn save_ranges(ranges: &Vec<range::Range>) -> Result<(), Box<dyn Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Marble Devices", "Range Trainer") {
        fs::create_dir_all(proj_dirs.data_dir())?;
        let data_dir = proj_dirs.data_dir().join("ranges.json");
        let path = data_dir.as_path();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
            
        file.write_all(&serde_json::to_string(ranges)?.as_bytes())?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unable to create app directory.")))
    }
}
