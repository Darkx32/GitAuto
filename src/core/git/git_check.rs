use git2::StatusOptions;

use crate::core::git::git_helper::get_git_data;

pub fn check_if_repo_has_changes() -> color_eyre::Result<bool> {
    let git_data = get_git_data()?;

    let mut opts = StatusOptions::new();
    opts.include_ignored(false);
    opts.include_untracked(true);

    let statuses = git_data.repo.statuses(Some(&mut opts))?;

    Ok(!statuses.is_empty())
}