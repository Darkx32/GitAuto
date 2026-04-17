use git2::{Repository, StatusOptions};

pub fn check_if_directory_is_repo() -> bool {
    match Repository::open(".") {
        Ok(_) => return true,
        Err(_) => return false
    }
}

pub fn check_if_repo_has_changes() -> color_eyre::Result<bool> {
    let repo = Repository::open(".")?;

    let mut opts = StatusOptions::new();
    opts.include_ignored(false);
    opts.include_untracked(true);

    let statuses = repo.statuses(Some(&mut opts))?;

    Ok(!statuses.is_empty())
}