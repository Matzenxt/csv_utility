use std::fs::File;
use std::path::Path;
use seahorse::Context;
use seahorse::error::FlagError;
use std::io::BufReader;

pub fn get_file(c: &Context, string_flag: &str) -> File {
    match c.string_flag(string_flag) {
        Ok(path) => {
            match Path::new(&path).exists() {
                true => {
                    File::open(path).unwrap()
                }
                false => {
                    eprintln!("Source file: '{}' does not exists!", path);
                    std::process::exit(0);
                }
            }
        }
        Err(e) => match e {
            FlagError::Undefined => panic!("undefined operator..."),
            FlagError::ArgumentError => panic!("argument error..."),
            FlagError::NotFound => panic!("not found flag..."),
            FlagError::ValueTypeError => panic!("value type mismatch..."),
            FlagError::TypeError => panic!("flag type mismatch..."),
        },
    }
}

pub fn create_output_file(c: &Context) -> File {
    match c.string_flag("output") {
        Ok(path) => {
            match File::create(&path) {
                Ok(file) => {
                    file
                }
                Err(_) => {
                    eprintln!("Could not create output file: '{}'!", path);
                    std::process::exit(0);
                }
            }
        }
        Err(e) => match e {
            FlagError::Undefined => panic!("undefined operator..."),
            FlagError::ArgumentError => panic!("argument error..."),
            FlagError::NotFound => panic!("not found flag..."),
            FlagError::ValueTypeError => panic!("value type mismatch..."),
            FlagError::TypeError => panic!("flag type mismatch..."),
        },
    }
}

pub fn get_threshold(c: &Context) -> usize {
    match c.string_flag("threshold") {
        Ok(threshold) => {
            match threshold.parse::<usize>() {
                Ok(threshold) => {
                    threshold
                }
                Err(_) => {
                    eprintln!("Could not convert {}", threshold);
                    std::process::exit(0);
                }
            }
        }
        Err(e) => match e {
            FlagError::Undefined => panic!("undefined operator..."),
            FlagError::ArgumentError => panic!("argument error..."),
            FlagError::NotFound => panic!("not found flag..."),
            FlagError::ValueTypeError => panic!("value type mismatch..."),
            FlagError::TypeError => panic!("flag type mismatch..."),
        },
    }
}

pub fn get_mappings_file(c: &Context) -> Option<(Option<File>, String)> {
    match c.string_flag("mappings") {
        Ok(path) => {
            let mut file: Option<File>;

            match Path::new(&path).exists() {
                true => {
                    file = Some(File::open(&path).unwrap());
                }
                false => {
                    file = None;
                }
            }

            Some((file, path))
        }
        Err(e) => match e {
            FlagError::Undefined => panic!("undefined operator..."),
            FlagError::ArgumentError => panic!("argument error..."),
            FlagError::NotFound => {
                None
            },
            FlagError::ValueTypeError => panic!("value type mismatch..."),
            FlagError::TypeError => panic!("flag type mismatch..."),
        },
    }
}
