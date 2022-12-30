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
}
