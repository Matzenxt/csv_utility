use seahorse::Context;
use std::fs::File;
use csv::{ReaderBuilder, Writer, Reader, StringRecord};
use seahorse::error::FlagError;

use crate::util::{get_file, create_output_file, get_threshold};

pub fn remove_empty_rows(c: &Context) {
    let source_file: File = get_file(c, "source");

    let output_file: File = create_output_file(c);

    let mut writer: Writer<File> = csv::Writer::from_writer(output_file);
    let mut reader: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file);

    let mut added_row_counter: usize = 0;
    let mut removed_row_counter: usize = 0;

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        let mut entity_counter: usize = 0;

        for i in 0..record.len() {
            if !record[i].is_empty() {
                entity_counter += 1;
            }
        }

        if entity_counter == 0 {
            writer.write_record(&record);
            added_row_counter += 1;
        } else {
            removed_row_counter += 1;
        }
    }

    writer.flush();
    println!("Removed {} from {} lines , {} lines remaining.", removed_row_counter, (added_row_counter + removed_row_counter), added_row_counter);
}

pub fn remove_rows_with_threshold(c: &Context) {
    let source_file: File = get_file(c, "source");

    let output_file: File = create_output_file(c);

    let threshold: usize = get_threshold(c);

    let mut writer: Writer<File> = csv::Writer::from_writer(output_file);
    let mut reader: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file);

    let mut added_row_counter: usize = 0;
    let mut removed_row_counter: usize = 0;

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        let mut entity_counter: usize = 0;

        for i in 0..record.len() {
            if !record[i].is_empty() {
                entity_counter += 1;
            }
        }

        if entity_counter > threshold {
            writer.write_record(&record);
            added_row_counter += 1;
        } else {
            removed_row_counter += 1;
        }
    }

    writer.flush();
    println!("Removed {} from {} lines , {} lines remaining.", removed_row_counter, (added_row_counter + removed_row_counter), added_row_counter);
}
