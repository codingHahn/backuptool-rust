extern crate blake3;
extern crate cdchunking;
extern crate serde;
extern crate serde_json;

mod ChunkedFile;
mod backup;
mod chunked_writer;
mod cli_parser;
mod configuration;
mod index_manager;
mod path_filter;
use std::env;
use std::path::Path;

#[cfg(test)]
mod cli_tests;

fn main() {
    // A example config to test the implementation
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::parse_options(args).unwrap();

    // Call backup
    backup::backup(&config.source, &config.destination, &config).unwrap();

    // Test index
    let ret = index_manager::read_index_file(Path::new("index.index"));
    println!("The returned thing was: {:?}", ret);
}
