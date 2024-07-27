use crate::error::{Error, Result};
use std::{collections::HashMap, env, fs, path::Path};

#[cfg(feature = "keyring")]
use keyring::Entry;
use serde::{Deserialize, Serialize};

use crate::remote::{Auth, CloneProtocol, Provider, RemoteConfig};

use self::colors::ConfigColorMap;

pub mod colors;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// A list of remotes.
    /// Maps the remote name to the remote configuration.
    pub remotes: HashMap<String, GitRemoteConfig>,
    pub secrets: Secrets,
    pub colors: Option<ConfigColorMap>,
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

pub type InlineSecrets = HashMap<String, AuthConfig>;
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
    pub fn save_default(path: &Option<String>) -> Result<()> {
        let mut config = Config::default();
        if let Some(path) = path {
            config.path.clone_from(path);
        }
        config.save()
    }
    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string(self)?;
        fs::create_dir_all(Path::new(&self.path).parent().unwrap())?;
        fs::write(&self.path, toml)?;
        println!("Saved config to {}.", &self.path);
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
                        return Err(Error::not_found(format!(
                            "Could not find config at {config_path} or {fallback}."
                        )));
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
        Err(Error::not_found(format!("Could not find remote {name}")))
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
        Err(Error::not_found(format!("Could not find remote {name}")))
    }
    pub fn store_token(&mut self, name: &str, token: &str) -> Result<()> {
        if !self.remotes.contains_key(name) {
            return Err(Error::not_found(format!("Could not find remote {name}")));
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
                if !path.exists() {
                    let toml = toml::to_string(&InlineSecrets::new())?;
                    fs::write(&file, toml)?;
                }

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
                    return Err(Error::authentication(format!(
                        "Could not find auth for remote {name}.\n{}\n{}",
                        "Did you forget to add it to the config?",
                        "You need to set either a username/password combination, or an api token."
                    )));
                }
            }
            Secrets::SecretsFile(file) => {
                let file = file.replace('~', env::var("HOME").unwrap().as_str());
                if !Path::new(&file).exists() {
                    return Err(Error::not_found(format!(
                        "Could not find secrets file {file}."
                    )));
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
                return Err(Error::authentication(format!(
                    "Could not find auth for remote {name}.\n{}",
                    "Did you forget to add it to the keyring?"
                )));
            }
        }
        Err(Error::authentication(format!(
            "Could not find auth for remote {name}.\n{}",
            "Did you forget to add it to the config?"
        )))
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = env::var("HOME").unwrap();
        let xdg_config = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{home}/.config"));
        Self {
            path: format!("{xdg_config}/gritty/config.toml"),
            remotes: HashMap::new(),
            colors: None,
            #[cfg(feature = "keyring")]
            secrets: Secrets::Keyring,
            #[cfg(not(feature = "keyring"))]
            secrets: Secrets::SecretsFile(format!("{xdg_config}/gritty/secrets.toml")),
        }
    }
}
