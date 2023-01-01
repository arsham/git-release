#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SUMMARY_RE: Regex = Regex::new(r#"^\s*(\w+!?)\(?([\w,_-]+)?\)?(!)?:?(.*)"#).unwrap();
}

/// A Commit represents a commit in the repository with its metadata.
pub struct Commit<'a> {
    commit: git2::Commit<'a>,
}

impl Commit<'_> {
    /// Returns the summary of the commit. If there is an error or the body is not valid UTF-8 it
    /// returns an empty string.
    pub fn title(&self) -> Option<&str> {
        self.commit.summary()
    }

    /// Returns the verb in the summary of the commit message if specified.
    pub fn verb(&self) -> &str {
        return self
            .title()
            .and_then(|title| {
                SUMMARY_RE.captures(title).and_then(|caps| {
                    caps.get(1)
                        .map(|verb| match verb.as_str().to_lowercase().as_str() {
                            "feat" | "feature" => "Feature",
                            "fix" | "fixed" | "fixes" => "Fix",
                            "ref" | "refactor" | "refactored" => "Refactor",
                            "chore" => "Chore",
                            "enhance" | "enhanced" => "Enhancements",
                            "enhancement" | "enhancements" => "Enhancements",
                            "improve" | "improved" | "improves" => "Enhancements",
                            "improvement" | "improvements" => "Enhancements",
                            "style" => "Style",
                            "ci" => "CI",
                            "doc" | "docs" => "Documentation",
                            _ => "Misc",
                        })
                })
            })
            .unwrap_or("Misc");
    }
}

impl<'a> From<git2::Commit<'a>> for Commit<'a> {
    fn from(commit: git2::Commit<'a>) -> Self {
        Commit { commit }
    }
}
