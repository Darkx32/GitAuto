use std::path::Path;

use color_eyre::eyre::{Context, Ok};
use git2::{Index, Repository};

use crate::core::git::git_helper;

pub fn commit(msg: String, is_add_all: Option<bool>) -> color_eyre::Result<String> {
    let repo = Repository::open(Path::new(".")).context("Error to load init")?;
    let mut index = repo.index().context("Error to get git index")?;

    let final_add_all = is_add_all.unwrap_or(false);

    if final_add_all {
        add_all(&repo, &mut index).context("Error to add all")?;
    }

    let has_staged = git_helper::has_staged_changes(&repo)?;
    if !has_staged {
        return Ok("Enough staged files has founded.".into());
    }

    let tree_id = index.write_tree().context("Error to get tree id")?;
    let tree = repo.find_tree(tree_id).context("Error to get tree")?;

    let parent_commit = repo.head()?.peel_to_commit()?;
    let parent = vec![&parent_commit];

    let sign = repo.signature().context("Error to get user signature")?;
    repo.commit(Some("HEAD"), &sign, &sign, &msg, &tree, &parent)
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
            println!("Add {}", path.display());
            0
        } else {
            1
        }
    };

    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, Some(cb))?;
    index.write()?;

    Ok(())
}