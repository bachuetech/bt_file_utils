use std::{env, fs, io::Error};

use bt_logger::{log_error, log_trace};

pub fn get_file(env_variable: &str, or_file_name: &str) -> Result<String, Error>{
    let file_to_read: String;
    let error_msg: String;

    match env::var(env_variable) {
        Ok(value) => {error_msg = format!("Unable to read file from env variable: {} pointing to file {}", &env_variable, &value); file_to_read = value; },
        Err(_e) => {error_msg = format!("Unable to read file from default file: {}.", &or_file_name); file_to_read = or_file_name.to_owned();},
    }

    log_trace!("get_file", "file name: {}",file_to_read);
    match fs::read_to_string(file_to_read) {
        Ok(content) => Ok(content), // File read successfully, return the content
        Err(e) => {log_error!("get_file","{}. Error: {}",error_msg,&e.to_string()); Err(e)}, // Return the error if file is not found or another IO error occurs
    }
}
