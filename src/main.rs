use anyhow::{Context, Result};
use clap::Parser;
use std::{fs, path::PathBuf};
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "vexwm", version, about = "A lightweight configurable Wayland compositor")]
struct Args {
    /// Validate a configuration and exit.
    #[arg(long)]
    check: bool,
    /// Configuration path.
    #[arg(short, long, env = "VEXWM_CONFIG")]
    config: Option<PathBuf>,
}

fn config_path(arg: Option<PathBuf>) -> PathBuf {
    if let Some(path) = arg { return path; }
    if let Ok(path) = std::env::var("VEXWM_CONFIG") { return PathBuf::from(path); }
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap()).join(".config"));
    base.join("vexwm/config.kdl")
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let args = Args::parse();
    let path = config_path(args.config);
    let text = fs::read_to_string(&path)
        .with_context(|| format!("cannot read configuration: {}", path.display()))?;
    let document: kdl::KdlDocument = text.parse()
        .with_context(|| format!("invalid KDL configuration: {}", path.display()))?;
    info!(sections = document.nodes().len(), path = %path.display(), "configuration loaded");
    if args.check {
        println!("configuration is valid: {}", path.display());
        return Ok(());
    }
    eprintln!("VexWM scaffold: configuration parsed successfully.");
    eprintln!("The compositor backend is intentionally the next implementation milestone.");
    Ok(())
}
