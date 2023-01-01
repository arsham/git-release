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
