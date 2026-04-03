use std::{fs, path::PathBuf};

pub(super) fn create_folder_it_not_exists(folder: &PathBuf) -> color_eyre::Result<()> {
    if !folder.is_dir() {
        fs::create_dir_all(folder)?;
    }

    Ok(())
}