use seahorse::Context;
use std::fs::File;
use csv::{ReaderBuilder, Writer, Reader, StringRecord};

use crate::util::{get_file, create_output_file};

pub fn append(c: &Context) {
    let source_file_1: File = get_file(c, "source");
    let source_file_2: File = get_file(c, "source");
    let output_file: File = create_output_file(c);

    let mut reader_1: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file_1);
    let mut reader_2: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file_2);
    let mut writer: Writer<File> = csv::Writer::from_writer(output_file);

    let header_1 = reader_1.headers().unwrap();
    let header_2 = reader_2.headers().unwrap();

    if header_1.eq(header_2) {
        for record in reader_1.records() {
            let record: StringRecord = record.unwrap();

            writer.write_record(&record);
        }

        for record in reader_2.records() {
            let record: StringRecord = record.unwrap();

            writer.write_record(&record);
        }

        writer.flush();

        println!("Appended two files.")
    } else {
        println!("Did not append files, because the files do not have the same header.")
    }
}