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

#[cfg(test)]
mod unzip_tests {
    use std::error::Error;

    use bt_file_utils::temp_unzip;

    const TEST_ZIP_FILE: &str = "test_files/t_file.zip";
    #[test]
    fn unzip_extracts_files() -> Result<(), Box<dyn Error>> {
        let out_dir = temp_unzip(TEST_ZIP_FILE)?;

        let file1 = std::fs::read_to_string(out_dir.path().join("t_file.txt"))?;

        assert_eq!(file1, "Hello World!");
        Ok(())
    }

        #[test]
    fn unzip_fails_for_missing_file() {
        let result = temp_unzip("this_file_does_not_exist.zip");
        assert!(result.is_err());
    }

}
