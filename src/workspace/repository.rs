use std::path::Path;

use git2::Commit;
use lazy_static::lazy_static;
use regex::Regex;

use crate::workspace::errors;

#[cfg(test)]
#[path = "./repository_test.rs"]
mod repository_test;

lazy_static! {
    static ref REPO_RE: Regex =
        Regex::new(r#"github.com[:/](?P<user>[^/]+)/(?P<repo>.+?)(?:.git)?\n?$"#).unwrap();
}

/// Repository represents the workspace in a git repository. You can use this struct to query for
/// various information against the underlying database.
pub struct Repository {
    repo: git2::Repository,
}

impl Repository {
    pub fn new<T: AsRef<Path>>(dir: T) -> Result<Self, errors::GRError> {
        let repo = git2::Repository::open(dir).map_err(errors::GRError::Repository)?;
        Ok(Self { repo })
    }

    /// Returns the latest tag in the chronological order.
    ///
    /// # Errors
    ///
    /// If there are no tags, `Err` is returned.
    pub fn latest_tag(&self) -> Result<String, errors::GRError> {
        let tags = self
            .repo
            .tag_names(None)
            .map_err(errors::GRError::TagNameList)?;
        if let Some(Some(tag)) = tags.iter().last() {
            Ok(tag.to_owned())
        } else {
            Err(errors::GRError::TagNotFound("latest".to_owned()))
        }
    }

    pub fn validate_tag(&self, tag: &str) -> Result<git2::Oid, errors::GRError> {
        let rev = self
            .repo
            .revparse(tag)
            .map_err(errors::GRError::Repository)?;
        Ok(rev
            .from()
            .ok_or_else(|| errors::GRError::TagNotFound(tag.to_owned()))?
            .id())
    }

    /// Returns the tag before the given input. If the tag is the first tag in the repository, the
    /// hash of the first commit is returned.
    ///
    /// # Errors
    ///
    /// If the tag is not in the repository, an `Err` is returned.
    pub fn previous_tag(&self, current: &str) -> Result<String, errors::GRError> {
        self.validate_tag(current)?;

        let tags = self
            .repo
            .tag_names(None)
            .map_err(errors::GRError::TagNameList)?;
        let mut tag = tags
            .iter()
            .rev()
            .skip_while(|&hash| hash != Some(current))
            .skip(1)
            .take(1);
        if let Some(Some(tag)) = tag.next() {
            Ok(tag.to_owned())
        } else {
            let head = self.repo.head()?;
            let id = head.peel_to_commit()?.id();
            Ok(id.to_string())
        }
    }

    /// Returns and iterator that would produce all commits between two tags. It excludes the
    /// commit that `from` is pointing at, and includes the commit that the `to` is pointing at.
    ///
    /// # Errors
    ///
    /// If either tags is not in the repository, or both point to the same commit, an `Err` is
    /// returned.
    pub fn commits_between_tags(
        &self,
        from: &str,
        to: &str,
    ) -> Result<impl Iterator<Item = Commit>, errors::GRError> {
        let from_obj = self.repo.revparse_single(from)?;
        let to_obj = self.repo.revparse_single(to)?;
        let from = from_obj.id();
        let to = to_obj.id();

        let from_obj = from_obj.peel_to_commit()?;
        let to_obj = to_obj.peel_to_commit()?;
        if from_obj.id() == to_obj.id() {
            return Err(errors::GRError::TwinTags);
        }

        let mut res = self.repo.revwalk()?;
        let range = format!("{from}..{to}");
        res.push_range(&range)?;
        res.set_sorting(git2::Sort::REVERSE)?;
        let res = res.filter_map(Result::ok).filter_map(|oid| {
            if let Ok(oid) = self.repo.find_commit(oid) {
                Some(oid)
            } else {
                None
            }
        });
        Ok(res)
    }

    fn repo_name_username(&self, remote: &str, index: usize) -> Result<String, errors::GRError> {
        let url = self.repo.find_remote(remote)?;
        let url = url.url().ok_or_else(|| {
            errors::GRError::URLError("could not get the url of the repository".to_owned())
        })?;
        if let Some(caps) = REPO_RE.captures(url) {
            return Ok(caps
                .get(index)
                .ok_or_else(|| {
                    errors::GRError::URLError(format!(
                        "could not find the setting from the url: {url}",
                    ))
                })?
                .as_str()
                .to_owned());
        }
        Ok(url.to_owned())
    }

    /// Returns the repository name.
    pub fn repo_name(&self, remote: &str) -> Result<String, errors::GRError> {
        self.repo_name_username(remote, 2)
    }

    /// Returns the user or organisation of the repository.
    pub fn username(&self, remote: &str) -> Result<String, errors::GRError> {
        self.repo_name_username(remote, 1)
    }
}
