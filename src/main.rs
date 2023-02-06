use clap::Parser;
use once_cell::sync::Lazy;
use path_absolutize::Absolutize;
use std::path::PathBuf;

mod cli;

static MAIN_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("~/.run-that/").absolutize().unwrap().into());
static BIN_PATH: Lazy<PathBuf> = Lazy::new(|| MAIN_PATH.join("bin"));

fn main() {
    let args = cli::GlobalArgs::parse();

    #[cfg(debug_assertions)]
    println!("{:?}", args);

    match args.action {
        cli::GlobalAction::Check => println!("Check"),
        cli::GlobalAction::Install => println!("Install"),
        cli::GlobalAction::Remove => println!("Remove"),
        cli::GlobalAction::Show(args) => {
            if args.install_path {
                println!("Binaries are installed in: {:?}", BIN_PATH);
            } else if args.installed_packages {
                println!("Installed packages:");
            }
        }
    }
}
