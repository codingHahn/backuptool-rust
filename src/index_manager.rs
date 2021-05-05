use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io;

use serde_json;
use crate::ChunkedFile::ChunkedFile;

// TODO: mmap file so that updating a file does not require rewriting the whole file
// This could be done using https://docs.rs/mmap-storage

/// Reads a file from `path` and tries to extract a `Vec<ChunkedFile>`
pub fn read_index_file(path: &Path) -> Result<Vec<ChunkedFile>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u : Vec<ChunkedFile> = serde_json::from_reader(reader).unwrap();
    return Ok(u);
}

/// Writes a current `Vec<ChunkedFile>` to a path specified by `path`
pub fn write_index_file(chunks: &Vec<ChunkedFile>, path: &Path) -> Result<(), io::Error> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, &chunks)?;
    Ok(())
}
