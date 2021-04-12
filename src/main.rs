mod cli_parser;
mod configuration;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod cli_tests;

fn main() {
    // A example config to test the implementation
    let config = configuration::ConfStruct {
        exclude_patterns : vec![".git".to_string()],
        source : "./".to_string(),
        destination : "./dest".to_string(),
        help : false,
    };
    let files_in_source = fs::read_dir(&config.source).unwrap();

    println!("Files in source folder: {}", config.source);

    let mut path_wo_excludes : Vec<PathBuf> = Vec::new();

    // Check if any paths are in the exclude_patterns and remove them if they are
    // TODO: Handle when destination is in source
    for path_result in files_in_source {
        match path_result {
            Err(why) => panic!("Something bad happened: {}", why),
            Ok(path) => {
                let p = path.file_name().into_string().unwrap();
                if !config.exclude_patterns.contains(&p) {
                    path_wo_excludes.push(path.path());
                }
            },
        };
    }

    println!("Final file list without excludes: {:?}", path_wo_excludes);
}
