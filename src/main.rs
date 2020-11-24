mod rm;
mod stats;
mod util;
mod mapper;
mod append;

use seahorse::{App, Command, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .description("csv_utility command [flags]")
        .action(|c| println!("TODO"))
        .command(map_command())
        .command(rm_empty_rows_command())
        .command(rm_rows_with_threshold_command())
        .command(append_command())
        .command(stats_show_command())
        .flag(source_file_flag())
        .flag(destination_file_flag())
        .flag(output_file_flag())
        .flag(mappings_file_flag())
        .flag(threshold_flag());

    app.run(args);
}

fn map_command() -> Command {
    Command::new("map")
        .alias("m")
        .description("Maps entries from source file to header from header file and saves to output file")
        .action(mapper::menu::main)
        .flag(source_file_flag())
        .flag(destination_file_flag())
        .flag(output_file_flag())
        .flag(mappings_file_flag())
}

fn rm_empty_rows_command() -> Command {
    Command::new("rmer")
        .alias("rer")
        .description("Removes empty rows from source csv file and saves to output file")
        .action(rm::remove_empty_rows)
        .flag(source_file_flag())
        .flag(output_file_flag())
}

fn rm_rows_with_threshold_command() -> Command {
    Command::new("rmwt")
        .alias("rrwt")
        .description("Remove rows with less than --threshold entries from source csv file and saves to output file")
        .action(rm::remove_rows_with_threshold)
        .flag(source_file_flag())
        .flag(output_file_flag())
        .flag(threshold_flag())
}

fn append_command() -> Command {
    Command::new("append")
        .alias("a")
        .description("Appends two csv files with the same header line")
        .action(append::append)
        .flag(source_file_flag())
        .flag(destination_file_flag())
        .flag(output_file_flag())
}

fn stats_show_command() -> Command {
    Command::new("stats")
        .alias("s")
        .description("Shows some stats about the content of the csv file")
        .action(stats::show_stats)
        .flag(source_file_flag())
}

fn source_file_flag() -> Flag {
    Flag::new("source", FlagType::String)
        .description("path to source file")
        .alias("s")
}

fn output_file_flag() -> Flag {
    Flag::new("output", FlagType::String)
        .description("path to output file")
        .alias("o")
}

fn destination_file_flag() -> Flag {
    Flag::new("destination", FlagType::String)
        .description("path to destination file")
        .alias("d")
}

fn mappings_file_flag() -> Flag {
    Flag::new("mappings", FlagType::String)
        .description("path to mappings file")
        .alias("m")
}

fn threshold_flag() -> Flag {
    Flag::new("threshold", FlagType::String)
        .description("threshold, a positiv number")
        .alias("t")
}
