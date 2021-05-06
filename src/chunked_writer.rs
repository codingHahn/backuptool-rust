use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

//pub fn chunker(src: &Path, dest: &Path) -> Result<(), io::Error> {
//    let chunker = Chunker::new(ZPAQ::new(14));
//    let file = File::open(&src)?;
//
//    for chunk in chunker.whole_chunks(file) {
//        let chunk = chunk.expect("Error reading from file");
//        println!("Here is the chunk hash {:?}", blake3::hash(&chunk));
//    }
//    Ok(())
//}
