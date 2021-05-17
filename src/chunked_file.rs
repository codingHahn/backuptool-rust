use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use cdchunking::{Chunker, ZPAQ};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct ChunkedFile {
    //TODO Permissions and other Metadata
    pub chunks: Vec<String>,
    pub path: PathBuf,
    pub last_modified: u128,
    pub created: u128,
}

impl ChunkedFile {
    /// Creates new instance of `ChunkedFile`
    pub fn new() -> Self {
        ChunkedFile {
            chunks: Vec::new(),
            path: PathBuf::new(),
            last_modified: 0,
            created: 0,
        }
    }

    /// Constructs a `ChunkedFile` by reading and chunking the file at `path`
    pub fn from_path(path: PathBuf, repo_path: &Path) -> Result<Self, io::Error> {
        let file = File::open(&path)?;
        let mut chunked_file = ChunkedFile::new();
        chunked_file.path = path;

        // Read the metadata that's currently supported
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

        // Create a chunker with 16KiB block size
        let chunker = Chunker::new(ZPAQ::new(14));
        let chunks: Vec<Vec<u8>> = chunker.all_chunks(file)?;

        // Hash and save all chunks individually
        for chunk in chunks {
            let hash = blake3::hash(&chunk);
            let dest_path = &repo_path.join("chunks/").join(&hash.to_hex().to_string());
            let mut fp = File::create(dest_path)?;
            fp.write_all(&chunk)?;
            chunked_file.chunks.push(hash.to_hex().to_string());
        }

        Ok(chunked_file)
    }

    /// Returns the raw bytes of the file represented by `ChunkedFile` by iterating
    /// over it's chunks, reading them from the disk an then putting them together
    pub fn to_bytes(&self, repo_path: &Path) -> Result<Vec<u8>, io::Error> {
        // Will hold the whole file at once
        let mut big_vec: Vec<u8> = Vec::new();

        // TODO: set right metadata
        for chunk_hash in &self.chunks {
            let chunk_path = &repo_path.join("chunks/").join(&chunk_hash);

            // Open the chunk. If it doesn't exist, something's wrong
            let mut chunk_file = File::open(chunk_path)?;

            // Temporary buffer for the reader to write into
            let mut chunk_data: Vec<u8> = Vec::new();
            chunk_file.read_to_end(&mut chunk_data)?;

            // Merge temporary buffer into main file buffer
            big_vec.extend(chunk_data);
        }
        Ok(big_vec)
    }
}
