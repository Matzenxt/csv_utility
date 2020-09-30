use seahorse::Context;
use console::{Term, Style};
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::Select;
use std::fs::File;
use crate::util::{get_source_file, create_output_file};
use csv::{Reader, Writer, ReaderBuilder, StringRecord};
use std::{thread, time};

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


    let term = Term::stdout();
    term.set_title("CSV mapper");

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    loop {
        //term.clear_screen();

        let next_menu = Select::with_theme(&theme)
            .with_prompt("Choose action")
            .default(0)
            .item("Map")
            .item("Cancel")
            .item("Save and exit")
            .interact().unwrap();

        match next_menu {
            0 => {
                map_view(&term, &theme, &dest_headers, &source_headers);
            },
            1 => {
                term.clear_screen();
                std::process::exit(0);
            },
            2 => {
                // TODO: Impl save functionality
                term.clear_screen();
                std::process::exit(0);
            },
            _ => {
                term.clear_screen();
                std::process::exit(0);
            }
        }
    }
}

fn map_view(term: &Term, theme: &ColorfulTheme, header_dest: &Vec<String>, header_source: &Vec<String>) {
    loop {
        term.clear_screen();

        let next_menu = Select::with_theme(theme)
            .with_prompt("Map headers")
            .default(0)
            .item("Back")
            .items(header_dest)
            .interact()
            .unwrap();

        match next_menu {
            0 => {
                break
            },
            _ => {
                match item_selector(&term, &theme, header_source) {
                    Some(pos) => {
                        match pos {
                            1 => {
                                println!("Set to empty");
                            },
                            _ => {
                                // -2 because empty and back entry in item list
                                let position_source = pos - 2;

                                // -1 because back entry in item list
                                let position_dest = next_menu - 1;

                                println!("Map source header {} to dest header {}", header_source[position_source], header_dest[position_dest]);
                            }
                        }
                    },
                    None => {
                        // Do nothing.
                    }
                };

                thread::sleep(time::Duration::from_millis(8000));
            }
        }
    }
}

fn item_selector(term: &Term, theme: &ColorfulTheme, header_source: &Vec<String>) -> Option<usize> {
    loop {
        term.clear_screen();

        let next_menu = Select::with_theme(theme)
            .with_prompt("Map headers")
            .default(0)
            .item("Back")
            .item("Empty")
            .items(header_source)
            .interact()
            .unwrap();

        return match next_menu {
            0 => {
                None
            },
            _ => {
                Some(next_menu)
            }
        }
    }
}

fn get_headers_from_file(headers: &StringRecord) -> Vec<String> {
    let mut temp_header: Vec<String> = Vec::new();
    for entry in headers {
        temp_header.push(entry.to_string());
    }

    temp_header
}
