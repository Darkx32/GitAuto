use git2::Repository;

pub fn get_actual_branch() -> color_eyre::Result<String> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;

    Ok(head.shorthand().map(str::to_string).unwrap())
}

pub fn get_last_commit() -> color_eyre::Result<String> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    Ok(commit.message().unwrap().to_string())
}