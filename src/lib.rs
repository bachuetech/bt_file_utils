use std::{env, fs::{self, File},  error::Error};

use bt_logger::get_error;
use tempfile::TempDir;
use zip::ZipArchive;

pub fn get_file(env_variable: &str, or_file_name: &str) -> Result<String, Box<dyn Error>>{
    let file_to_read: String;
    let error_msg: String;

    match env::var(env_variable) {
        Ok(value) => {error_msg = format!("Unable to read file from env variable: {} pointing to file {}", &env_variable, &value); file_to_read = value; },
        Err(_e) => {error_msg = format!("Unable to read file from default file: {}.", &or_file_name); file_to_read = or_file_name.to_owned();},
    }

    match fs::read_to_string(file_to_read) {
        Ok(content) => Ok(content), // File read successfully, return the content
        Err(e) => Err(get_error!("get_file","Cannot read file. {}. Error: {}",error_msg, e).into() ), // Return the error if file is not found or another IO error occurs
    }
}

///Temporarily unzip a file. File is unzip in a temp directory
pub fn temp_unzip(zip_path: &str) -> Result<TempDir, Box<dyn Error>>{
    let file = File::open(zip_path)?;
    let mut zip = ZipArchive::new(file)?;

    let dir = TempDir::new()?;
    for i in 0..zip.len() {
        let mut f = zip.by_index(i)?;
        let out = dir.path().join(f.name());

        if f.name().ends_with('/') {
            std::fs::create_dir_all(&out)?;
        } else {
            if let Some(p) = out.parent() {
                std::fs::create_dir_all(p)?;
            }
            let mut out_file = File::create(&out)?;
            std::io::copy(&mut f, &mut out_file)?;
        }
    }
    Ok(dir)
}
