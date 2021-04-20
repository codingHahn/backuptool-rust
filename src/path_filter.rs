use crate::configuration::ConfStruct;
use std::path::Path;

/// Returns if a path is considered `filtered` by looking at info from the `ConfStruct`
pub fn is_filtered(path: &Path, conf: &ConfStruct) -> bool {
    // TODO: Handle when destination is in source
    let p = path.strip_prefix("./").unwrap().display().to_string();
    println!("The path to filter: {}", p);
    if !conf.exclude_strings.contains(&p) && !conf.exclude_regex.is_match(&p) {
        return false;
    }
    true
}
