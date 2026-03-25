use std::{path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

use color_eyre::eyre::Context;
use git2::{Repository, Status};

pub(super) struct GitData {
    pub(super) repo: Repository
}

static GIT_DATA: OnceLock<Mutex<GitData>> = OnceLock::new();

impl GitData {
    fn new() -> color_eyre::Result<Self> {
        let repo = Repository::open(Path::new("."))
            .context("Not found repository on that folder")?;

        Ok(Self { repo })
    }

    pub(super) fn global() -> &'static Mutex<GitData> {
        GIT_DATA.get_or_init(|| {
            Mutex::new(GitData::new().expect("Error to start git"))
        })
    }
}

unsafe impl Send for GitData {}
unsafe impl Sync for GitData {}

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