use seahorse::Context;
use std::fs::File;
use crate::util::{get_file};
use csv::{Reader, ReaderBuilder, StringRecord};


pub fn show_stats(c: &Context) {
    let source_file: File = get_file(c, "source");

    let mut reader: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file);

    let columns_count: usize = reader.headers().unwrap().len();
    let mut line_counter: usize = 0;
    let mut full_row_counter: usize = 0;
    let mut empty_row_counter: usize = 0;

    for record in reader.records() {
        let record: StringRecord = record.unwrap();

        let mut entity_counter: usize = 0;

        for i in 0..record.len() {
            if !record[i].is_empty() {
                entity_counter += 1;
            }
        }

        if entity_counter == 0 {
            empty_row_counter += 1;
        } else if entity_counter == columns_count {
            full_row_counter += 1;
        }

        line_counter += 1;
    }

    println!("Stats for {}", c.string_flag("source").unwrap());
    println!("  - {} columns", reader.headers().unwrap().len());
    println!("  - {} lines total", line_counter);
    println!("  - {} full lines", full_row_counter);
    println!("  - {} partly full lines", (line_counter - full_row_counter - empty_row_counter));
    println!("  - {} empty lines", empty_row_counter);
}