use crate::configuration::ConfStruct;
use crate::path_filter;
use crate::ChunkedFile::ChunkedFile;
use std::fs;
use std::io;
use std::path::Path;

// TODO: Test

/// Copies all files from `src` to `dest_folder` while considering the excludes
/// set in the `conf` struct
pub fn backup(
    src: &Path,
    repo_path: &Path,
    chunked_files: &mut Vec<ChunkedFile>,
    conf: &ConfStruct,
) -> Result<(), io::Error> {
    // Early return here, if path is in exclude_regex or exclude_strings
    // If it passes this step, the path will be processed
    if path_filter::is_filtered(&src, &conf) {
        return Ok(());
    }

    // Make sure the `dest_folder` exists
    let res = fs::create_dir(&repo_path);
    match res {
        // We need to explicitly allow this case, because a backup folder will
        // be created once and written into many times
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e),
        Ok(()) => (),
    }

    let res = fs::create_dir(&repo_path.join("chunks/"));
    match res {
        // We need to explicitly allow this case, because a backup folder will
        // be created once and written into many times
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e),
        Ok(()) => (),
    }

    // We need to handle when `path` is a folder, because then we need to recursively copy
    // the file tree
    if !src.is_dir() {
        chunked_files.push(ChunkedFile::from_path(src.to_path_buf(), &repo_path)?);
    } else {
        let files_in_folder = fs::read_dir(src)?;
        println!("Folder detected! Copying recursively");
        println!("Contents found: {:?}", files_in_folder);
        for file in files_in_folder {
            if let Ok(f) = file {
                if let Err(why) = backup(&f.path(), &repo_path, chunked_files, &conf) {
                    eprintln!("Backing up {:?} failed, because of {:?}", f, why);
                };
            };
        }
    }

    return Ok(());
}

/*
pub fn restore(path: &Path, &conf: ConfStruct){
    let file_dest = dest_folder.join(src);
    println!("{:?} will be copied to {:?}", src, file_dest);

    // Checks if we encountered the backup folder
    if are_equal(&src, &dest_folder) == true {
        println!("Source and destination are the same; skipping");
        return Ok(());
    }




}
*/

/// Check if `src` and `file_dest` are the same. To do that, we
/// convert both paths to absolute paths. FIXME: Due to symlinking and other
/// shenanigans, this is not exhaustive, but should be good enough for now
fn are_equal(path1: &Path, path2: &Path) -> bool {
    let source_full = fs::canonicalize(&path1).unwrap();
    let dest_full = std::env::current_dir().unwrap().join(&path2);
    if source_full == dest_full {
        // We have to abort here, otherwise we will infinitely copy to and from dest,
        // because we will write some stuff in there, look inside and find the files we
        // have written just now and write them again to the directory
        return true;
    }
    false
}
