use crate::common_test::new_commit;
use crate::workspace::commit::Reference;
use crate::workspace::commit::{Commit, Verb};

#[cfg(test)]
mod title {
    use super::*;

    #[test]
    fn message_is_one_line() -> Result<(), Box<dyn std::error::Error>> {
        let title = "this is a title";
        let body = format!("{title}\n\nThis is the body");
        let (repo, oid) = new_commit("filename", &body)?;
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
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            let res = commit.verb();
            assert_eq!(Verb::Misc, res, "{body}");
        }
        Ok(())
    }

    #[test]
    fn verb_with_no_section() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            ("feat", Verb::Feature),
            ("feature", Verb::Feature),
            ("fix", Verb::Fix),
            ("fixed", Verb::Fix),
            ("fixes", Verb::Fix),
            ("ref", Verb::Refactor),
            ("refactor", Verb::Refactor),
            ("refactored", Verb::Refactor),
            ("chore", Verb::Chore),
            ("enhance", Verb::Enhancements),
            ("enhanced", Verb::Enhancements),
            ("enhancement", Verb::Enhancements),
            ("enhancements", Verb::Enhancements),
            ("improve", Verb::Enhancements),
            ("improved", Verb::Enhancements),
            ("improves", Verb::Enhancements),
            ("improvement", Verb::Enhancements),
            ("improvements", Verb::Enhancements),
            ("style", Verb::Style),
            ("ci", Verb::CI),
            ("CI", Verb::CI),
            ("doc", Verb::Documentation),
            ("docs", Verb::Documentation),
        ];
        for tc in tcs {
            let body = format!("{} something", tc.0);
            let (repo, oid) = new_commit("filename", &body)?;
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
            "(server)!: something",
            "!(server): something",
        ];
        for tc in tcs {
            let body = format!("feat{tc}");
            let (repo, oid) = new_commit("filename", &body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            let res = commit.verb();
            assert_eq!(Verb::Feature, res);
        }
        Ok(())
    }
}

#[cfg(test)]
mod references {
    use super::*;

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
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert_eq!(Vec::<Reference>::new(), commit.references(), "{body}");
        }
        Ok(())
    }

    #[test]
    fn in_summary() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat(commit): this links to #123\n\nSomething is here";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123)], commit.references(), "{body}");
        Ok(())
    }

    #[test]
    fn in_start_of_line() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat:something 23\n\n#123: is the ref";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123)], commit.references(), "{body}");
        Ok(())
    }

    #[test]
    fn at_the_end_of_line() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat:something 23\n\nRef #123";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123)], commit.references());
        Ok(())
    }

    #[test]
    fn multiple_refs_in_line() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat:something 23\n\nRef #123, Close #456";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();
        assert_eq!(vec![Reference(123), Reference(456)], commit.references());
        Ok(())
    }
}

#[cfg(test)]
mod subjects {
    use super::*;

    #[test]
    fn no_subject() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            "something",
            "ref something",
            "ref(): something",
            "ref: something",
            "ref: (something)",
            "ref: (something): something",
        ];
        for body in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            let subjects = commit.subjects();
            assert!(subjects.is_none(), "{body} -> {:?}", subjects.unwrap());
        }
        Ok(())
    }

    #[test]
    fn subjects() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            ("ref(repo): something", vec!["repo"]),
            ("ref(Repo): something", vec!["Repo"]),
            ("ref (repo): something", vec!["repo"]),
            ("ref (repo) : something", vec!["repo"]),
            ("ref ( repo ) : something", vec!["repo"]),
            ("ref(repo,server): something", vec!["repo", "server"]),
            ("ref(repo, server): something", vec!["repo", "server"]),
            ("ref(repo ,server): something", vec!["repo", "server"]),
            ("ref(repo server): something", vec!["repo server"]),
            ("ref(repo.server): something", vec!["repo.server"]),
            ("ref(repo .server): something", vec!["repo .server"]),
            ("ref(repo. server): something", vec!["repo. server"]),
            ("ref(repo-server): something", vec!["repo-server"]),
            ("ref(repo -server): something", vec!["repo -server"]),
            ("ref(repo- server): something", vec!["repo- server"]),
            ("ref(repo - server): something", vec!["repo - server"]),
            ("ref(repo_server): something", vec!["repo_server"]),
            ("ref(repo _server): something", vec!["repo _server"]),
            ("ref(repo_ server): something", vec!["repo_ server"]),
            ("ref(repo _ server): something", vec!["repo _ server"]),
            ("ref(repo/server): something", vec!["repo/server"]),
            (
                "ref(repo server, repo): something",
                vec!["repo server", "repo"],
            ),
        ];
        for (body, want) in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            let subjects = commit.subjects();
            assert!(subjects.is_some(), "{body}");
            assert_eq!(&subjects.unwrap(), &want);
        }
        Ok(())
    }
}

#[cfg(test)]
mod is_breaking {
    use super::*;

    #[test]
    fn not_breaking() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            "ref: something 23\n\nRef #123, Close #456",
            "ref: !something 23\n\nRef #123, Close #456",
            "ref(repo): something 23\n\nRef #123, Close #456",
            "ref(repo): something 23\n\nRef #123!, Close #456",
        ];
        for body in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert!(!commit.is_breaking(), "{body}");
        }
        Ok(())
    }

    #[test]
    fn in_title() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            "ref!: something 23\n\nRef #123, Close #456",
            "ref(repo)!: something 23\n\nRef #123, Close #456",
            "ref!(repo): something 23\n\nRef #123, Close #456",
        ];
        for body in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert!(commit.is_breaking(), "{body}");
        }
        Ok(())
    }

    #[test]
    fn in_footer() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            "ref(repo): this is a new api\n\nBREAKING CHANGE: this is a changed api",
            "ref(repo): this is a new api\n\nSomething.\nBREAKING CHANGE: this is a changed api",
        ];
        for body in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert!(commit.is_breaking(), "{body}");
        }
        Ok(())
    }
}

#[cfg(test)]
mod display_fmt {
    use super::*;

    #[test]
    fn just_title() -> Result<(), Box<dyn std::error::Error>> {
        let body = "this is a test\n\nBody.\n\nFooter";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();

        let want = "This is a test";
        assert_eq!(want, format!("{commit}"));

        Ok(())
    }

    #[test]
    fn with_verb() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat: this is a test\n\nBody.\n\nFooter";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();
        let want = "This is a test";
        assert_eq!(want, format!("{commit}"));

        Ok(())
    }

    #[test]
    fn with_subjects() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat(repo, server): this is a test\n\nBody.\n\nFooter";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();

        let want = "**repo, server:** This is a test";
        assert_eq!(want, format!("{commit}"));

        Ok(())
    }

    #[test]
    fn with_breaking_in_summary() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat!: this is a test\n\nBody.\n\nFooter";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();

        let want = "This is a test [**BREAKING CHANGE**]";
        assert_eq!(want, format!("{commit}"));

        Ok(())
    }

    #[test]
    fn with_breaking_in_footer() -> Result<(), Box<dyn std::error::Error>> {
        let body = "feat: this is a test\n\nBody.\n\nBREAKING CHANGE: there was a change";
        let (repo, oid) = new_commit("filename", body)?;
        let commit: Commit = repo.find_commit(oid)?.into();

        let want = "This is a test [**BREAKING CHANGE**]";
        assert_eq!(want, format!("{commit}"));

        Ok(())
    }

    #[test]
    fn issue_in_summary() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            ("title ref #123", "Title (ref #123)"),
            ("title (ref #123)", "Title (ref #123)"),
            ("title close #123", "Title (ref #123)"),
            ("title close #123 ref #456", "Title (ref #123, ref #456)"),
        ];
        for (body, want) in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert_eq!(want, format!("{commit}"));
        }

        Ok(())
    }

    #[test]
    fn issue_in_body() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            ("title\n\nbody ref #123", "Title (ref #123)"),
            ("title\n\nbody (ref #123)", "Title (ref #123)"),
            ("title\n\nbody close #123", "Title (ref #123)"),
            (
                "title\n\nbody close #123 and ref #456",
                "Title (ref #123, ref #456)",
            ),
        ];
        for (body, want) in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert_eq!(want, format!("{commit}"));
        }

        Ok(())
    }

    #[test]
    fn issue_in_footer() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            ("title\n\nbody\n\n ref #123", "Title (ref #123)"),
            ("title\n\nbody\n\n (ref #123)", "Title (ref #123)"),
            ("title\n\nbody\n\n close #123", "Title (ref #123)"),
            (
                "title\n\nbody\n\n close #123 and ref #456",
                "Title (ref #123, ref #456)",
            ),
        ];
        for (body, want) in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert_eq!(want, format!("{commit}"));
        }

        Ok(())
    }

    #[test]
    fn issue_in_all_places() -> Result<(), Box<dyn std::error::Error>> {
        let tcs = vec![
            (
                "title ref #11\n\nbody ref #22.\n\nref #33",
                "Title (ref #11, ref #22, ref #33)",
            ),
            (
                "title (ref #11)\n\nbody (ref #22).\n\n(ref #33)",
                "Title (ref #11, ref #22, ref #33)",
            ),
            (
                "title ref #11\n\nbody ref #22.\nanother ref #33\n\nref #44",
                "Title (ref #11, ref #22, ref #33, ref #44)",
            ),
        ];
        for (body, want) in tcs {
            let (repo, oid) = new_commit("filename", body)?;
            let commit: Commit = repo.find_commit(oid)?.into();
            assert_eq!(want, format!("{commit}"));
        }

        Ok(())
    }
}
