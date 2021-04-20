use crate::configuration::ConfStruct;
use regex;
use std::fs::ReadDir;
use std::path::PathBuf;

/// Returns a vector of all paths that aren't excluded by conf.exclude_strings
pub fn filter_paths(paths: ReadDir, conf: &ConfStruct) -> Vec<PathBuf> {
    // TODO: Handle when destination is in source
    let mut result: Vec<PathBuf> = Vec::new();
    for path in paths {
        match path {
            Err(why) => panic!("Something bad happened: {}", why),
            Ok(path) => {
                let p = path.file_name().into_string().unwrap();
                if !conf.exclude_strings.contains(&p) && !conf.exclude_regex.is_match(&p) {
                    result.push(path.path());
                }
            }
        };
    }
    result
}
