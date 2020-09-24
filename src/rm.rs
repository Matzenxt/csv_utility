use seahorse::Context;
use std::fs::File;
use csv::{ReaderBuilder, Error, Writer, Reader, StringRecord};
use std::path::Path;
use std::num::ParseIntError;

pub fn remove_empty_rows(c: &Context) {
    // TODO: Impl.
    println!("Remove empty rows");
}

pub fn remove_rows_with_threshold(c: &Context) {

    if c.args.len() != 3 {
        eprintln!("Please check your arguments");
        std::process::exit(0);
    }

    let source_file: File = match Path::new(c.args.get(0).unwrap()).exists() {
        true => {
            File::open(c.args.get(0).unwrap()).unwrap()
        }
        false => {
            eprintln!("Source file: '{}' does not exists!", c.args.get(0).unwrap());
            std::process::exit(0);
        }
    };

    let output_file: File = match File::create(c.args.get(1).unwrap()) {
        Ok(file) => {
            file
        }
        Err(_) => {
            eprintln!("Could not create output file: '{}'!", c.args.get(0).unwrap());
            std::process::exit(0);
        }
    };

    let threshold: usize = match c.args.get(2).unwrap().parse::<usize>() {
        Ok(threshold) => {
            threshold
        }
        Err(_) => {
            eprintln!("Could not parse threshold '{}'!", c.args.get(0).unwrap());
            std::process::exit(0);
        }
    };

    let mut writer: Writer<File> = csv::Writer::from_writer(output_file);
    let mut reader: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file);

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        let mut counter: usize = 0;

        for i in 0..record.len() {
            if !record[i].is_empty() {
                counter = counter + 1;
            }
        }

        if counter > threshold {
            writer.write_record(&record);
        }
    }

    writer.flush();
}
