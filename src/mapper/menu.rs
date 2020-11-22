use seahorse::Context;
use console::{Term, Style};
use dialoguer::theme::{ColorfulTheme};
use dialoguer::{Select, Input};
use std::fs::File;
use crate::util::{get_file, create_output_file, get_mappings_file};
use csv::{Reader, Writer, ReaderBuilder, StringRecord};

use crate::mapper::data::{HeaderEntry, Mappings};
use std::borrow::{BorrowMut, Borrow};
use std::io::BufReader;

pub fn main(c: &Context) {
    let source_file: File = get_file(c, "source");
    let dest_file: File = get_file(c, "destination");
    let output_file: File = create_output_file(c);
    let mappings_file: Option<(Option<File>, String)> = get_mappings_file(c);

    let mut reader_source: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file);
    let mut reader_dest: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(dest_file);
    let writer_output: Writer<File> = csv::Writer::from_writer(output_file);

    // Read headers to vectors
    let dest_headers: Vec<String> = get_headers_from_file(reader_dest.headers().unwrap());
    let source_headers: Vec<String> = get_headers_from_file(reader_source.headers().unwrap());

    // TODO: Check if headers of mappings machts csv files
    let mut header_mappings: Mappings;
    let mut mappings_path: Option<String>;

    // Handle header loading/creating empty header mappings
    match mappings_file {
        Some(mappings) => {
            match mappings.0 {
                Some(mapping_file) => {
                    let reader_mappings:BufReader<File> = BufReader::new(mapping_file);

                    let mappings_res: Result<Mappings, serde_json::Error> = serde_json::from_reader(reader_mappings);

                    match mappings_res {
                        Ok(mappings) => {
                            header_mappings = mappings;
                        }
                        Err(_) => {
                            header_mappings = Mappings::new(dest_headers);
                        }
                    }
                }
                None => {
                    header_mappings = Mappings::new(dest_headers);
                }
            }

            mappings_path = Some(mappings.1);
        }
        None => {
            header_mappings = Mappings::new(dest_headers);
            mappings_path = None;
        }
    }

    let term = Term::stdout();
    term.set_title("CSV mapper");

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    loop {
        term.clear_screen();

        let next_menu = Select::with_theme(&theme)
            .with_prompt("Choose action")
            .default(0)
            .item("Map")
            .item("Save mapping file")
            .item("Save as new mapping file")
            .item("Cancel")
            .item("Save and exit")
            .interact().unwrap();

        match next_menu {
            0 => {
                map_view(&term, &theme, header_mappings.borrow_mut(), &source_headers);
            },
            1 => {
                let file_path;

                match &mappings_path {
                    Some(path) => {
                        file_path = path.to_owned();
                    }
                    None => {
                        file_path = get_new_mappings_file_name(&theme);
                        mappings_path = Some(file_path.clone());
                    }
                }

                serde_json::to_writer_pretty(&File::create(&file_path).unwrap(), &header_mappings).unwrap();
            },
            2 => {
                let new_file_path = get_new_mappings_file_name(&theme);
                mappings_path = Some(new_file_path.clone());

                serde_json::to_writer_pretty(&File::create(&new_file_path).unwrap(), &header_mappings).unwrap();
            },
            3 => {
                term.clear_screen();
                std::process::exit(0);
            },
            4 => {
                term.clear_screen();
                save_mapped_to_file(reader_source, reader_dest, writer_output, &header_mappings);
                std::process::exit(0);
            },
            _ => {
                term.clear_screen();
                std::process::exit(0);
            }
        }
    }
}

fn map_view(term: &Term, theme: &ColorfulTheme, header_mappings: &mut Mappings, header_source: &[String]) {
    loop {
        term.clear_screen();

        // TODO: Set default pos of course to first empty entry?

        let next_menu = Select::with_theme(theme)
            .with_prompt("Mapping")
            .default(0)
            .item("Back")
            .items(header_mappings.mappings.borrow())
            .interact()
            .unwrap();

        match next_menu {
            0 => {
                break
            },
            _ => {
                // -1 because back entry in item list
                let position_dest = next_menu - 1;

                match item_selector(&term, &theme, header_source, &header_mappings.mappings[position_dest].dest_entry.name) {
                    0 => {
                        // Do nothing
                    },
                    1 => {
                        header_mappings.mappings[position_dest].set_source_entry(None);
                    },
                    pos_source => {
                        // -2 because empty and back entry in item list
                        let position_source = pos_source - 2;

                        header_mappings.mappings[position_dest].set_source_entry(Option::from(HeaderEntry {
                            name: header_source[position_source].clone(),
                            position: position_source
                        }))
                    }
                };
            }
        }
    }
}

fn item_selector(term: &Term, theme: &ColorfulTheme, header_source: &[String], prompt_text: &str) -> usize {
    term.clear_screen();

    Select::with_theme(theme)
        .with_prompt(format!("Map to {}", prompt_text))
        .default(0)
        .item("Back")
        .item("Empty")
        .items(header_source)
        .interact()
        .unwrap()
}

fn get_headers_from_file(headers: &StringRecord) -> Vec<String> {
    let mut temp_header: Vec<String> = Vec::new();
    for entry in headers {
        temp_header.push(entry.to_string());
    }

    temp_header
}

fn save_mapped_to_file(mut source: Reader<File>, mut dest: Reader<File>, mut output: Writer<File>, header_mappings: &Mappings) {
    // Write header to output file
    output.write_record(dest.headers().unwrap());

    // Write entries
    for source_record in source.records() {
        let record: StringRecord = source_record.unwrap();

        let mut new_record: StringRecord = StringRecord::new();

        // Build new mapped record
        for entry in &header_mappings.mappings {
            match &entry.source_entry {
                None => {
                    new_record.push_field("");
                }
                Some(dest) => {
                    new_record.push_field(&record[dest.position]);
                }
            }
        }

        output.write_record(&new_record);
    }

    output.flush();
}

fn get_new_mappings_file_name(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
        .with_prompt("New mappings file name")
        .default("mappings.json".to_string())
        .interact()
        .unwrap()
}
