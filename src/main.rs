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

fn load_config() -> Result<Config, anyhow::Error> {
    let content = std::fs::read_to_string("advrunner.toml")
        .context("unable to read the advrunner config file")?;
    let config = toml::from_str(&content).context("unable to parse the advrunner config file")?;
    Ok(config)
}

fn detect_is_test() -> Result<bool, anyhow::Error> {
    let mut args = std::env::args().skip(1);
    let Some(file) = args.next() else {
        anyhow::bail!("no file passed as an argument");
    };
    // The path contains deps, must be a test.
    Ok(file.contains("/deps/"))
}

fn exec_profile(profile: &Profile) -> Result<(), anyhow::Error> {
    let args = std::env::args_os().skip(1);
    duct::cmd(&profile.command, args).run()?;
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let config = load_config()?;
    let is_test = detect_is_test()?;
    let profile = if is_test { config.test } else { config.run };
    exec_profile(&profile)?;
    Ok(())
}
