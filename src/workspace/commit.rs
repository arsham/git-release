#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SUMMARY_RE: Regex = Regex::new(r#"^\s*(\w+)\(?([\w,_-]+)?\)?(!)?:?(.*)"#).unwrap();
    static ref REF_RE: Regex = Regex::new(r#"\w+\s+#(\d+)"#).unwrap();
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

    /// Returns a vector of references to other issues on github.
    pub fn references(&self) -> Vec<Reference> {
        let body = &format!(
            "{}\n{}",
            self.title().unwrap_or(""),
            self.commit.body().unwrap_or("")
        );
        let mut refs = vec![];
        for cap in REF_RE.captures_iter(body) {
            if let Some(num) = cap.get(1) {
                if let Ok(num) = num.as_str().parse() {
                    refs.push(Reference(num));
                }
            }
        }
        refs
    }

    /// Returns true if the commit has breaking changes. There are two ways a commit is breaking:
    /// 1. If the title has an explanation mark in front of the verb.
    /// 2. If the footer starts with `BREAKING CHANGE:`.
    pub fn is_breaking(&self) -> bool {
        self.title()
            .and_then(|title| {
                SUMMARY_RE
                    .captures(title)
                    .and_then(|caps| caps.get(3).map(|_| true))
            })
            .unwrap_or_else(|| {
                self.commit
                    .body()
                    .map(|body| {
                        body.lines()
                            .last()
                            .unwrap_or("")
                            .starts_with("BREAKING CHANGE:")
                    })
                    .unwrap_or(false)
            })
    }
}

impl<'a> From<git2::Commit<'a>> for Commit<'a> {
    fn from(commit: git2::Commit<'a>) -> Self {
        Commit { commit }
    }
}

/// A Reference represents a link to a github issue.
#[derive(Debug, PartialEq, Eq)]
pub struct Reference(pub u16);
