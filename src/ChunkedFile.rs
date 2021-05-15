use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

use crate::configuration::ConfStruct;
use cdchunking::{Chunker, ZPAQ};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChunkedFile {
    //TODO Permissions and other Metadata
    pub chunks: Vec<String>,
    pub path: PathBuf,
    pub last_modified: u128,
    pub created: u128,
}

impl ChunkedFile {
    pub fn new() -> Self {
        ChunkedFile {
            chunks: Vec::new(),
            path: PathBuf::new(),
            last_modified: 0,
            created: 0,
        }
    }
    pub fn from_path(path: PathBuf, conf: &ConfStruct) -> Result<Self, io::Error> {
        let file = File::open(&path)?;
        let mut chunked_file = ChunkedFile::new();
        chunked_file.path = path;
        chunked_file.last_modified = file
            .metadata()?
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_millis();
        chunked_file.created = file
            .metadata()?
            .created()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_millis();

        let chunker = Chunker::new(ZPAQ::new(14));
        let chunks: Vec<Vec<u8>> = chunker.all_chunks(file)?;
        for chunk in chunks {
            let hash = blake3::hash(&chunk);
            println!("Here is the chunk hash {:?}", hash);
            let dest_path = &conf
                .destination
                .join("chunks/")
                .join(&hash.to_hex().to_string());
            println!("Here is the path: {:?}", dest_path);
            let mut fp = File::create(dest_path)?;
            fp.write_all(&chunk)?;
            chunked_file.chunks.push(hash.to_hex().to_string());
        }

        Ok(chunked_file)
    }

    pub fn to_bytes(self, conf: &ConfStruct) -> Result<Vec<u8>, io::Error>{

        let mut big_vec : Vec<u8> = Vec::new();

        for chunk_hash in self.chunks {
            // Convert Hash to String
            let chunk_path = &conf.destination.join("chunks/")
                .join(&chunk_hash);

            println!("Chunked file path: {:?}", chunk_path);

            // try to open file with hash as name
            let mut chunk_file = File::open(chunk_path)?;
            // try to read file with hash as name
            let mut chunk_data : Vec<u8> = Vec::new();
            chunk_file.read_to_end(&mut chunk_data)?;

            // append all chunks to one file
            big_vec.extend(chunk_data);
        } 
        // write that file to self.path (relative?)
        Ok(big_vec)
    }
}
