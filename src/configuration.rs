use regex::RegexSet;
use std::path::PathBuf;

#[derive(PartialEq)]
pub enum Operation {
    None,
    Backup,
    Restore,
}

/// This struct holds all the info neccessary to run a backup or restore routine
pub struct ConfStruct {
    pub exclude_strings: Vec<String>,
    pub exclude_regex: RegexSet,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub verbose: bool,
    pub operation: Operation,
}

impl ConfStruct {
    pub fn new(
        exclude_strings: Vec<String>,
        exclude_regex: RegexSet,
        source: PathBuf,
        destination: PathBuf,
        verbose: bool,
        operation: Operation,
    ) -> Self {
        ConfStruct {
            exclude_strings,
            exclude_regex,
            source,
            destination,
            verbose,
            operation,
        }
    }
}
