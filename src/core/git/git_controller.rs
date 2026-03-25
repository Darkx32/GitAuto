use std::path::Path;

use color_eyre::eyre::{Context, Ok};
use git2::{Index, Repository};
use owo_colors::OwoColorize;

use crate::core::git::git_helper;

pub fn commit(msg: String, is_add_all: Option<bool>) -> color_eyre::Result<String> {
    let git_data = git_helper::GitData::global().lock().unwrap();
    let mut index = git_data.repo.index().context("Error to get git index")?;

    let final_add_all = is_add_all.unwrap_or(false);

    if final_add_all {
        add_all(&git_data.repo, &mut index).context("Error to add all")?;
    }

    let has_staged = git_helper::has_staged_changes(&git_data.repo)?;
    if !has_staged {
        return Ok("Enough staged files has founded.".into());
    }

    let tree_id = index.write_tree().context("Error to get tree id")?;
    let tree = git_data.repo.find_tree(tree_id).context("Error to get tree")?;

    let parent_commit = git_data.repo.head()?.peel_to_commit()?;
    let parent = vec![&parent_commit];

    let sign = git_data.repo.signature().context("Error to get user signature")?;
    git_data.repo.commit(Some("HEAD"), &sign, &sign, &msg, &tree, &parent)
        .context("Error to create commit")?;

    Ok("Commit has been commited.".into())
}

pub fn add_all(repo: &Repository, index: &mut Index) -> color_eyre::Result<()> {
    let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
        let status = repo.status_file(path).unwrap();

        if status.contains(git2::Status::WT_MODIFIED)
            || status.contains(git2::Status::WT_DELETED)
            || status.contains(git2::Status::WT_NEW)
            || status.contains(git2::Status::WT_RENAMED) 
        {
            println!("Add {}", path.display().green());
            0
        } else {
            1
        }
    };

    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, Some(cb))?;
    index.write()?;

    Ok(())
}

pub fn add(files: Vec<String>) -> color_eyre::Result<()> {
    let git_data = git_helper::GitData::global().lock().unwrap();
    let mut index = git_data.repo.index()?;

    for file in files {
        index.add_path(Path::new(file.as_str()))?;
    }
    index.write()?;

    Ok(())
}

pub fn get_all_files_untracked() -> color_eyre::Result<Vec<String>> {
    let git_data = git_helper::GitData::global().lock().unwrap();

    let all_files: color_eyre::Result<Vec<std::path::PathBuf>> = git_data.repo.statuses(None)?
        .iter()
        .filter(|e| e.status() != git2::Status::IGNORED)
        .map(|entry| {
            let path = entry
                .index_to_workdir().unwrap()
                .old_file()
                .path()
                .ok_or_else(|| git2::Error::from_str("Wrong path"))?
                .to_path_buf();

            Ok(path)
        }).collect();

    let all_files: Vec<String> = all_files?.iter().map(
        |f| git_helper::pathbuf_to_string(f)
    ).flatten().collect();

    Ok(all_files)
}

#[test]
fn test_git() {
    // TODO: Just to test things
}