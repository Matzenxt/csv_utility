use seahorse::Context;
use console::{Term, Style};
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::Select;
use std::fs::File;
use crate::util::{get_source_file, create_output_file};
use csv::{Reader, Writer, ReaderBuilder, StringRecord};

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

    // Read dest header to vec
    let mut dest_headers: Vec<String> = Vec::new();
    let headers = reader_dest.headers().unwrap();
    for entry in headers {
        dest_headers.push(entry.to_string());
    }

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
                map_view(&term, &theme);
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

fn map_view(term: &Term, theme: &ColorfulTheme) {
    loop {
        term.clear_screen();

        // TODO: Create items from 'new header' file

        let next_menu = Select::with_theme(theme)
            .with_prompt("Map headers")
            .default(0)
            .item("Back")
            .interact().unwrap();

        match next_menu {
            0 => {
                break
            },
            _ => {
                // TODO: Go to next menu where user can select header item from 'source file' he wants to map to selected header item
            }
        }
    }
}

fn get_headers_from_file(reader: Reader<File>) -> Vec<String> {

}
