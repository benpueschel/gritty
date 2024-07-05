use core::fmt;
use std::{collections::HashMap, env, error::Error, fmt::Display, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::remote::{Auth, Provider, RemoteConfig};

pub type Result<T> = std::result::Result<T, ConfigError>;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigError {
    pub message: String,
    pub kind: ConfigErrorKind,
}
impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for ConfigError {}
impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
            kind: ConfigErrorKind::ConfigParseError,
        }
    }
}
impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self {
            message: value.message().to_string(),
            kind: ConfigErrorKind::ConfigParseError,
        }
    }
}
impl From<toml::ser::Error> for ConfigError {
    fn from(value: toml::ser::Error) -> Self {
        Self {
            message: value.to_string(),
            kind: ConfigErrorKind::ConfigParseError,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigErrorKind {
    ConfigNotFound,
    ConfigParseError,
    RemoteNotFound,
    AuthNotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// A list of remotes.
    /// Maps the remote name to the remote configuration.
    pub remotes: HashMap<String, GitRemoteConfig>,
    pub secrets: Secrets,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitRemoteConfig {
    pub provider: Provider,
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Secrets {
    /// Use the system keyring to store secrets.
    /// This works on Linux, macOS, Windows, and probably on BSD variants.
    Keyring,
    SecretsFile(String),
    Plaintext(HashMap<String, Auth>),
}

impl Config {
    pub fn save_default() -> Result<()> {
        let config = Config::default();
        let toml = toml::to_string(&config)?;
        let home = env::var("HOME").unwrap();
        let xdg_config = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{home}/.config"));
        let config_path = format!("{xdg_config}/gitrc-rs/config.toml");
        fs::create_dir_all(Path::new(&config_path).parent().unwrap())?;
        fs::write(&config_path, toml)?;
        println!("Saved default config to '{config_path}'");
        Ok(())
    }
    pub fn load_from_file(path: Option<String>) -> Result<Self> {
        let path = match path {
            Some(path) => path,
            None => {
                let home = env::var("HOME").unwrap();
                let xdg_config = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{home}/.config"));
                let config_path = format!("{xdg_config}/gitrc-rs/config.toml");
                let fallback = format!("{home}/.gitrc-rs.toml");

                if Path::new(&config_path).exists() {
                    config_path
                } else {
                    if !Path::new(&fallback).exists() {
                        return Err(ConfigError {
                            message: format!(
                                "Could not find config at '{config_path}' or '{fallback}'."
                            ),
                            kind: ConfigErrorKind::ConfigNotFound,
                        });
                    }
                    fallback
                }
            }
        };
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn get_remote_provider(&self, name: &str) -> Result<Provider> {
        if let Some(remote) = self.remotes.get(name) {
            return Ok(remote.provider.clone());
        }
        Err(ConfigError {
            message: format!("Could not find remote '{name}'"),
            kind: ConfigErrorKind::RemoteNotFound,
        })
    }
    pub fn get_remote_config(&self, name: &str) -> Result<RemoteConfig> {
        if let Some(remote) = self.remotes.get(name) {
            return Ok(RemoteConfig {
                username: remote.username.clone(),
                url: remote.url.clone(),
                auth: self.get_auth(name)?,
            });
        }
        Err(ConfigError {
            message: format!("Could not find remote '{name}'"),
            kind: ConfigErrorKind::RemoteNotFound,
        })
    }
    pub fn get_auth(&self, name: &str) -> Result<Auth> {
        match &self.secrets {
            Secrets::Plaintext(secrets) => {
                if let Some(auth) = secrets.get(name) {
                    return Ok(auth.clone());
                }
            }
            _ => unimplemented!(),
        }
        Err(ConfigError {
            message: format!(
                r#"Could not find auth for remote '{name}'.
                Did you forget to add it to the config?"#
            ),
            kind: ConfigErrorKind::AuthNotFound,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            remotes: HashMap::from([
                (
                    "gitea".to_string(),
                    GitRemoteConfig {
                        provider: Provider::Gitea,
                        url: "https://gitea.example.com".to_string(),
                        username: "awesome-user".to_string(),
                    },
                ),
                (
                    "github".to_string(),
                    GitRemoteConfig {
                        provider: Provider::GitHub,
                        url: "https://github.com".to_string(),
                        username: "awesome-user".to_string(),
                    },
                ),
            ]),
            secrets: Secrets::Plaintext(HashMap::from([
                (
                    "gitea".to_string(),
                    Auth::Token {
                        token: "gitea-token".to_string(),
                    },
                ),
                (
                    "github".to_string(),
                    Auth::Token {
                        token: "github-token".to_string(),
                    },
                ),
            ])),
        }
    }
}
