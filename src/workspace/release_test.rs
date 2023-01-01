use std::collections::HashMap;

use super::Release;
use crate::common_test;
use crate::workspace::commit::{Commit, Verb};

mod get_verb_groups {
    use super::*;

    #[test]
    fn no_commits() -> Result<(), Box<dyn std::error::Error>> {
        let release = Release::new(vec![]);
        assert!(release.get_verb_groups().is_empty());
        Ok(())
    }

    #[test]
    fn one_commit() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let body = "feat(repo): the title\n\nThe body.\n\nThe footer. Ref #123";
        let (oid, _) = common_test::commit(&repo, "filename", Some(body));
        let commit = repo.find_commit(oid)?;

        let release: Release = vec![commit.clone()].into();
        let commit: Commit = commit.into();

        let got = release.get_verb_groups();
        let got = got
            .get(&Verb::Feature)
            .ok_or("not found")?
            .get(0)
            .ok_or("not found")?;
        assert_eq!(commit, **got);
        Ok(())
    }

    #[test]
    fn multiple_verbs() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;

        let msg = "feat(repo): one";
        let (oid, _) = common_test::commit(&repo, "filename1", Some(msg));
        let commit1 = repo.find_commit(oid)?;

        let msg = "fix(repo): two";
        let (oid, _) = common_test::commit(&repo, "filename2", Some(msg));
        let commit2 = repo.find_commit(oid)?;

        let msg = "feat(repo): three";
        let (oid, _) = common_test::commit(&repo, "filename3", Some(msg));
        let commit3 = repo.find_commit(oid)?;

        let release: Release = vec![commit1.clone(), commit2.clone(), commit3.clone()].into();
        let mut want = HashMap::<Verb, Vec<&Commit>>::new();
        let commit1 = &commit1.into();
        let commit2 = &commit2.into();
        let commit3 = &commit3.into();
        want.insert(Verb::Feature, vec![commit1, commit3]);
        want.insert(Verb::Fix, vec![commit2]);

        let got = release.get_verb_groups();
        assert_eq!(want, got);
        Ok(())
    }
}
