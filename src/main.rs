extern crate blake3;
extern crate cdchunking;
extern crate serde;
extern crate serde_json;

mod chunked_file;
mod backup;
mod cli_parser;
mod configuration;
mod index_manager;
mod path_filter;
use std::env;
use std::process;

#[cfg(test)]
mod cli_tests;

fn main() {
    // A example config to test the implementation
    let args: Vec<String> = env::args().collect();
    let c = cli_parser::parse_options(args);
    let mut chunked_files = Vec::new();
    match c {
        Ok(config) => {
            match config.operation {
                // User selected backup as option
                configuration::Operation::Backup => {
                    backup::backup(
                        &config.source,
                        &config.destination,
                        &mut chunked_files,
                        &config,
                    )
                    .unwrap();
                    index_manager::write_index_file(
                        &chunked_files,
                        &config.destination.join("index.index"),
                    )
                    .unwrap();
                }
                // User selected restore as option
                configuration::Operation::Restore => {
                    backup::restore(&config);
                }
                configuration::Operation::None => {
                    process::exit(-1);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            process::exit(-1);
        }
    }

    // Call backup
}
