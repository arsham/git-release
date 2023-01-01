use crate::{common_test, workspace::commit::Commit};

#[cfg(test)]
mod title {
    use super::*;

    #[test]
    fn message_is_one_line() -> Result<(), Box<dyn std::error::Error>> {
        let title = "this is a title";
        let body = format!("{title}\n\nThis is the body");
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let (oid, _) = common_test::commit(&repo, "filename", Some(&body));

        let commit: Commit = repo.find_commit(oid)?.into();
        let res = commit.title().ok_or("no title")?;
        assert_eq!(title, res);
        Ok(())
    }
}

#[cfg(test)]
mod verb {
    use super::*;

    #[test]
    fn no_verbs() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            "nothing is here",
            "(and): so this one",
            "title\n\nignore: this",
        ];
        for body in tcs {
            let (dir, _) = common_test::repo_init();
            let repo = git2::Repository::open(&dir)?;
            let (oid, _) = common_test::commit(&repo, "filename", Some(body));

            let commit: Commit = repo.find_commit(oid)?.into();
            let res = commit.verb();
            assert_eq!("Misc", res, "{body}");
        }
        Ok(())
    }

    #[test]
    fn verb_with_no_section() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            ("feat", "Feature"),
            ("feature", "Feature"),
            ("fix", "Fix"),
            ("fixed", "Fix"),
            ("fixes", "Fix"),
            ("ref", "Refactor"),
            ("refactor", "Refactor"),
            ("refactored", "Refactor"),
            ("chore", "Chore"),
            ("enhance", "Enhancements"),
            ("enhanced", "Enhancements"),
            ("enhancement", "Enhancements"),
            ("enhancements", "Enhancements"),
            ("improve", "Enhancements"),
            ("improved", "Enhancements"),
            ("improves", "Enhancements"),
            ("improvement", "Enhancements"),
            ("improvements", "Enhancements"),
            ("style", "Style"),
            ("ci", "CI"),
            ("CI", "CI"),
            ("doc", "Documentation"),
            ("docs", "Documentation"),
        ];
        for tc in tcs {
            let (dir, _) = common_test::repo_init();
            let repo = git2::Repository::open(&dir)?;
            let body = format!("{} something", tc.0);
            let (oid, _) = common_test::commit(&repo, "filename", Some(&body));

            let commit: Commit = repo.find_commit(oid)?.into();
            let res = commit.verb();
            assert_eq!(tc.1, res, "{}", tc.0);
        }
        Ok(())
    }

    #[test]
    fn verb_with_section() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            "(repo) something",
            "(s3) something",
            "(repo): something",
            "(repo,server) something",
            "(s3,server) something",
            "(server,s3) something",
            "(repo, server) something",
            "(s3, server) something",
            "(server, s3) something",
            "(repo.exec): something",
            "(s3.exec): something",
            "(exec.s3): something",
            "(repo-exec): something",
            "(s3-exec): something",
            "(exec-s3): something",
            "(repo_exec): something",
            "(s3_exec): something",
            "(repo exec): something",
            "(s3 exec): something",
            "(repo/exec): something",
            "(s3/exec): something",
            "(exec/s3): something",
        ];
        for tc in tcs {
            let (dir, _) = common_test::repo_init();
            let repo = git2::Repository::open(&dir)?;
            let body = format!("feat{tc}");
            let (oid, _) = common_test::commit(&repo, "filename", Some(&body));

            let commit: Commit = repo.find_commit(oid)?.into();
            let res = commit.verb();
            assert_eq!("Feature", res);
        }
        Ok(())
    }
}

#[cfg(test)]
mod references {
    use super::*;
    use crate::workspace::commit::Reference;

    #[test]
    fn no_references() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            "feat(commit): this is a title",
            "feat(commit): 123 this is a title",
            "feat(commit): #,123 this is a title",
            "feat(commit): # 123 this is a title",
            "feat(commit): #-123 this is a title",
            "feat(commit): #a123 this is a title",
        ];
        for body in tcs {
            let (dir, _) = common_test::repo_init();
            let repo = git2::Repository::open(&dir)?;
            let (oid, _) = common_test::commit(&repo, "filename", Some(body));
            let commit: Commit = repo.find_commit(oid)?.into();
            assert_eq!(Vec::<Reference>::new(), commit.references(), "{body}");
        }
        Ok(())
    }

    #[test]
    fn in_summary() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let body = "feat(commit): this links to #123\n\nSomething is here";
        let (oid, _) = common_test::commit(&repo, "filename", Some(body));
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123)], commit.references(), "{body}");
        Ok(())
    }

    #[test]
    fn in_start_of_line() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let body = "feat:something 23\n\n#123: is the ref";
        let (oid, _) = common_test::commit(&repo, "filename", Some(body));
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123)], commit.references(), "{body}");
        Ok(())
    }

    #[test]
    fn at_the_end_of_line() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let body = "feat:something 23\n\nRef #123";
        let (oid, _) = common_test::commit(&repo, "filename", Some(body));
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123)], commit.references());
        Ok(())
    }

    #[test]
    fn multiple_refs_in_line() -> Result<(), Box<dyn std::error::Error>> {
        let (dir, _) = common_test::repo_init();
        let repo = git2::Repository::open(&dir)?;
        let body = "feat:something 23\n\nRef #123, Close #456";
        let (oid, _) = common_test::commit(&repo, "filename", Some(body));
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123), Reference(456)], commit.references());
        Ok(())
    }
}
