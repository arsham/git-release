use args::Tag;
use workspace::release::Release;

mod args;
mod workspace;

#[cfg(test)]
mod common_test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = args::Opt::new();

    if let Some(args::Command::Version) = opt.sub_commands {
        println!(
            "git-release version: {}, git commit: {}",
            env!("APP_VERSION"),
            env!("CURRENT_SHA")
        );
    }

    let repo = workspace::repository::Repository::new(".")?;
    let latest: String;
    let prev: String;
    match opt.tags {
        Tag::None => {
            latest = repo.latest_tag()?;
            prev = repo.previous_tag(&latest)?;
        },
        Tag::From(tag) => {
            repo.validate_tag(&tag)?;
            latest = repo.latest_tag()?;
            prev = tag;
        },
        Tag::Single(tag) => {
            repo.validate_tag(&tag)?;
            latest = tag;
            prev = repo.previous_tag(&latest)?;
        },
        Tag::Range(from, to) => {
            repo.validate_tag(&from)?;
            repo.validate_tag(&to)?;
            latest = to;
            prev = from;
        },
    }
    let commits = repo.commits_between_tags(&prev, &latest)?;
    let release: Release = commits.into();
    println!("{release}");
    Ok(())
}
