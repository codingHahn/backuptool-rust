extern crate blake3;
extern crate cdchunking;
extern crate serde;
extern crate serde_json;

mod ChunkedFile;
mod backup;
mod cli_parser;
mod configuration;
mod index_manager;
mod path_filter;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
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
                configuration::Operation::Restore => {
                    let ret = index_manager::read_index_file(
                        &config.source.join(Path::new("index.index")),
                    )
                    .unwrap();
                    println!("The returned thing was: {:?}", ret);
                    for i in ret {
                        if let Ok(bytes) = &i.to_bytes(&config.source) {
                            println!("Big file dump: {:?}", bytes);
                            let path = &config.destination.join(&i.path);
                            let res = fs::create_dir_all(&path.parent().unwrap());
                            match res {
                                // We need to explicitly allow this case, because a backup folder will
                                // be created once and written into many times
                                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
                                Err(e) => panic!("{}", e),
                                Ok(()) => (),
                            }
                            let mut file = File::create(&config.destination.join(&i.path)).unwrap();
                            if let Err(err) = file.write_all(&bytes) {
                                println!(
                                    "Could not restore File at {:?} because of error {}",
                                    i.path, err
                                );
                            }
                        }
                    }
                }
                configuration::Operation::None => {
                    process::exit(-1);
                }
            }

            // Test index
        }
        Err(err) => {
            println!("{}", err);
            process::exit(-1);
        }
    }

    // Call backup
}
