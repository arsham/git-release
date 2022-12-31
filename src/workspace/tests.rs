use super::super::common_test;
use crate::workspace::Workspace;

#[cfg(test)]
mod latest_tag {
    use super::*;

    #[test]
    fn no_tag_set() -> Result<(), git2::Error> {
        let (dir, _) = common_test::repo_init();
        let ws = Workspace::new(dir)?;
        let tag = ws.latest_tag();
        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn one_tag() -> Result<(), git2::Error> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let (commit, _) = common_test::commit(&repo, "file");
        let obj = repo.find_object(commit, None)?;
        let sig = repo.signature()?;
        let tag_name = "v0.6.6.6";
        repo.tag(tag_name, &obj, &sig, "msg", false)?;

        let ws = Workspace::new(&dir)?;
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

        let ws = Workspace::new(&dir)?;
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

        let ws = Workspace::new(&dir)?;
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

        let ws = Workspace::new(&dir)?;
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

        let ws = Workspace::new(&dir)?;
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

        let ws = Workspace::new(&dir)?;
        let tag = ws.validate_tag(tag)?;
        assert_eq!(oid, tag);
        Ok(())
    }

    #[test]
    fn input_is_not_in_workspace() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit_tag(&repo, "file", "tag");

        let ws = Workspace::new(&dir)?;
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

        let ws = Workspace::new(&dir)?;
        let tag = ws.previous_tag(tag_name)?;
        assert_eq!(commit.to_string(), tag);
        Ok(())
    }

    #[test]
    fn commit_in_the_middle() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1");

        let tag1 = "tag1";
        common_test::commit_tag(&repo, "file2", tag1);

        common_test::commit(&repo, "file3");
        common_test::commit(&repo, "file4");

        let tag2 = "tag2";
        common_test::commit_tag(&repo, "file5", tag2);

        common_test::commit(&repo, "file5");
        common_test::commit(&repo, "file6");

        let ws = Workspace::new(&dir)?;
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
        common_test::commit(&repo, "file1");

        let tag = "tag";
        common_test::commit_tag(&repo, "file2", tag);

        let ws = Workspace::new(&dir)?;
        let res = ws.commits_between_tags("tag_not_found", tag);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn to_tag_not_in_repo() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1");

        let tag = "tag";
        common_test::commit_tag(&repo, "file2", tag);

        let ws = Workspace::new(&dir)?;
        let res = ws.commits_between_tags(tag, "tag_not_found");
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn from_and_to_point_to_same_commit() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1");

        let (commit, _) = common_test::commit(&repo, "file2");
        let obj = repo.find_object(commit, None)?;
        let sig = repo.signature()?;
        let tag1 = "tag1";
        repo.tag(tag1, &obj, &sig, "msg", false)?;
        let tag2 = "tag2";
        repo.tag(tag2, &obj, &sig, "msg", false)?;

        let ws = Workspace::new(&dir)?;
        let res = ws.commits_between_tags(tag1, tag2);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn adjacent_commits() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file1");

        let tag1 = "tag1";
        common_test::commit_tag(&repo, "file2", tag1);

        let tag2 = "tag2";
        let (commit2, _) = common_test::commit_tag(&repo, "file3", tag2);

        let ws = Workspace::new(&dir)?;
        let res: Vec<git2::Oid> = ws
            .commits_between_tags(tag1, tag2)?
            .iter()
            .map(git2::Commit::id)
            .collect();
        assert_eq!(vec![commit2], res);

        Ok(())
    }

    #[test]
    fn multiple_commits() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        common_test::commit(&repo, "file_p1");
        common_test::commit(&repo, "file_p2");

        let tag1 = "tag1";
        common_test::commit_tag(&repo, "file2", tag1);

        let (commit2, _) = common_test::commit(&repo, "file3");
        let (commit3, _) = common_test::commit(&repo, "file4");

        let tag2 = "tag2";
        let (commit4, _) = common_test::commit_tag(&repo, "file5", tag2);

        common_test::commit(&repo, "file5");
        common_test::commit(&repo, "file6");

        let ws = Workspace::new(&dir)?;
        let res: Vec<git2::Oid> = ws
            .commits_between_tags(tag1, tag2)?
            .iter()
            .map(git2::Commit::id)
            .collect();
        assert_eq!(vec![commit2, commit3, commit4], res);

        Ok(())
    }
}
