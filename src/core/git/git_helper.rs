use std::{path::{Path, PathBuf}, sync::{Mutex, MutexGuard}};

use color_eyre::eyre::{bail, eyre};
use git2::{Repository, Status};
use once_cell::sync::OnceCell;

pub(super) struct GitData {
    pub(super) repo: Repository
}

static GIT_DATA: OnceCell<Mutex<GitData>> = OnceCell::new();

impl GitData {
    fn new() -> color_eyre::Result<Self> {
        let repo = match Repository::open(Path::new(".")) {
            Ok(repo) => repo,
            Err(_) => {
                bail!("Not found any repository on this current folder.")
            }
        };

        Ok(Self { repo })
    }
}

unsafe impl Send for GitData {}
unsafe impl Sync for GitData {}

pub(super) fn get_git_data() -> color_eyre::Result<MutexGuard<'static, GitData>> {
    let mutex = GIT_DATA.get_or_try_init(|| {
        GitData::new().map(Mutex::new)
    })?;

    mutex.lock()
        .map_err(|_| eyre!("Mutex poisoned."))
}

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

pub(super) fn pathbuf_to_string(path: &PathBuf) -> Option<String> {
    match path.to_str() {
        Some(s) => {
            Some(s.to_string())
        }
        None => {
            eprintln!("Path contains non-UTF8");
            None
        }
    }
}