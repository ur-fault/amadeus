use git2::Repository;
use lazy_regex::regex;
use std::path::{Path, PathBuf};

const DEFAULT_DOMAIN: &str = "github.com";

mod errors {
    use std::fmt::Display;

    use thiserror::Error;

    #[derive(Debug, Error)]
    pub struct InvalidGitAddress;

    impl Display for InvalidGitAddress {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Invalid git address")
        }
    }

    #[derive(Debug, Error)]
    pub enum GitCloneError {
        #[error("Cannot clone repository")]
        CannotClone(#[from] git2::Error),
        #[error("IO error")]
        Io(#[from] std::io::Error),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GitSource {
    domain: String,
    user: String,
    name: String,
    spec: Option<GitSpecifier>,
}

impl GitSource {
    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn spec(&self) -> Option<&GitSpecifier> {
        self.spec.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GitSpecifier {
    Branch(String),
    Tag(String),
    Commit(String),
}

fn parse_git_address(addr: impl AsRef<str>) -> Option<GitSource> {
    let r = regex!(
        "^((?P<domain>(([a-zA-Z]{1})|([a-zA-Z]{1}[a-zA-Z]{1})|([a-zA-Z]{1}[0-9]{1})|([0-9]{1}[a-zA-Z]{1})|([a-zA-Z0-9][a-zA-Z0-9-_]{1,61}[a-zA-Z0-9]))\\.([a-zA-Z]{2,6}|[a-zA-Z0-9-]{2,30}\\.[a-zA-Z]{2,3})):)?(?P<user>[\\w_-]+)/(?P<name>[\\w_-]+)((?P<spectype>[@#$])(?P<spec>\\w+))?$"
    );

    let caps = r.captures(addr.as_ref())?;

    Some(GitSource {
        domain: caps
            .name("domain")
            .map(|d| d.as_str())
            .unwrap_or(DEFAULT_DOMAIN)
            .to_string(),
        user: caps.name("user").unwrap().as_str().to_string(),
        name: caps.name("name").unwrap().as_str().to_string(),
        spec: match caps.name("spectype") {
            Some(t) => match t.as_str() {
                "$" => Some(GitSpecifier::Branch(
                    caps.name("spec").unwrap().as_str().to_string(),
                )),
                "@" => Some(GitSpecifier::Tag(
                    caps.name("spec").unwrap().as_str().to_string(),
                )),
                "#" => Some(GitSpecifier::Commit(
                    caps.name("spec").unwrap().as_str().to_string(),
                )),
                _ => unreachable!(),
            },
            None => None,
        },
    })
}

fn clone_repo(
    source: &GitSource,
    path: impl AsRef<Path>,
    force: bool,
) -> Result<bool, errors::GitCloneError> {
    let mut user = source.user();
    // We can't have user named "_local" because it's reserved for local packages
    if user == "_local" {
        user = "__local";
    }

    let path = path.as_ref().join(user).join(source.name());

    if path.exists() {
        if force {
            std::fs::remove_dir_all(&path)?;
        } else {
            return Ok(false);
        }
    }

    let url = format!(
        "https://{}/{}/{}",
        source.domain(),
        source.user(),
        source.name()
    );

    Repository::clone(&url, &path)?;

    Ok(true)
}

enum PackageSourceInner {
    Git(GitSource),
    Local(PathBuf),
}

pub struct PackageSource {
    inner: PackageSourceInner,
}

impl PackageSource {
    pub fn new_git(address: String) -> Result<Self, errors::InvalidGitAddress> {
        if let Some(source) = parse_git_address(address) {
            Ok(Self {
                inner: PackageSourceInner::Git(source),
            })
        } else {
            Err(errors::InvalidGitAddress)
        }
    }

    pub fn new_local(path: impl AsRef<Path>) -> Self {
        Self {
            inner: PackageSourceInner::Local(path.as_ref().to_path_buf()),
        }
    }

    fn put_to(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        match &self.inner {
            PackageSourceInner::Git(_) => todo!(),
            PackageSourceInner::Local(p) => {
                std::fs::copy(p, path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_address() {
        assert_eq!(
            parse_git_address("github.com:ur-fault/run-that@tag"),
            Some(GitSource {
                domain: "github.com".to_string(),
                user: "ur-fault".to_string(),
                name: "run-that".to_string(),
                spec: Some(GitSpecifier::Tag("tag".to_string())),
            })
        );

        assert_eq!(
            parse_git_address("gitlab.com:ur-fault/run-that#abc123"),
            Some(GitSource {
                domain: "gitlab.com".to_string(),
                user: "ur-fault".to_string(),
                name: "run-that".to_string(),
                spec: Some(GitSpecifier::Commit("abc123".to_string())),
            })
        );
        assert_eq!(
            parse_git_address("codeberg.com:ur-fault/lil-game$branch"),
            Some(GitSource {
                domain: "codeberg.com".to_string(),
                user: "ur-fault".to_string(),
                name: "lil-game".to_string(),
                spec: Some(GitSpecifier::Branch("branch".to_string())),
            })
        );
        assert_eq!(
            parse_git_address("codeberg.com:ur-fault/lil-game"),
            Some(GitSource {
                domain: "codeberg.com".to_string(),
                user: "ur-fault".to_string(),
                name: "lil-game".to_string(),
                spec: None,
            })
        );
        assert_eq!(
            parse_git_address("ur-fault/lil-game$asd"),
            Some(GitSource {
                domain: DEFAULT_DOMAIN.to_string(),
                user: "ur-fault".to_string(),
                name: "lil-game".to_string(),
                spec: Some(GitSpecifier::Branch("asd".to_string())),
            })
        );
        assert_eq!(parse_git_address(":ur-fault/lil-game$asd"), None);
        assert_eq!(parse_git_address("ur-fault/lil-game$"), None);
    }
}
