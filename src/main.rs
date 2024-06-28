use std::{io::{self, IsTerminal, Read}, process::{exit, ExitCode}};
use clap::Parser;
use cli::cli::Arguments;
use config::config::Config;
use prettytable::{format, row, Table};
use utils::utils::format_grapheme;

mod config;
mod charset;
mod utf8_groups;
mod utils;
mod cli;

fn main() -> ExitCode {
    let config = Config::build();
    let args = Arguments::parse();

    let input = match args.text.as_str() {
        "" => {
            if io::stdin().is_terminal() {
                String::new()
            } else {
                let mut input = String::new();
                io::stdin().read_to_string(&mut input).unwrap();
                input
            }
        }
        _ => args.text
    };

    dbg!(&input);

    if input.len() == 0 {
        eprintln!("Please provide some input either via argument or stdin");
        return ExitCode::FAILURE;
    }

    let graphemes = input.chars();

    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .padding(1, 1)
        .build();
    table.set_format(format);

    for grapheme in graphemes {
        format_grapheme(&mut table, &grapheme, &config.use_truecolor);
    }

    table.set_titles(row![
        "Grapheme",
        "Byte in binary",
        "Byte in decimal",
        "Unicode in decimal",
        "Unicode in hex",
        "Byte Type",
        "Unicode Group",
    ]);

    table.printstd();

    ExitCode::SUCCESS
}
