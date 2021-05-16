use crate::configuration::ConfStruct;
use crate::index_manager;
use crate::path_filter;
use crate::chunked_file::ChunkedFile;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
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

/// Restore all files from the repository
pub fn restore(config: &ConfStruct) -> Result<(), io::Error> {
    let index = index_manager::read_index_file(&config.source.join(Path::new("index.index")))?;
    for chunked_file in index {
        // Figure out path. The strip_prefix is needed, because joining with 
        // an absolute path just uses the absolute path. Otherwise this could end in dataloss
        let path = &config.destination.join(
            &chunked_file
                .path
                .strip_prefix("/")
                .unwrap_or(&chunked_file.path),
        );
        if let Ok(bytes) = &chunked_file.to_bytes(&config.source) {
            // Create all directories on the way. We don't save directories in
            // the ChunkedFile struct. This probably has unhandled edgecases
            let res = fs::create_dir_all(&path.parent().unwrap());
            match res {
                // We need to explicitly allow this case, because a restored folder 
                // could already exist
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
                Err(e) => return Err(e),
                Ok(()) => (),
            }
            // Get file handle
            let mut file = File::create(&config.destination.join(&chunked_file.path))?;
            if let Err(err) = file.write_all(&bytes) {
                println!(
                    "Could not restore File at {:?} because of error {}",
                    chunked_file.path, err
                );
            } else {
                println!("Restored file {:?}", path);
            }
        } else {
            // If this happens, some chunks from the file are missing. This probably means
            // that the repository is in an unsafe state. Will lead to dataloss. TODO: Implement
            // repository checksumming and health checks
            println!(
                "Could not restore File at {:?} because of it could not be hydrated",
                path
            );
        }
    }
    Ok(())
}
