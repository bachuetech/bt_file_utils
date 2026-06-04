# Project Title
BT FILE UTILS

## Description
A simple implementation to read a file from relative location or environment variable and return a String.

## Usage
```
let file_content = get_file(env_variable, or_file_name);  
```

## Version History
* 0.1.0
    * Initial Release
* 0.1.1
    * Use Rust 2024 Edition
* 0.1.2
    * Update dependencies
* 0.1.3
    * Update dependencies
* 0.2.0
    * Change of Error type returned by get_file to AnyErr
    * New function to unzip a file to a temp directory temp_unzip
    * Update dependencies
* 0.2.1
    * Update dependencies    
* 0.2.2
    * Update dependencies   
* 0.3.0
    * Update dependencies. Move from Box<dyn Error> to BT AnyErr.

## License
GPL-3.0-only