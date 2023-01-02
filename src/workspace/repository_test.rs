use super::super::super::common_test;
use crate::workspace::repository::Repository;

#[cfg(test)]
mod latest_tag {
    use super::*;

    #[test]
    fn no_tag_set() -> Result<(), git2::Error> {
        let (dir, _) = common_test::repo_init();
        let ws = Repository::new(dir)?;
        let tag = ws.latest_tag();
        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn one_tag() -> Result<(), git2::Error> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let (commit, _) = common_test::commit(&repo, "file", None);
        let obj = repo.find_object(commit, None)?;
        let sig = repo.signature()?;
        let tag_name = "v0.6.6.6";
        repo.tag(tag_name, &obj, &sig, "msg", false)?;

        let ws = Repository::new(&dir)?;
        let tag = ws.latest_tag()?;
        assert_eq!(tag_name, tag);
        Ok(())
    }

    #[test]
    fn two_tags() -> Result<(), git2::Error> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let tag_name = "tag1";
        common_test::commit_tag(&repo, "file", tag_name);
        let tag_name = "tag2";
        common_test::commit_tag(&repo, "file", tag_name);

        let ws = Repository::new(&dir)?;
        let tag = ws.latest_tag()?;
        assert_eq!(tag_name, tag);
        Ok(())
    }
}

#[cfg(test)]
mod previous_tag {
    use super::*;

    #[test]
    fn validate_tag_signed_not_valid() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let tag = "tag";
        common_test::commit_tag(&repo, "file", tag);

        let ws = Repository::new(&dir)?;
        let tag = ws.validate_tag("not_exists");
        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn validate_tag_signed_valid() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let tag = "tag";
        let (_, oid) = common_test::commit_tag(&repo, "file", tag);

        let ws = Repository::new(&dir)?;
        let tag = ws.validate_tag(tag)?;
        assert_eq!(oid, tag);
        Ok(())
    }

    #[test]
    fn validate_tag_lightweight_not_valid() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let tag = "tag";
        common_test::commit_lightweight_tag(&repo, "file", tag);

        let ws = Repository::new(&dir)?;
        let tag = ws.validate_tag("not_exists");
        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn validate_tag_lightweight_valid() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let tag = "tag";
        let (_, oid) = common_test::commit_lightweight_tag(&repo, "file", tag);

        let ws = Repository::new(&dir)?;
        let tag = ws.validate_tag(tag)?;
        assert_eq!(oid, tag);
        Ok(())
    }

    #[test]
    fn input_is_not_in_workspace() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit_tag(&repo, "file", "tag");

        let ws = Repository::new(&dir)?;
        let tag = ws.previous_tag("not_exists");
        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn input_is_the_only_tag() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let tag_name = "tag1";
        let (commit, _) = common_test::commit_tag(&repo, "file", tag_name);

        let ws = Repository::new(&dir)?;
        let tag = ws.previous_tag(tag_name)?;
        assert_eq!(commit.to_string(), tag);
        Ok(())
    }

    #[test]
    fn commit_in_the_middle() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1", None);

        let tag1 = "tag1";
        common_test::commit_tag(&repo, "file2", tag1);

        common_test::commit(&repo, "file3", None);
        common_test::commit(&repo, "file4", None);

        let tag2 = "tag2";
        common_test::commit_tag(&repo, "file5", tag2);

        common_test::commit(&repo, "file5", None);
        common_test::commit(&repo, "file6", None);

        let ws = Repository::new(&dir)?;
        let tag = ws.previous_tag(tag2)?;
        assert_eq!(tag1.to_string(), tag);
        Ok(())
    }
}

#[cfg(test)]
mod commits_between_tags {
    use super::*;

    #[test]
    fn from_tag_not_in_repo() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1", None);

        let tag = "tag";
        common_test::commit_tag(&repo, "file2", tag);

        let ws = Repository::new(&dir)?;
        let res = ws.commits_between_tags("tag_not_found", tag);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn to_tag_not_in_repo() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1", None);

        let tag = "tag";
        common_test::commit_tag(&repo, "file2", tag);

        let ws = Repository::new(&dir)?;
        let res = ws.commits_between_tags(tag, "tag_not_found");
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn from_and_to_point_to_same_commit() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1", None);

        let (commit, _) = common_test::commit(&repo, "file2", None);
        let obj = repo.find_object(commit, None)?;
        let sig = repo.signature()?;
        let tag1 = "tag1";
        repo.tag(tag1, &obj, &sig, "msg", false)?;
        let tag2 = "tag2";
        repo.tag(tag2, &obj, &sig, "msg", false)?;

        let ws = Repository::new(&dir)?;
        let res = ws.commits_between_tags(tag1, tag2);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn adjacent_commits() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1", None);

        let tag1 = "tag1";
        common_test::commit_tag(&repo, "file2", tag1);

        let tag2 = "tag2";
        let (commit2, _) = common_test::commit_tag(&repo, "file3", tag2);

        let ws = Repository::new(&dir)?;
        let res: Vec<git2::Oid> = ws
            .commits_between_tags(tag1, tag2)?
            .map(|c| c.id())
            .collect();
        assert_eq!(vec![commit2], res);

        Ok(())
    }

    #[test]
    fn multiple_commits() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file_p1", None);
        common_test::commit(&repo, "file_p2", None);

        let tag1 = "tag1";
        common_test::commit_tag(&repo, "file2", tag1);

        let (commit2, _) = common_test::commit(&repo, "file3", None);
        let (commit3, _) = common_test::commit(&repo, "file4", None);

        let tag2 = "tag2";
        let (commit4, _) = common_test::commit_tag(&repo, "file5", tag2);

        common_test::commit(&repo, "file5", None);
        common_test::commit(&repo, "file6", None);

        let ws = Repository::new(&dir)?;
        let res: Vec<git2::Oid> = ws
            .commits_between_tags(tag1, tag2)?
            .map(|c| c.id())
            .collect();
        assert_eq!(vec![commit2, commit3, commit4], res);

        Ok(())
    }
}

#[cfg(test)]
mod getting_names {
    use crate::{common_test::repo_init, workspace::repository::Repository};

    #[test]
    fn repo_name() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, repo) = repo_init();
        repo.remote_set_url("origin", "git@github.com:arsham/shark.git")?;
        let repo = Repository::new(dir)?;
        let name = repo.repo_name("origin")?;
        assert_eq!("shark", &name);

        Ok(())
    }

    #[test]
    fn username() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, repo) = repo_init();
        repo.remote_set_url("origin", "git@github.com:arsham/shark.git")?;
        let repo = Repository::new(dir)?;
        let name = repo.username("origin")?;
        assert_eq!("arsham", &name);

        Ok(())
    }
}
