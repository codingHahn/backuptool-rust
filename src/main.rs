extern crate blake3;
extern crate cdchunking;
extern crate serde;
extern crate serde_json;

mod backup;
mod chunked_file;
mod cli_parser;
mod configuration;
mod index_manager;
mod path_filter;
use std::env;
use std::path::Path;
use std::process;

#[cfg(test)]
mod cli_tests;

fn main() {
    // Get the configuration for this run of the program
    let args: Vec<String> = env::args().collect();
    let c = cli_parser::parse_options(args);

    // Placeholder Vec<ChunkedFile>
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

                    // After backup up all files, update the index file
                    // This is done at the end because it is expensive
                    // and an error in the backup routine can't destroy
                    // the index file
                    index_manager::write_index_file(
                        &chunked_files,
                        &config.destination.join("index.index"),
                    )
                    .unwrap();
                }

                // User selected restore as option
                configuration::Operation::Restore => {
                    let index = index_manager::read_index_file(
                        &config.source.join(Path::new("index.index")),
                    )
                    .unwrap();

                    if let Err(err) = backup::restore(&config, &index) {
                        println!("Error during restoring files: {}", err);
                    }
                }
                // This case is handled by cli_parser. This should never execute
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
