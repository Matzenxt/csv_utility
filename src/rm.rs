use seahorse::Context;
use std::fs::File;
use csv::{ReaderBuilder, Error, Writer, Reader, StringRecord};
use std::path::Path;
use std::num::ParseIntError;
use crate::util::{get_source_file, create_output_file};

pub fn remove_empty_rows(c: &Context) {
    if c.args.len() != 2 {
        eprintln!("Please check your arguments");
        std::process::exit(0);
    }

    let source_file: File = get_source_file(c, 0);

    let output_file: File = create_output_file(c, 1);

    let mut writer: Writer<File> = csv::Writer::from_writer(output_file);
    let mut reader: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file);

    let mut added_row_counter: usize = 0;
    let mut removed_row_counter: usize = 0;

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        let mut entity_counter: usize = 0;

        for i in 0..record.len() {
            if !record[i].is_empty() {
                entity_counter = entity_counter + 1;
            }
        }

        if entity_counter == 0 {
            writer.write_record(&record);
            added_row_counter = added_row_counter + 1;
        } else {
            removed_row_counter = removed_row_counter + 1;
        }
    }

    writer.flush();
    println!("Removed {} from {} lines , {} lines remaining.", removed_row_counter, (added_row_counter + removed_row_counter), added_row_counter);
}

pub fn remove_rows_with_threshold(c: &Context) {
    if c.args.len() != 3 {
        eprintln!("Please check your arguments");
        std::process::exit(0);
    }

    let source_file: File = get_source_file(c, 0);

    let output_file: File = create_output_file(c, 1);

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

    let mut added_row_counter: usize = 0;
    let mut removed_row_counter: usize = 0;

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        let mut entity_counter: usize = 0;

        for i in 0..record.len() {
            if !record[i].is_empty() {
                entity_counter = entity_counter + 1;
            }
        }

        if entity_counter > threshold {
            writer.write_record(&record);
            added_row_counter = added_row_counter + 1;
        } else {
            removed_row_counter = removed_row_counter + 1;
        }
    }

    writer.flush();
    println!("Removed {} from {} lines , {} lines remaining.", removed_row_counter, (added_row_counter + removed_row_counter), added_row_counter);
}
