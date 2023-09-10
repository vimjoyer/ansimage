#![allow(dead_code)]

use std::fs;
use std::io;
use std::io::{Read, Write};
use crate::log::*;
use crate::error;

pub fn read(path: &str) -> Result<String, io::Error> {
    match fs::File::open(path) {
        Ok(mut o) => {
            let mut file_contents = String::new();
            match o.read_to_string(&mut file_contents) {
                Ok(_o) => {},
                Err(e) => {
                    error!("Failed to read file contents!");

                    return Err(e);
                },
            };

            return Ok(file_contents.trim().to_string());
        },
        Err(e) => {
            error!("Failed to open file!");

            return Err(e)
        },
    }
}

pub fn write(contents: &str, path: &str) -> Result<(), io::Error> {
    match fs::File::create(path) {
        Ok(mut o) => {
            match o.write_all(contents.as_bytes()) {
                Ok(_o) => {},
                Err(e) => {
                    error!("Failed to write to file!");

                    return Err(e);
                },
            };
        },
        Err(e) => {
            error!("Failed to create file!");

            return Err(e);
        },
    };
    
    return Ok(());
}
