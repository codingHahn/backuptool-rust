use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::chunked_file::ChunkedFile;
use serde_json;

// TODO: mmap file so that updating a file does not require rewriting the whole file
// This could be done using https://docs.rs/mmap-storage

/// Reads a file from `path` and tries to extract a `Vec<ChunkedFile>`
pub fn read_index_file(path: &Path) -> Result<Vec<ChunkedFile>, io::Error> {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        let u: Vec<ChunkedFile> = serde_json::from_reader(reader).unwrap();
        return Ok(u);
    } else {
        let ret: Vec<ChunkedFile> = Vec::new();
        Ok(ret)
    }
}

/// Writes a current `Vec<ChunkedFile>` to a path specified by `path`
pub fn write_index_file(chunks: &Vec<ChunkedFile>, path: &Path) -> Result<(), io::Error> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, &chunks)?;
    Ok(())
}
