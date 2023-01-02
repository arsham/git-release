mod args;
mod workspace;

#[cfg(test)]
mod common_test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = args::Opt::new();

    if let Some(args::Command::Version) = opt.sub_commands {
        println!(
            "git-release version: {}, git commit: {}",
            env!("APP_VERSION"),
            env!("CURRENT_SHA")
        );
    }
    Ok(())
}
