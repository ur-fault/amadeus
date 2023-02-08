use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct GlobalArgs {
    #[clap(subcommand)]
    pub(crate) action: GlobalAction,
}

#[derive(Debug, Subcommand)]
pub(crate) enum GlobalAction {
    Check,
    Install(InstallArguments),
    Remove,
    Show(ShowArguments),
    Info(InfoArgumnets),
}

#[derive(Debug, Args)]
pub(crate) struct InstallArguments {
    #[clap(required_unless_present = "path")]
    pub address: Option<String>,
    #[clap(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct InfoArgumnets {
    #[clap(short, long, conflicts_with = "path")]
    pub name: Option<String>,
    #[clap(short, long, conflicts_with = "name", required = false, default_value = ".")]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct ShowArguments {
    #[clap(short = 'i', long, default_value = "false", conflicts_with_all = ["install_path"], required = true)]
    pub installed_packages: bool,
    #[clap(short = 'p', long, default_value = "false")]
    pub install_path: bool,
}