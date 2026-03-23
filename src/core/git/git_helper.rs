use git2::{Repository, Status};

pub(super) fn has_staged_changes(repo: &Repository) -> color_eyre::Result<bool> {
    let statuses = repo.statuses(None)?;

    Ok(statuses.iter().any(|entry| {
        let s = entry.status();

        s.intersects(
            Status::INDEX_NEW |
            Status::INDEX_MODIFIED |
            Status::INDEX_DELETED |
            Status::INDEX_RENAMED |
            Status::INDEX_TYPECHANGE
        )
    }))
}