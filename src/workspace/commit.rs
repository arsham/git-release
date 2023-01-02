use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

#[cfg(test)]
#[path = "./commit_test.rs"]
mod commit_test;

lazy_static! {
    static ref SUMMARY_RE: Regex =
        Regex::new(r#"^\s*(\w+)( *\(([ /.\w,_-]+)?\) *)?(!)? *:?(.*)"#).unwrap();
    static ref REF_RE: Regex = Regex::new(r#"\(?\w+\s+#(\d+)\)?"#).unwrap();
}

/// A Commit represents a commit in the repository with its metadata.
#[derive(Debug, Clone)]
pub struct Commit<'a> {
    commit: git2::Commit<'a>,
}

impl Commit<'_> {
    /// Returns the summary of the commit. If there is an error or the body is not valid UTF-8 it
    /// returns an empty string.
    pub fn title(&self) -> Option<String> {
        self.commit
            .summary()
            .map(|summary| REF_RE.replace_all(summary, "").trim().to_string())
    }

    /// Returns the verb in the summary of the commit message if specified.
    pub fn verb(&self) -> Verb {
        return self
            .commit
            .summary()
            .and_then(|title| {
                SUMMARY_RE.captures(title).and_then(|caps| {
                    caps.get(1)
                        .map(|verb| verb.as_str().to_lowercase().as_str().into())
                })
            })
            .unwrap_or(Verb::Misc);
    }

    /// Returns a vector of references to other issues on github.
    pub fn references(&self) -> Vec<Reference> {
        let body = &format!(
            "{}\n{}",
            self.commit.summary().unwrap_or(""),
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

    /// Returns a vector of subjects in the title if provided. Subjects are separated by comma.
    // TODO: rename this to avoid confusion with git's subject.
    pub fn subjects(&self) -> Option<Vec<&str>> {
        self.commit.summary().and_then(|title| {
            return SUMMARY_RE
                .captures(title)
                .and_then(|caps| Some(caps.get(3)?.as_str().split(',').map(str::trim).collect()));
        })
    }

    /// Returns true if the commit has breaking changes. There are two ways a commit is breaking:
    /// 1. If the title has an explanation mark in front of the verb.
    /// 2. If the footer starts with `BREAKING CHANGE:`.
    pub fn is_breaking(&self) -> bool {
        self.commit
            .summary()
            .and_then(|title| {
                SUMMARY_RE
                    .captures(title)
                    .and_then(|caps| caps.get(4).map(|_| true))
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

impl<'a> PartialEq for Commit<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.commit.id() == other.commit.id()
    }
}

/// A Reference represents a link to a github issue.
#[derive(Debug, PartialEq, Eq)]
pub struct Reference(pub u16);

impl Reference {
    /// Returns a pound sign with the issue number.
    pub fn issue_ref(&self) -> String {
        format!("#{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Verb {
    Feature,
    Fix,
    Refactor,
    Chore,
    Enhancements,
    Style,
    CI,
    Documentation,
    Misc,
}

impl From<&str> for Verb {
    fn from(value: &str) -> Self {
        match value {
            "feat" | "feature" => Verb::Feature,
            "fix" | "fixed" | "fixes" => Verb::Fix,
            "ref" | "refactor" | "refactored" => Verb::Refactor,
            "chore" => Verb::Chore,
            "enhance" | "enhanced" => Verb::Enhancements,
            "enhancement" | "enhancements" => Verb::Enhancements,
            "improve" | "improved" | "improves" => Verb::Enhancements,
            "improvement" | "improvements" => Verb::Enhancements,
            "style" => Verb::Style,
            "ci" => Verb::CI,
            "doc" | "docs" => Verb::Documentation,
            _ => Verb::Misc,
        }
    }
}

impl Display for Commit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut subjects = String::new();
        if let Some(s) = self.subjects() {
            subjects.push_str("**");
            subjects.push_str(&s.join(", "));
            subjects.push_str(":** ");
        }

        let title = self.title();
        let title = title.and_then(|title| {
            if !title.contains(':') {
                return Some(title);
            }
            SUMMARY_RE
                .captures(&title)
                .and_then(|caps| Some(caps.get(5)?.as_str().trim().to_string()))
        });
        let mut title = match title {
            Some(t) => t,
            None => return Err(std::fmt::Error),
        };
        first_letter_uppercase(&mut title);

        let breaking = if self.is_breaking() {
            " [**BREAKING CHANGE**]"
        } else {
            ""
        };

        let mut refs = self
            .references()
            .into_iter()
            .map(|reference| format!("ref {}", reference.issue_ref()))
            .collect::<Vec<String>>()
            .join(", ");
        if !refs.is_empty() {
            refs.insert_str(0, " (");
            refs.push(')');
        }

        write!(f, "{subjects}{title}{breaking}{refs}")?;
        Ok(())
    }
}

fn first_letter_uppercase(s: &mut String) {
    let mut c = s.chars();
    if let Some(f) = c.next() {
        s.remove(0);
        // Should be this way because the character's uppercase could be two chars.
        let upper = f.to_uppercase().to_string();
        s.insert_str(0, &upper);
    }
}
