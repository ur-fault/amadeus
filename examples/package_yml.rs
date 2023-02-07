use run_that::package::{Command, CommandSet, Package, RunCommand, RunCommands};
use serde_yaml::to_string as to_yaml;

fn main() {
    let package = Package {
        name: "Test Package".to_string(),
        description: "This is a test package".to_string(),
        authors: vec!["ur-fault".to_string()],
        init: CommandSet {
            global: vec![Command {
                program: "cargo".to_string(),
                args: ["build", "--release"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
            }],
            ..Default::default()
        },
        run: RunCommands {
            default: Some(Command {
                program: "cargo".to_string(),
                args: ["run", "--release"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
            }),
            win: RunCommand::Custom(Command {
                program: "cargo".to_string(),
                args: vec![],
            }),
            ..Default::default()
        },
        checks: CommandSet {
            global: vec![Command {
                program: "cargo".to_string(),
                args: ["--version"].into_iter().map(str::to_string).collect(),
            }],
            ..Default::default()
        },
    };

    println!(
        "{}",
        to_yaml(&package).unwrap_or("Could not convert to yaml".to_string())
    );
}
