#![allow(dead_code)]

#[cfg(test)]
#[path = "./release_test.rs"]
mod release_test;

use super::commit::{Commit, Verb};
use std::collections::HashMap;

/// A Release represents one of more Commits, grouped by the verbs in their title.
pub struct Release<'a> {
    commits: Vec<Commit<'a>>,
}

impl<'a> Release<'a> {
    pub fn new(commits: Vec<Commit<'a>>) -> Self {
        Release { commits }
    }

    pub fn get_verb_groups(&self) -> HashMap<Verb, Vec<&Commit<'a>>> {
        let mut map: HashMap<Verb, Vec<&Commit>> = HashMap::with_capacity(self.commits.len());
        for commit in &self.commits {
            let verb = commit.verb();
            map.entry(verb).or_default().push(commit);
        }
        map
    }
}

impl<'a> From<Vec<git2::Commit<'a>>> for Release<'a> {
    fn from(commits: Vec<git2::Commit<'a>>) -> Self {
        let commits = commits.into_iter().map(Commit::from).collect();
        Release { commits }
    }
}
