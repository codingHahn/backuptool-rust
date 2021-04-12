use std::path::PathBuf;

pub struct ConfStruct {
    pub exclude_patterns: Vec<String>,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub help: bool,
}
