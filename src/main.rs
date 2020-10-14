mod rm;
mod stats;
mod util;
mod mapper;

use seahorse::{App, Command, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args.get(0).unwrap()))
        .command(map_command())
        .command(rm_empty_rows_command())
        .command(rm_rows_with_threshold_command())
        .command(stats_show_command());

    app.run(args);
}

fn map_command() -> Command {
    Command::new("map")
        .alias("m")
        .usage("Maps entries from source file to header from header file and saves to output file")
        .action(mapper::menu::main)
        .flag(source_file_flag())
        .flag(destination_file_flag())
        .flag(output_file_flag())
}

fn rm_empty_rows_command() -> Command {
    Command::new("rmer")
        .alias("rer")
        .usage("Removes empty rows from csv file")
        .action(rm::remove_empty_rows)
        .flag(source_file_flag())
        .flag(output_file_flag())
}

fn rm_rows_with_threshold_command() -> Command {
    Command::new("rmwt")
        .alias("rrwt")
        .usage("Remove rows with less than 'x' entries")
        .action(rm::remove_rows_with_threshold)
        .flag(source_file_flag())
        .flag(output_file_flag())
        .flag(Flag::new("threshold", FlagType::String)
            .usage("cli threshold --threshold(-t)")
            .alias("t")
        )
}

fn stats_show_command() -> Command {
    Command::new("stats")
        .alias("s")
        .usage("Shows some stats about the content of the csv file")
        .action(stats::show_stats)
        .flag(source_file_flag())
}

fn source_file_flag() -> Flag {
    Flag::new("source", FlagType::String)
        .usage("cli source file path --source(-s)")
        .alias("s")
}

fn output_file_flag() -> Flag {
    Flag::new("output", FlagType::String)
        .usage("cli output file path --output(-o)")
        .alias("o")
}

fn destination_file_flag() -> Flag {
    Flag::new("destination", FlagType::String)
        .usage("cli destination file path --destination(-d)")
        .alias("d")
}
