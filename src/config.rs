extern crate toml;

use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};
use toml::de::Error;

fn get_env<T: FromStr>(key: &str, default: T) -> T {
    match env::var(key) {
        Ok(val) => val.parse::<T>().unwrap_or(default),
        Err(_) => default,
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct SSH {
    /// Server to ssh to
    pub host: String,
    /// default user to use when ssh-ing
    pub user: String,
    /// shh authentication, method. Defaults to "password", you can use "publickey,password" instead'
    pub auth: String,
    /// Password to use when sshing
    pub pass: Option<String>,
    /// path to an optional client private key, connection will be password-less and insecure!
    pub key: Option<String>,
    /// Port to ssh to
    pub port: i16,
    /// ssh knownHosts file to use
    pub known_hosts: String,
    /// alternative ssh configuration file, see "-F" option in ssh(1)
    pub config: Option<String>,
}
impl Default for SSH {
    fn default() -> Self {
        SSH {
            user: get_env("SSHUSER", "".to_string()),
            host: get_env("SSHHOST", "localhost".to_string()),
            auth: get_env("SSHAUTH", "password".to_string()),
            port: get_env("SSHPORT", 22),
            known_hosts: get_env("KNOWNHOSTS", "/dev/null".to_string()),
            config: None,
            key: None,
            pass: None,
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Server {
    /// URL base to serve resources from
    pub base: String,
    /// address to listen on including the port eg: 0.0.0.0:3000
    pub address: SocketAddr,
    // Page title
    pub title: String,
    /// allow wetty to be embedded in an iframe
    pub allow_iframe: bool,
}
impl Default for Server {
    fn default() -> Self {
        Server {
            base: get_env("BASE", "wetty".to_string()),
            address: get_env(
                "ADDRESS",
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030),
            ),
            title: get_env("TITLE", "WeTTy - The Web Terminal Emulator".to_string()),
            allow_iframe: false,
        }
    }
}

fn default_command() -> String {
    get_env("COMMAND", "login".to_string())
}

fn force_ssh() -> bool {
    get_env("FORCESSH", false)
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Config {
    #[serde(default)]
    pub server: Server,
    #[serde(default)]
    pub ssh: SSH,
    #[serde(default = "force_ssh")]
    /// Force sshing to local machine over login if running as root
    pub force_ssh: bool,
    #[serde(default = "default_command")]
    /// Command to run on server. Login will use ssh if connecting to different server
    pub default_command: String,
}

impl Config {
    pub fn from_file(filename: &PathBuf) -> Result<Self, Error> {
        debug!("loading config; path={:?}", filename);
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        toml::from_str(&*contents)
    }

    pub fn print_default() {
        let conf: Config = toml::from_str("").expect("failed to set default");
        println!(
            "{}",
            toml::Value::try_from(conf).expect("failed to Serialize conf to toml")
        )
    }
}
