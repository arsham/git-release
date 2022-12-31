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
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag_name = "v0.6.6.6";
        repo.tag(tag_name, &obj, &sig, "msg", false).unwrap();

        let ws = Workspace::new(&dir)?;
        let tag = ws.latest_tag()?;
        assert_eq!(tag_name, tag);
        Ok(())
    }

    #[test]
    fn two_tags() -> Result<(), git2::Error> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let (commit, _) = common_test::commit(&repo, "file");
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag_name = "tag1";
        repo.tag(tag_name, &obj, &sig, "msg", false).unwrap();

        let (commit, _) = common_test::commit(&repo, "file");
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag_name = "tag2";
        repo.tag(tag_name, &obj, &sig, "msg", false).unwrap();

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
    fn input_is_not_in_workspace() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let (commit, _) = common_test::commit(&repo, "file");
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag_name = "tag1";
        repo.tag(tag_name, &obj, &sig, "msg", false).unwrap();

        let ws = Workspace::new(&dir)?;
        let tag = ws.previous_tag("not_exists");
        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn input_is_the_only_tag() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let (commit, _) = common_test::commit(&repo, "file");
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag_name = "tag1";
        repo.tag(tag_name, &obj, &sig, "msg", false).unwrap();

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

        let (commit, _) = common_test::commit(&repo, "file2");
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag1 = "tag1";
        repo.tag(tag1, &obj, &sig, "msg", false).unwrap();

        common_test::commit(&repo, "file3");
        common_test::commit(&repo, "file4");

        let (commit, _) = common_test::commit(&repo, "file5");
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag2 = "tag2";
        repo.tag(tag2, &obj, &sig, "msg", false).unwrap();

        common_test::commit(&repo, "file5");
        common_test::commit(&repo, "file6");

        let ws = Workspace::new(&dir)?;
        let tag = ws.previous_tag(tag2)?;
        assert_eq!(tag1.to_string(), tag);
        Ok(())
    }
}
