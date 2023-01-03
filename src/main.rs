use args::Tag;
use colored::*;
use workspace::release::Release;

mod args;
mod gh;
mod workspace;

#[cfg(test)]
mod common_test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let token = &std::env::var("GITHUB_TOKEN")?;
    let user = &repo.username(&opt.remote)?;
    let repo_name = &repo.repo_name(&opt.remote)?;
    let commits = repo.commits_between_tags(&prev, &latest)?;
    let release: Release = commits.collect::<Vec<git2::Commit>>().into();

    if !opt.publish {
        println!("{release}");
        return Ok(());
    }

    let releaser = gh::Release {
        token,
        user,
        repository: repo_name,
        tag: &latest,
        description: &format!("{release}"),
    };

    if let Err(err) = releaser.create().await {
        if !opt.force {
            return Err(err.into());
        }
        let id = releaser.release_id().await?;
        releaser.update(id.0).await?;
        println!("Force updated the {} tag", latest.green().bold());
    }
    Ok(())
}
