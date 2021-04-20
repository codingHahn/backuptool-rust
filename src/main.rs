mod backup;
mod cli_parser;
mod configuration;
mod path_filter;
use std::env;

#[cfg(test)]
mod cli_tests;

fn main() {
    // A example config to test the implementation
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::parse_options(args).unwrap();

    // Call backup
    backup::backup(&config.source, &config.destination, &config).unwrap();
}
