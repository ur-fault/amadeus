use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub struct Package {
    pub name: String,         // Name of the package
    pub description: String,  // Description of the package
    pub authors: Vec<String>, // Authors of the package
    #[serde(default)]
    pub init: RunCommands, // Commands to run to initialize the package
    pub run: RunCommands,     // Commands to run
    #[serde(default)]
    pub checks: Checks, // Checks if required programs are available
}

#[derive(Debug, Deserialize, Clone, Serialize, Default, PartialEq, Eq)]
pub struct Checks {
    pub global: Vec<Command>, // Default checks to run
    #[serde(default)]
    pub win: Vec<Command>, // Checks to run on windows
    #[serde(default)]
    pub linux: Vec<Command>, // Checks to run on linux
    #[serde(default)]
    pub mac: Vec<Command>, // Checks to run on mac
}

impl Display for Checks {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let print_checks = |cmds: &[Command]| -> Result<(), std::fmt::Error> {
            for command in cmds {
                write!(f, "\t{}", command)?;
            }

            Ok(())
        };

        if self.global.is_empty() {
            
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone, Default, Serialize, PartialEq, Eq)]
pub struct RunCommands {
    pub default: Option<Command>, // Default command to run
    #[serde(default)]
    pub win: RunCommand, // Command to run on windows
    #[serde(default)]
    pub linux: RunCommand, // Command to run on linux
    #[serde(default)]
    pub mac: RunCommand, // Command to run on mac
}

impl Display for RunCommands {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Default: {}",
            self.default
                .clone()
                .map(|c| format!("{}", c))
                .unwrap_or("No default run command".to_string())
        )?;
        write!(f, "Windows: {}", self.win)?;
        write!(f, "Linux: {}", self.linux)?;
        write!(f, "Mac: {}", self.mac)
    }
}

#[derive(Debug, Deserialize, Clone, Default, Serialize, PartialEq, Eq)]
pub enum RunCommand {
    #[serde(rename = "null")]
    Null, // Command can't be run on this platform
    #[default]
    #[serde(rename = "default")]
    Default, // Command is the same as the default
    #[serde(rename = "custom")]
    Custom(Command), // Command is custom
}

impl Display for RunCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RunCommand::Null => write!(f, "Repo cannot be run on this platform"),
            RunCommand::Default => write!(f, "Same as default command"),
            RunCommand::Custom(command) => write!(f, "{}", command),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub struct Command {
    pub program: String, // Program to run
    #[serde(default)]
    pub args: Vec<String>, // Arguments to pass to the program
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.program)?;
        for arg in &self.args {
            if arg.contains(" ") {
                write!(f, " \"{}\"", arg)?;
            } else {
                write!(f, " {}", arg)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::from_str as from_yaml;

    #[test]
    fn package_from_yaml_simple() {
        let yaml = r#"
            name: Test Package
            description: This is a test package
            authors:
            - ur-fault
            init:
              default:
                program: cargo
                args:
                - build
                - --release
              win: default
              linux: default
              mac: default
            run:
              default:
                program: cargo
                args:
                - run
                - --release
              win: default
              linux: default
              mac: default
            checks:
              global:
              - program: cargo
                args:
                - --version
              win: []
              linux: []
              mac: []"#;

        let package_yaml: Package = from_yaml(yaml).unwrap();
        let package = Package {
            name: "Test Package".to_string(),
            description: "This is a test package".to_string(),
            authors: vec!["ur-fault".to_string()],
            init: RunCommands {
                default: Some(Command {
                    program: "cargo".to_string(),
                    args: ["build", "--release"]
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                }),
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
                ..Default::default()
            },
            checks: Checks {
                global: vec![Command {
                    program: "cargo".to_string(),
                    args: ["--version"].into_iter().map(str::to_string).collect(),
                }],
                ..Default::default()
            },
        };

        assert_eq!(package_yaml, package);
    }

    #[test]
    fn package_from_yaml_full() {
        let yaml = r#"
            name: Test Package
            description: This is a test package
            authors:
            - ur-fault
            - ur-fault2
            init:
              default:
                program: cargo
                args:
                - build
                - --release
            run:
              default:
                program: cargo
                args:
                - run
                - --release
              win: !custom
                program: cargo
                args:
                - run
                - --release
            checks:
              global:
              - program: cargo
                args:
                - --version"#;

        let package_yaml: Option<Package> = from_yaml(yaml).ok();
        let package = Package {
            name: "Test Package".to_string(),
            description: "This is a test package".to_string(),
            authors: ["ur-fault", "ur-fault2"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            init: RunCommands {
                default: Some(Command {
                    program: "cargo".to_string(),
                    args: ["build", "--release"]
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                }),
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
                    args: ["run", "--release"]
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                }),
                ..Default::default()
            },
            checks: Checks {
                global: vec![Command {
                    program: "cargo".to_string(),
                    args: ["--version"].into_iter().map(str::to_string).collect(),
                }],
                ..Default::default()
            },
        };

        assert_eq!(package_yaml, Some(package));
    }
}
