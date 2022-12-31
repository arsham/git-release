use git2::{Oid, Repository, RepositoryInitOptions};
use std::fs::File;
use std::path::Path;
use tempfile::TempDir;

// Some of these helpers are borrowed from the git2 code.

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

pub fn repo_init() -> (TempDir, Repository) {
    let td = TempDir::new().unwrap();
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("master");
    let repo = Repository::init_opts(td.path(), &opts).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "name").unwrap();
        config.set_str("user.email", "email").unwrap();
        let mut index = repo.index().unwrap();
        let id = index.write_tree().unwrap();

        let tree = repo.find_tree(id).unwrap();
        let sig = repo.signature().unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial\n\nbody", &tree, &[])
            .unwrap();
    }
    (td, repo)
}

pub fn commit(repo: &Repository, filename: &str, msg: Option<&str>) -> (Oid, Oid) {
    let mut index = t!(repo.index());
    let root = repo.path().parent().unwrap();
    t!(File::create(root.join(filename)));
    t!(index.add_path(Path::new(filename)));

    let tree_id = t!(index.write_tree());
    let tree = t!(repo.find_tree(tree_id));
    let sig = t!(repo.signature());
    let head_id = t!(repo.refname_to_id("HEAD"));
    let parent = t!(repo.find_commit(head_id));
    let msg = if let Some(msg) = msg {
        msg
    } else {
        "some commit"
    };
    let commit = t!(repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&parent]));
    (commit, tree_id)
}

pub fn commit_tag(repo: &Repository, filename: &str, tag: &str) -> (Oid, Oid) {
    let (commit, _) = commit(repo, filename, None);
    let obj = repo.find_object(commit, None).unwrap();
    let sig = repo.signature().unwrap();
    let tag = repo.tag(tag, &obj, &sig, "msg", false).unwrap();
    (commit, tag)
}

pub fn commit_lightweight_tag(repo: &Repository, filename: &str, tag: &str) -> (Oid, Oid) {
    let (commit, _) = commit(repo, filename, None);
    let obj = repo.find_object(commit, None).unwrap();
    let tag = repo.tag_lightweight(tag, &obj, false).unwrap();
    (commit, tag)
}
