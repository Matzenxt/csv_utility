use std::fs::File;
use std::path::Path;
use seahorse::Context;

pub fn get_source_file(c: &Context, pos_of_source_file_path: usize) -> File {
    match Path::new(c.args.get(pos_of_source_file_path).unwrap()).exists() {
        true => {
            File::open(c.args.get(pos_of_source_file_path).unwrap()).unwrap()
        }
        false => {
            eprintln!("Source file: '{}' does not exists!", c.args.get(0).unwrap());
            std::process::exit(0);
        }
    }
}

pub fn create_output_file(c: &Context, pos_of_output_file_path: usize) -> File {
    match File::create(c.args.get(pos_of_output_file_path).unwrap()) {
        Ok(file) => {
            file
        }
        Err(_) => {
            eprintln!("Could not create output file: '{}'!", c.args.get(pos_of_output_file_path).unwrap());
            std::process::exit(0);
        }
    }
}