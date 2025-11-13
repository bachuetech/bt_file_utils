#[cfg(test)]
mod file_utils_tests {

use bt_file_utils::get_file;
use bt_logger::{build_logger, LogLevel, LogTarget};

#[test]
fn test_relative_location(){
    build_logger("BACHUETECH", "BT.FILE.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );

    let content = get_file("fake_variable", "test_files/t_file.txt");
    println!("Content {:?}",&content);
    assert_eq!(content.unwrap().to_owned(),"Hello World!");
}

#[test]
fn test_env_variable(){
    build_logger("BACHUETECH", "BT.FILE.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR, None);

    let file_loc_var = "file_loc";

    unsafe{
        std::env::set_var(file_loc_var, "test_files/t_file.txt");
    }

    let content = get_file(file_loc_var, "fake_location/t_file.txt");
    println!("Content {:?}",&content);
    assert_eq!(content.unwrap().to_owned(),"Hello World!");
}

#[test]
fn test_negative(){
    build_logger("BACHUETECH", "BT.FILE.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );

    let content = get_file("fake_variable", "fake_location/t_file.txt");
    assert!(content.is_err());
}
}