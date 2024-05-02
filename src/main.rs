use std::{
    env,
    ffi::OsStr,
    fs::{read_dir, read_to_string, write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Error, Result};
use clap::{ArgGroup, Parser};
use itertools::Itertools;
use toml::{Table, Value};
use tracing::warn;

const THEME_DIR_NAME: &str = "./themes/themes/";
const CONFIG_FILE_PATHS: &[&str] = &["./alacritty.toml", "./alacritty/alacritty.toml"];

const CONFIG_IMPORT_KEY: &str = "import";

// alacritty-theme-switcher
///
/// Change the color theme in your alacritty.toml based on the themes available in .config/alacritty/themes/themes/
#[derive(Parser, Debug, Clone)]
#[command(version, about, author)]
#[command(group(ArgGroup::new("mode").required(true).args(&["theme", "list"])))]
struct Args {
    /// Name of the theme to activate. Must be a .toml file in theme_dir
    theme: Option<String>,
    /// List all available themes instead of switching
    #[arg(short, long)]
    list: bool,
    /// The directory in which themes are stored. Defaults to $XDG_CONFIG_HOME/alacritty/themes/themes/
    #[arg(short('d'), long)]
    theme_dir: Option<PathBuf>,
    #[arg(short, long)]
    /// alacritty.toml config file to use. Defaults to $XDG_CONFIG_HOME/alacritty/alacritty.toml
    config_file: Option<PathBuf>,
}

fn expand_home(path: &Path) -> Result<PathBuf, Error> {
    env::var("HOME")
        .context("Could not expand ~ because $HOME is not set")
        .map(|home| PathBuf::from(home).join(path.strip_prefix("~/").unwrap_or(path)))
}

fn list_themes(theme_dir: &Path) -> Result<()> {
    let themes = read_dir(theme_dir)
        .context(format!(
            "could not read themes directory: {}",
            theme_dir.display()
        ))?
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or(OsStr::new("")) == "toml" {
                    path.file_stem()
                        .map(|stem| stem.to_string_lossy().to_string())
                } else {
                    None
                }
            }
            Err(e) => {
                warn!("Ignoring file due to read error: {e}");
                None
            }
        })
        .collect_vec();
    println!("Available themes:");
    for theme in &themes {
        println!("{}", theme);
    }
    Ok(())
}

fn set_theme_file(theme_file: &Path, config_file: &Path) -> Result<()> {
    let mut config: Table = read_to_string(config_file)
        .context("could not read alacritty config file")?
        .parse()?;
    let mut updated_imports = config[CONFIG_IMPORT_KEY]
        .as_array()
        .context("'imports' in alacritty config is not in expected format")?
        .iter()
        .filter_map(|path| {
            let Some(path) = path.as_str() else {
                return Some(Err(anyhow!(
                    "invalid imports content in alacritty config: {path:?}"
                )));
            };
            let path = match expand_home(&PathBuf::from(path)) {
                Ok(path) => path,
                Err(e) => {
                    return Some(Err(anyhow!(e)));
                }
            };
            // safe, we select all files from a directory
            if !path.starts_with(theme_file.parent().unwrap()) {
                Some(Ok(Value::String(path.to_string_lossy().to_string())))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    updated_imports.push(Value::String(theme_file.to_string_lossy().to_string()));

    config[CONFIG_IMPORT_KEY] = Value::Array(updated_imports);
    write(
        config_file,
        toml::to_string_pretty(&config).context("could not write alacritty config")?,
    )
    .context("could not write alacritty config")?;
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let config_home = expand_home(&PathBuf::from(env::var("XDG_CONFIG_HOME").unwrap_or(
        format!(
            "{}/.config/",
            env::var("HOME")
                .context("Could not determine config directory because $HOME is not set")?
        ),
    )))?;

    let theme_dir = args.theme_dir.unwrap_or(
        PathBuf::from(&config_home)
            .join("./alacritty/")
            .join(THEME_DIR_NAME)
            .canonicalize()
            .context("Could not determine themes path".to_string())?,
    );
    if args.list {
        return list_themes(&theme_dir);
    }

    let theme = args.theme.unwrap(); // safe because of claps group assignment
    let theme_file = theme_dir.join(format!("{}.toml", theme));
    if !theme_file.exists() {
        return Err(anyhow!(
            "theme '{}' not found ('{}' does not exist)!",
            theme,
            theme_file.display()
        ));
    }

    let config_file = args.config_file.unwrap_or(
        CONFIG_FILE_PATHS
            .iter()
            .map(|loc| config_home.clone().join(loc))
            .find(|path| path.exists())
            .context("could not find alacritty.toml config file")?,
    );

    set_theme_file(&theme_file, &config_file)
}
