#![allow(dead_code)]

use std::path::Path;

use git2::{Commit, ErrorClass, ErrorCode};

/// Repository represents the workspace in a git repository. You can use this struct to query for
/// various information against the underlying database.
pub struct Repository {
    repo: git2::Repository,
}

impl Repository {
    pub fn new<T: AsRef<Path>>(dir: T) -> Result<Self, git2::Error> {
        let repo = git2::Repository::open(dir)?;
        Ok(Self { repo })
    }

    /// Returns the latest tag in the chronological order.
    ///
    /// # Errors
    ///
    /// If there are no tags, `Err` is returned.
    pub fn latest_tag(&self) -> Result<String, git2::Error> {
        let tags = self.repo.tag_names(None)?;
        if let Some(Some(tag)) = tags.iter().last() {
            Ok(tag.to_string())
        } else {
            Err(git2::Error::new(
                ErrorCode::NotFound,
                ErrorClass::Tag,
                "no tags found",
            ))
        }
    }

    pub fn validate_tag(&self, tag: &str) -> Result<git2::Oid, git2::Error> {
        let rev = self.repo.revparse(tag)?;
        Ok(rev
            .from()
            .ok_or_else(|| {
                git2::Error::new(
                    ErrorCode::NotFound,
                    ErrorClass::Tag,
                    format!("given {tag} tag is not valid"),
                )
            })?
            .id())
    }

    /// Returns the tag before the given input. If the tag is the first tag in the repository, the
    /// hash of the first commit is returned.
    ///
    /// # Errors
    ///
    /// If the tag is not in the repository, an `Err` is returned.
    pub fn previous_tag(&self, current: &str) -> Result<String, git2::Error> {
        self.validate_tag(current)?;

        let tags = self.repo.tag_names(None)?;
        let mut tag = tags
            .iter()
            .rev()
            .skip_while(|&hash| hash != Some(current))
            .skip(1)
            .take(1);
        if let Some(Some(tag)) = tag.next() {
            Ok(tag.to_string())
        } else {
            let head = self.repo.head()?;
            Ok(head.peel_to_commit().unwrap().id().to_string())
        }
    }

    /// Returns all commits between two tags. It excludes the commit that `from` is pointing at,
    /// and includes the commit that the `to` is pointing at.
    ///
    /// # Errors
    ///
    /// If either tags is not in the repository, or both point to the same commit, an `Err` is
    /// returned.
    pub fn commits_between_tags(&self, from: &str, to: &str) -> Result<Vec<Commit>, git2::Error> {
        let from_obj = self.repo.revparse_single(from)?;
        let to_obj = self.repo.revparse_single(to)?;
        let from = from_obj.id();
        let to = to_obj.id();

        let from_obj = from_obj.peel_to_commit()?;
        let to_obj = to_obj.peel_to_commit()?;
        if from_obj.id() == to_obj.id() {
            return Err(git2::Error::new(
                ErrorCode::User,
                ErrorClass::Tag,
                "both tags are pointed at the same commit",
            ));
        }

        let mut res = self.repo.revwalk()?;
        let range = format!("{from}..{to}");
        res.push_range(&range)?;
        res.set_sorting(git2::Sort::REVERSE)?;
        let res = res
            .filter_map(Result::ok)
            .filter_map(|oid| {
                if let Ok(oid) = self.repo.find_commit(oid) {
                    Some(oid)
                } else {
                    None
                }
            })
            .collect();
        Ok(res)
    }
}
