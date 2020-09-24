mod map;
mod rm;
mod stats;

use seahorse::{App, Command, Context};
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
        .usage("Maps entries from source file to header from header file and saves to output file [source file] [new header] [outputfile]")
        .action(map::map_csv)
}

fn rm_empty_rows_command() -> Command {
    Command::new("rmer")
        .alias("rer")
        .usage("Removes empty rows from csv file [input file] [outputfile]")
        .action(rm::remove_empty_rows)
}

fn rm_rows_with_threshold_command() -> Command {
    Command::new("rmwt")
        .alias("rrwt")
        .usage("Remove rows with less than 'x' entries [source file] [outputfile] [x]")
        .action(rm::remove_rows_with_threshold)
}

fn stats_show_command() -> Command {
    Command::new("stats")
        .alias("s")
        .usage("Shows some stats about the content of the csv file [source file]")
        .action(stats::show_stats)
}
