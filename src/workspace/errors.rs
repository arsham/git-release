use thiserror::Error;

/// GRError enumerates all errors for this application.
#[derive(Error, Debug)]
pub enum GRError {
    /// Returned when we can't get the repository information.
    #[error(transparent)]
    Repository(#[from] git2::Error),

    /// Returned when the provided tag is not found in the repository.
    #[error("Could not find the '{0}' tag")]
    TagNotFound(String),

    /// Returned when we can't get the list of the tags from the repository.
    #[error("Could not get the tag list")]
    TagNameList(git2::Error),

    /// Returned when both tags are pointing to the same commit.
    #[error("Both tags are pointing at the same commit")]
    TwinTags,

    #[error("Could not get the url of the repository at '{0}'")]
    URLError(String),
}
