use std::path::Path;

use color_eyre::eyre::Context;
use git2::Repository;

pub fn commit(msg: String, add_all: Option<bool>) -> color_eyre::Result<()> {
    let repo = Repository::open(Path::new(".")).context("Error to load init")?;
    let mut index = repo.index().context("Error to get git index")?;

    let final_add_all = add_all.unwrap_or(false);

    if final_add_all {
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
            .context("Error to add all")?;
    }

    let tree_id = index.write_tree().context("Error to get tree id")?;
    let tree = repo.find_tree(tree_id).context("Error to get tree")?;

    let sign = repo.signature().context("Error to get user signature")?;
    repo.commit(Some("HEAD"), &sign, &sign, &msg, &tree, &[])
        .context("Error to create commit")?;

    Ok(())
}