mod backup;
mod cli_parser;
mod configuration;
mod path_filter;
use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod cli_tests;

fn main() {
    // A example config to test the implementation
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::parse_options(args).unwrap();
    let files_in_source = fs::read_dir(&config.source).unwrap();

    println!("Files in source folder: {:?}", config.source);

    let path_wo_excludes: Vec<PathBuf> = path_filter::filter_paths(files_in_source, &config);

    println!(
        "Preliminary file list without excludes: {:?}",
        path_wo_excludes
    );

    // Backup every found file
    for path in path_wo_excludes {
        backup::backup(&path, &config.destination).unwrap();
    }
}
