#![allow(dead_code)]

use std::io;
use std::io::Read;
use std::fs::File;
use regex::Regex;

pub fn remove_ansi_colors(input: &str) -> String {
    // Create a regular expression pattern to match RGB ANSI color codes
    let ansi_pattern = Regex::new("\x1B\\[[0-9;]*m").unwrap();
    ansi_pattern.replace_all(input, "").into()
}

pub fn include_bytes_runtime(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e), // Failed to open file or file not found
    };

    let mut buffer = Vec::new();
    return match f.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e), // Failed to read from file
    };
}

pub fn path_exists(path: &str) -> bool {
    return std::path::Path::new(path).exists();
}

pub fn custom_error(error: &str) -> io::Error {
    return io::Error::new(io::ErrorKind::Other, error);
}
