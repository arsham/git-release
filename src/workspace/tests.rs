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
        let (commit, _) = common_test::commit(&repo);
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
        let (commit, _) = common_test::commit(&repo);
        let obj = repo.find_object(commit, None).unwrap();
        let sig = repo.signature().unwrap();
        let tag_name = "tag1";
        repo.tag(tag_name, &obj, &sig, "msg", false).unwrap();

        let (commit, _) = common_test::commit(&repo);
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
