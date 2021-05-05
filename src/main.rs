extern crate cdchunking;
extern crate blake3;
extern crate serde;
extern crate serde_json;

mod backup;
mod cli_parser;
mod configuration;
mod path_filter;
mod chunked_writer;
mod ChunkedFile;
mod index_manager;
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
