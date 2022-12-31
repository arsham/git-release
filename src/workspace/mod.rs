#![allow(dead_code)]

use std::path::Path;

#[cfg(test)]
mod tests;

/// Workspace represents the workspace in a git repository. You can use this struct to query for
/// various information against the underlying database.
pub struct Workspace {
    repo: git2::Repository,
}

impl Workspace {
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
                git2::ErrorCode::NotFound,
                git2::ErrorClass::Tag,
                "no tags found",
            ))
        }
    }

    /// Returns the tag before the given input. If the tag is the first tag in the repository, the
    /// hash of the first commit is returned.
    ///
    /// # Errors
    ///
    /// If the tag is not in the repository, an `Err` is returned.
    pub fn previous_tag(&self, current: &str) -> Result<String, git2::Error> {
        let tags = self.repo.tag_names(None)?;
        let mut tags = tags.iter().filter(|&tag| {
            if let Some(tag) = tag {
                tag == current
            } else {
                false
            }
        });

        if tags.next().is_none() {
            return Err(git2::Error::new(
                git2::ErrorCode::NotFound,
                git2::ErrorClass::Tag,
                "given tag is not valid",
            ));
        };

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
}
