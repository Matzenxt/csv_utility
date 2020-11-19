use std::fs::File;
use std::path::Path;
use seahorse::Context;
use seahorse::error::FlagError;
use std::io::BufReader;
use crate::mapper::data::Mappings;

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

pub fn get_mappings_file(c: &Context) -> Option<(Option<Mappings>, String)> {
    match c.string_flag("mappings") {
        Ok(path) => {
            let mut header_mappings: Option<Mappings>;

            match Path::new(&path).exists() {
                true => {
                    let mapping_file: File = File::open(&path).unwrap();
                    let mut reader_mappings:BufReader<File> = BufReader::new(mapping_file);
                    
                    let mut mappings_res: Result<Mappings, serde_json::Error> = serde_json::from_reader(reader_mappings);

                    match mappings_res {
                        Ok(mappings) => {
                            header_mappings = Some(mappings);
                        }
                        Err(_) => {
                            header_mappings = None
                        }
                    }
                }
                false => {
                    header_mappings = None
                }
            }

            Some((header_mappings, path))
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
