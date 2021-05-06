use regex::RegexSet;
use std::path::PathBuf;

pub struct ConfStruct {
    pub exclude_strings: Vec<String>,
    pub exclude_regex: RegexSet,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub verbose: bool,
}

impl ConfStruct {
    pub fn new(
        exclude_strings: Vec<String>,
        exclude_regex: RegexSet,
        source: PathBuf,
        destination: PathBuf,
        verbose: bool,
    ) -> Self {
        ConfStruct {
            exclude_strings: exclude_strings,
            exclude_regex: exclude_regex,
            source: source,
            destination: destination,
            verbose: verbose,
        }
    }
}
