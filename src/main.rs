use clap::Parser;
use colored::*;
use run_that::manager::{get_package_info, REPOS_PATH};

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::GlobalArgs::parse();

    #[cfg(debug_assertions)]
    println!("{args:?}");

    match args.action {
        cli::GlobalAction::Check => println!("Check"),
        cli::GlobalAction::Install(_args) => println!("Install"),
        cli::GlobalAction::Remove => println!("Remove"),
        cli::GlobalAction::Show(args) => {
            if args.install_path {
                println!("Repositories are stored in: {REPOS_PATH:?}");
            } else if args.installed_packages {
                println!("Installed repositories:");
            }
        }
        cli::GlobalAction::Info(args) => {
            let path = if let Some(name) = args.name {
                REPOS_PATH.join(name)
            } else if let Some(path) = args.path {
                path
            } else {
                std::env::current_dir().expect("Could not get current directory")
            };

            let package = get_package_info(&path)?;
            println!("{}\n{}", "Package info:".bright_magenta(), package);
        }
    }

    Ok(())
}
