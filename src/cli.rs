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
    Install,
    Remove,
    Show(ShowArguments),
}

#[derive(Debug, Args)]
pub(crate) struct ShowArguments {
    #[clap(short = 'i', long, default_value = "false", conflicts_with_all = ["install_path"], required = true)]
    pub installed_packages: bool,
    #[clap(short = 'p', long, default_value = "false")]
    pub install_path: bool,
}
