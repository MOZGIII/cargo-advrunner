use std::path::{Path, PathBuf};

use anyhow::Context;

#[derive(Debug, serde::Deserialize)]
struct Config {
    pub run: Profile,
    pub test: Profile,
}

#[derive(Debug, serde::Deserialize)]
struct Profile {
    pub command: String,
}

const CONFIG_FILE_PATH: &str = "advrunner.toml";

fn load_config(dir: impl AsRef<Path>) -> Result<Config, anyhow::Error> {
    let path = dir.as_ref().join(CONFIG_FILE_PATH);
    let content = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "unable to read the advrunner config file from {} while at {}",
            path.display(),
            std::env::current_dir().unwrap().display(),
        )
    })?;
    let config = toml::from_str(&content).with_context(|| {
        format!(
            "unable to parse the advrunner config file from {} while at {}",
            path.display(),
            std::env::current_dir().unwrap().display(),
        )
    })?;
    Ok(config)
}

fn locate_cargo_workspace_root() -> Result<String, anyhow::Error> {
    let root = duct::cmd(
        "cargo",
        ["locate-project", "--workspace", "--message-format", "plain"],
    )
    .read()
    .context("unable to read cargo metadata")?;
    let root = PathBuf::from(root)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    Ok(root)
}

fn detect_is_test() -> Result<bool, anyhow::Error> {
    let mut args = std::env::args().skip(1);
    let Some(file) = args.next() else {
        anyhow::bail!("no file passed as an argument");
    };
    let path = PathBuf::from(file);
    // The path contains "deps", must be a test.
    Ok(path.into_iter().any(|item| item == "deps"))
}

fn exec_profile(profile: Profile, at: String) -> Result<(), anyhow::Error> {
    let args = std::env::args_os().skip(1);
    duct::cmd(&profile.command, args).dir(at).run()?;
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let cargo_workspace_root = locate_cargo_workspace_root()?;
    let config = load_config(&cargo_workspace_root)?;
    let is_test = detect_is_test()?;
    let profile = if is_test { config.test } else { config.run };
    exec_profile(profile, cargo_workspace_root)?;
    Ok(())
}
