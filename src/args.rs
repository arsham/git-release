use structopt::clap::AppSettings::{ColorAuto, ColoredHelp, DisableVersion};
use structopt::StructOpt;

/// Options for running git-release.
#[derive(StructOpt, Debug)]
#[structopt(name = "git-release", about = "Make a github release for tags")]
#[structopt(no_version, global_settings = &[DisableVersion])]
#[structopt(setting(ColorAuto), setting(ColoredHelp))]
pub struct Opt {
    /// Tag, tags, range of tags. Defaults to latest tag.
    ///
    /// # Examples
    ///
    /// Last tag to the previous one: git release
    ///
    /// Single tag to the previous one: git release -t v0.1.0
    ///
    /// A tag up to the HEAD: git release -t v0.1.0..
    ///
    /// v0.1.0 (excluding) to v0.5.0 (including): git release -t v0.1.0..v0.5.0
    #[structopt(short, long)]
    tag: Option<String>,

    #[structopt(subcommand)]
    pub sub_commands: Option<Command>,

    #[structopt(skip)]
    pub tags: Tag,

    /// The remote to operate on.
    #[structopt(short, long, default_value = "origin")]
    pub remote: String,

    /// Publish the release results to github.
    #[structopt(short, long)]
    pub publish: bool,

    #[structopt(long, env, hide_env_values = true)]
    pub github_token: String,

    /// If publishing fails, try fording it. This will replace the previous contents!
    #[structopt(short, long)]
    pub force: bool,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Print the application version.
    Version,
    /// Publish the release information to github.
    Publish,
}

impl Opt {
    pub fn new() -> Opt {
        let mut opt = Opt::from_args();
        if let Some(ref tag) = opt.tag {
            if !tag.contains("..") {
                opt.tags = Tag::Single(tag.clone());
            } else if tag.ends_with("..") {
                opt.tags = Tag::From(tag.strip_suffix("..").unwrap().to_owned());
            } else {
                let mut splits = tag.split("..");
                let from = splits.next().unwrap().to_owned();
                let to = splits.next().unwrap().to_owned();
                opt.tags = Tag::Range(from, to);
            }
        }
        opt
    }
}

/// Tag is the value of the tag argument provided by the user.
#[derive(Debug)]
pub enum Tag {
    None,
    Single(String),
    From(String),
    Range(String, String),
}

impl Default for Tag {
    fn default() -> Self {
        Tag::None
    }
}
