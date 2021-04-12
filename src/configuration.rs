use regex::RegexSet;
use std::path::PathBuf;

pub struct ConfStruct {
    pub exclude_clearname: Vec<String>,
    pub exclude_regex: RegexSet,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub help: bool,
}
