use std::path::PathBuf;
use std::fs::File;
use std::time::{UNIX_EPOCH, Duration};
use std::io;
use std::io::Write;

use serde::{Deserialize, Serialize};
use crate::configuration::ConfStruct;
use cdchunking::{Chunker, ZPAQ};

#[derive(Serialize, Deserialize)]
pub struct ChunkedFile {
    //TODO Permissions and other Metadata
    pub chunks: Vec<[u8; 32]>,
    pub path: PathBuf,
    pub last_modified: u128,
    pub created: u128,
}

impl ChunkedFile {
    pub fn new() -> Self {
        ChunkedFile { chunks: Vec::new(), path: PathBuf::new(), last_modified: 0, created: 0}
    }
    pub fn from_path(path: PathBuf, conf: &ConfStruct) -> Result<Self, io::Error> {
        let file = File::open(&path)?;
        let mut chunked_file = ChunkedFile::new();
        chunked_file.path = path;
        chunked_file.last_modified = file.metadata()?.modified()?.duration_since(UNIX_EPOCH).unwrap_or(Duration::new(0,0)).as_millis();
        chunked_file.created = file.metadata()?.created()?.duration_since(UNIX_EPOCH).unwrap_or(Duration::new(0,0)).as_millis();

        let chunker = Chunker::new(ZPAQ::new(14));
        let chunks: Vec<Vec<u8>> = chunker.all_chunks(file)?;
        for chunk in chunks {
            let hash = blake3::hash(&chunk);
            println!("Here is the chunk hash {:?}", hash);
            let dest_path = &conf.destination.join("chunks/").join(&hash.to_hex().to_string()); 
            println!("Here is the path: {:?}", dest_path);
            let mut fp = File::create(dest_path)?;
            fp.write_all(&chunk)?;
            chunked_file.chunks.push(hash.as_bytes().to_owned());
        }

        Ok(chunked_file)
    }
}
