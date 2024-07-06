use core::fmt;
use std::{collections::HashMap, env, error::Error, fmt::Display, fs, path::Path};

#[cfg(feature = "keyring")]
use keyring::Entry;
use serde::{Deserialize, Serialize};

use crate::{
    log,
    remote::{Auth, CloneProtocol, Provider, RemoteConfig},
};

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
#[cfg(feature = "keyring")]
impl From<keyring::Error> for ConfigError {
    fn from(value: keyring::Error) -> Self {
        Self {
            message: value.to_string(),
            kind: ConfigErrorKind::KeyringError,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigErrorKind {
    ConfigNotFound,
    ConfigParseError,
    RemoteNotFound,
    AuthNotFound,
    SecretsFileNotFound,
    #[cfg(feature = "keyring")]
    KeyringError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// A list of remotes.
    /// Maps the remote name to the remote configuration.
    pub remotes: HashMap<String, GitRemoteConfig>,
    pub secrets: Secrets,
    #[serde(skip)]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitRemoteConfig {
    pub provider: Provider,
    pub clone_protocol: CloneProtocol,
    pub url: String,
    pub username: String,
}

type InlineSecrets = HashMap<String, AuthConfig>;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Secrets {
    /// Use the system keyring to store secrets.
    /// This works on Linux, macOS, Windows, and probably on BSD variants.
    #[cfg(feature = "keyring")]
    Keyring,
    SecretsFile(String),
    Plaintext(InlineSecrets),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
}

impl Config {
    pub fn save_default() -> Result<()> {
        let config = Config::default();
        config.save()
    }
    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string(self)?;
        fs::create_dir_all(Path::new(&self.path).parent().unwrap())?;
        fs::write(&self.path, toml)?;
        log::print("Saved config to '");
        log::info(&self.path);
        log::println("'.");
        Ok(())
    }
    pub fn load_from_file(path: Option<String>) -> Result<Self> {
        let path = match path {
            Some(path) => path,
            None => {
                let home = env::var("HOME").unwrap();
                let xdg_config = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{home}/.config"));
                let config_path = format!("{xdg_config}/gritty/config.toml");
                let fallback = format!("{home}/.gritty.toml");

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
        let contents = fs::read_to_string(&path)?;
        let mut config: Config = toml::from_str(&contents)?;
        config.path = path;
        Ok(config)
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
                clone_protocol: remote.clone_protocol.clone(),
                username: remote.username.clone(),
                url: remote.url.clone(),
                auth: self.get_auth(&self.secrets, name)?,
            });
        }
        Err(ConfigError {
            message: format!("Could not find remote '{name}'"),
            kind: ConfigErrorKind::RemoteNotFound,
        })
    }
    pub fn store_token(&mut self, name: &str, token: &str) -> Result<()> {
        if !self.remotes.contains_key(name) {
            return Err(ConfigError {
                message: format!("Could not find remote '{name}'"),
                kind: ConfigErrorKind::RemoteNotFound,
            });
        }

        match &mut self.secrets {
            #[cfg(feature = "keyring")]
            Secrets::Keyring => {
                let entry = Entry::new(&self.path, name)?;
                entry.set_password(token)?;
                Ok(())
            }
            Secrets::Plaintext(secrets) => {
                let auth = AuthConfig {
                    token: Some(token.to_string()),
                    ..Default::default()
                };
                secrets
                    .entry(name.to_string())
                    .and_modify(|x| *x = auth.clone())
                    .or_insert(auth);
                self.save()?;
                Ok(())
            }
            Secrets::SecretsFile(file) => {
                let file = file.replace('~', env::var("HOME").unwrap().as_str());
                let path = Path::new(&file);
                fs::create_dir_all(path.parent().unwrap())?;

                let contents = fs::read_to_string(&file)?;
                let mut secrets: InlineSecrets = toml::from_str(&contents)?;
                secrets.insert(
                    name.to_string(),
                    AuthConfig {
                        token: Some(token.to_string()),
                        ..Default::default()
                    },
                );

                let toml = toml::to_string(&secrets)?;
                fs::write(&file, toml)?;
                Ok(())
            }
        }
    }

    pub fn get_auth(&self, secrets: &Secrets, name: &str) -> Result<Auth> {
        match secrets {
            Secrets::Plaintext(secrets) => {
                if let Some(auth) = secrets.get(name) {
                    if let Some(token) = &auth.token {
                        return Ok(Auth::Token {
                            token: token.clone(),
                        });
                    }
                    // The user didn't provide a token, so we'll try to use the username and
                    // password.
                    if let Some(username) = &auth.username {
                        return Ok(Auth::Basic {
                            username: username.clone(),
                            password: auth.password.clone().unwrap_or_default(),
                        });
                    }
                    return Err(ConfigError {
                        message: format!(
                            r#"Could not find auth for remote '{name}'.
                             Did you forget to add it to the config?
                             You need to set either a username/password combination,
                             or an api token. "#
                        ),
                        kind: ConfigErrorKind::AuthNotFound,
                    });
                }
            }
            Secrets::SecretsFile(file) => {
                let file = file.replace('~', env::var("HOME").unwrap().as_str());
                if !Path::new(&file).exists() {
                    return Err(ConfigError {
                        message: format!("Could not find secrets file '{file}'."),
                        kind: ConfigErrorKind::SecretsFileNotFound,
                    });
                }
                let contents = fs::read_to_string(file)?;
                let secrets: InlineSecrets = toml::from_str(&contents)?;
                return self.get_auth(&Secrets::Plaintext(secrets), name);
            }
            #[cfg(feature = "keyring")]
            Secrets::Keyring => {
                // we get a unique secret id by combining the config path and the remote name.
                // This is mainly to allow users to use multiple configs without conflicts.
                let entry = Entry::new(&self.path, name)?;
                if let Ok(token) = entry.get_password() {
                    return Ok(Auth::Token { token });
                }
                return Err(ConfigError {
                    message: format!(
                        r#"Could not find auth for remote '{name}'.
                        Did you forget to add it to the keyring?"#
                    ),
                    kind: ConfigErrorKind::AuthNotFound,
                });
            }
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
        let home = env::var("HOME").unwrap();
        let xdg_config = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{home}/.config"));
        Self {
            path: format!("{xdg_config}/gritty/config.toml"),
            remotes: HashMap::from([
                (
                    "gitea".to_string(),
                    GitRemoteConfig {
                        provider: Provider::Gitea,
                        url: "https://gitea.example.com".to_string(),
                        username: "awesome-user".to_string(),
                        clone_protocol: CloneProtocol::HTTPS,
                    },
                ),
                (
                    "github".to_string(),
                    GitRemoteConfig {
                        provider: Provider::GitHub,
                        url: "https://github.com".to_string(),
                        username: "awesome-user".to_string(),
                        clone_protocol: CloneProtocol::SSH,
                    },
                ),
            ]),
            #[cfg(feature = "keyring")]
            secrets: Secrets::Keyring,
            #[cfg(not(feature = "keyring"))]
            secrets: Secrets::SecretsFile(format!("{xdg_config}/gritty/secrets.toml")),
        }
    }
}
