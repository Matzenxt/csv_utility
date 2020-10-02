use seahorse::Context;
use console::{Term, Style};
use dialoguer::theme::{ColorfulTheme};
use dialoguer::Select;
use std::fs::File;
use crate::util::{get_source_file, create_output_file};
use csv::{Reader, Writer, ReaderBuilder, StringRecord};

use crate::mapper::data::{HeaderEntry, Map};
use std::borrow::{BorrowMut};

pub fn main(c: &Context) {
    if c.args.len() != 3 {
        eprintln!("Please check your arguments");
        std::process::exit(0);
    }

    let source_file: File = get_source_file(c, 0);
    let dest_file: File = get_source_file(c, 1);
    let output_file: File = create_output_file(c, 2);

    let mut reader_source: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(source_file);
    let mut reader_dest: Reader<File> = ReaderBuilder::new().delimiter(b';').from_reader(dest_file);
    let mut writer_output: Writer<File> = csv::Writer::from_writer(output_file);

    // Read headers to vectors
    let dest_headers: Vec<String> = get_headers_from_file(reader_dest.headers().unwrap());
    let source_headers: Vec<String> = get_headers_from_file(reader_source.headers().unwrap());

    // Build header map
    let mut header_mappings: Vec<Map> = Vec::new();
    for (index, dest_header) in dest_headers.clone().iter().enumerate() {
        header_mappings.push(Map::new(index, dest_header.to_string()));
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
            .item("Cancel")
            .item("Save and exit")
            .interact().unwrap();

        match next_menu {
            0 => {
                map_view(&term, &theme, header_mappings.borrow_mut(), &source_headers);
            },
            1 => {
                term.clear_screen();
                std::process::exit(0);
            },
            2 => {
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

fn map_view(term: &Term, theme: &ColorfulTheme, header_mappings: &mut Vec<Map>, header_source: &Vec<String>) {
    loop {
        term.clear_screen();

        // TODO: Set default pos of course to first empty entry?
        // TODO: Add feature to save and load mapping?

        let next_menu = Select::with_theme(theme)
            .with_prompt("Mapping")
            .default(0)
            .item("Back")
            .items(header_mappings)
            .interact()
            .unwrap();

        match next_menu {
            0 => {
                break
            },
            _ => {
                // -1 because back entry in item list
                let position_dest = next_menu - 1;

                match item_selector(&term, &theme, header_source, &header_mappings[position_dest].dest_entry.name) {
                    0 => {
                        // Do nothing
                    },
                    1 => {
                        header_mappings[position_dest].set_source_entry(None);
                    },
                    pos_source => {
                        // -2 because empty and back entry in item list
                        let position_source = pos_source - 2;

                        header_mappings[position_dest].set_source_entry(Option::from(HeaderEntry {
                            name: header_source[position_source].clone(),
                            position: position_source
                        }))
                    }
                };
            }
        }
    }
}

fn item_selector(term: &Term, theme: &ColorfulTheme, header_source: &Vec<String>, prompt_text: &String) -> usize {
    loop {
        term.clear_screen();

        let next_menu = Select::with_theme(theme)
            .with_prompt(format!("Map to {}", prompt_text))
            .default(0)
            .item("Back")
            .item("Empty")
            .items(header_source)
            .interact()
            .unwrap();

        return next_menu;
    }
}

fn get_headers_from_file(headers: &StringRecord) -> Vec<String> {
    let mut temp_header: Vec<String> = Vec::new();
    for entry in headers {
        temp_header.push(entry.to_string());
    }

    temp_header
}

fn save_mapped_to_file(mut source: Reader<File>, mut dest: Reader<File>, mut output: Writer<File>, header_map: &Vec<Map>) {
    // Write header to output file
    output.write_record(dest.headers().unwrap());

    // Write entries
    for source_record in source.records() {
        let record: StringRecord = source_record.unwrap();

        let mut new_record: StringRecord = StringRecord::new();

        // Build new mapped record
        for entry in header_map {
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
