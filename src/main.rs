mod cli_parser;
mod backup;
mod configuration;
mod path_filter;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod cli_tests;

fn main() {
    // A example config to test the implementation
    let config = configuration::ConfStruct {
        exclude_patterns: vec![".git".to_string(), "dest".to_string()],
        source: "./".to_string(),
        destination: "./dest".to_string(),
        help: false,
    };
    let files_in_source = fs::read_dir(&config.source).unwrap();

    println!("Files in source folder: {}", config.source);

    let path_wo_excludes: Vec<PathBuf> = path_filter::filter_paths(files_in_source, &config);

    println!("Preliminary file list without excludes: {:?}", path_wo_excludes);

    let dest_folder = std::path::Path::new(&config.destination);

    // Backup every found file
    for path in path_wo_excludes {
        backup::backup(&path, &dest_folder).unwrap();
    }
}
