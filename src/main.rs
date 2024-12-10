use git2::{Error, Repository};

fn main() -> Result<(), Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;

    // Start at HEAD
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        println!(
            "Commit: {}\nAuthor: {}\nMessage: {}\n",
            commit.id(),
            commit.author(),
            commit.message().unwrap_or("No message")
        );
    }

    Ok(())
}
