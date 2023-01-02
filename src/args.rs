use structopt::clap::AppSettings::{ColorAuto, ColoredHelp, DisableVersion};
use structopt::StructOpt;
/// Options for running git-release.
#[derive(StructOpt, Debug)]
#[structopt(name = "git-release", about = "Make a github release for tags")]
#[structopt(no_version, global_settings = &[DisableVersion])]
#[structopt(setting(ColorAuto), setting(ColoredHelp))]
pub struct Opt {
    #[structopt(subcommand)]
    pub sub_commands: Option<Command>,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Print the application version.
    Version,
}

impl Opt {
    pub fn new() -> Opt {
        Opt::from_args()
    }
}
