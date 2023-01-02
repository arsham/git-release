use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()?;
    let current_sha = String::from_utf8(current_sha.stdout)?;

    let build_tag = Command::new("git")
        .args(["describe", "--abbrev", "--tags"])
        .output()?;
    let build_tag = String::from_utf8(build_tag.stdout)?;

    println!("cargo:rustc-env=CURRENT_SHA={current_sha}");
    println!("cargo:rustc-env=APP_VERSION={build_tag}");

    // let out_dir = std::path::PathBuf::from(
    //     std::env::var_os("OUT_DIR").ok_or("Env variable OUT_DIR is not set")?,
    // );
    //
    // let cmd = clap::Command::new("git-release")
    //     .arg(clap::arg!(-t --tag <TAG>))
    //     .arg(clap::arg!(-V --version <V>));
    //
    // let man = clap_mangen::Man::new(cmd);
    // let mut buffer: Vec<u8> = Default::default();
    // man.render(&mut buffer)?;
    //
    // std::fs::write(out_dir.join("git-release.1"), buffer)?;

    Ok(())
}
