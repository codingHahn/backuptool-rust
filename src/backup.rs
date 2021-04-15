use std::fs;
use std::path::Path;

// TODO: Test
pub fn backup(src: &Path, dest_folder: &Path) -> std::io::Result<()> {
    let file_dest = dest_folder.join(src);
    println!("{:?} will be copied to {:?}", src, file_dest);

    // Check if `src` and `file_dest` are the same
    if src == file_dest{
        println!("Source and destination are the same; skipping");
        return Ok(());
    }

    // Make sure the `dest_folder` exists
    let _ = fs::create_dir(&dest_folder);

    // We need to handle when `path` is a folder, because then we need to recursively copy
    // the file tree
    if !src.is_dir() {
        if file_dest.exists() {
            // If it already exists, we need to compare last edited timestamps
            println!("File {:?} already exists at destination", file_dest);

            if file_dest.metadata()?.modified()? < src.metadata()?.modified()? {
                println!("The file was modified since last backup");
                fs::copy(src, file_dest)?;
            }
        } else {
            fs::copy(src, file_dest)?;
        }
    } else {
        let files_in_folder = fs::read_dir(src)?;
        println!("Folder detected! Copying recursively");
        println!("Contents found: {:?}", files_in_folder);
        let exists_folder = fs::create_dir(&file_dest);
        match exists_folder {
            Err(_) => println!("Folder {:?} already exists", file_dest),
            _ => (),
        };

        // TODO: Need to apply exclude_regex once implemented
        for file in files_in_folder {
            if let Ok(f) = file {
                backup(&f.path(), &dest_folder)?;
            };
        }
    }

    return Ok(());
}

