use anyhow::{Result, anyhow, Context, bail};

pub fn load() -> Result<Config> {
    let conf_dir = dirs::config_dir().ok_or(anyhow!("The user config directory was not identified"))?;
    let conf_file = conf_dir.join("pico-bitcoin-wallet/config.toml");
    match std::fs::read_to_string(&conf_file) {
        Ok(toml_string) => {
            let config = toml::from_str::<ConfigFile>(&toml_string).with_context(|| format!("Failed to parse file {}", conf_file.display()))?;
            let auth = match (config.bitcoind_cookie_path, config.bitcoind_username, config.bitcoind_password) {
                (None, None, None) => bitcoincore_rpc::Auth::None,
                (Some(path), None, None) => bitcoincore_rpc::Auth::CookieFile(path),
                (None, Some(username), Some(password)) => bitcoincore_rpc::Auth::UserPass(username, password),
                _ => bail!("Invalid configuration: either cookie path or both username and password must be specified"),
            };
            Ok(Config {
                bitcoind_uri: config.bitcoind_uri,
                bitcoind_auth: auth,
            })
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Config::default(),
        // TODO: write more sensible code here
        Err(error) => Err(error).with_context(|| format!("Failed to read file {}", conf_file.display()))?,
    }
}

pub struct Config {
    pub bitcoind_uri: String,
    pub bitcoind_auth: bitcoincore_rpc::Auth,
}

impl Config {
    fn default() -> Result<Self> {
        let home_dir = dirs::home_dir().ok_or(anyhow!("The user home directory was not identified"))?;
        let bitcoind_dir = home_dir.join(".bitcoin");
        match bitcoind_dir.metadata() {
            Ok(_) => {
                Ok(Config {
                    // Ideally port shouldn't be fixed but I'm too lazy for that
                    bitcoind_uri: "http://127.0.0.1:18443".to_owned(),
                    bitcoind_auth: bitcoincore_rpc::Auth::CookieFile(bitcoind_dir.join(".cookie")),
                })
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if std::fs::metadata("/etc/bitcoin-rpc-proxy-regtest").is_ok() {
                    Ok(Config {
                        // Ideally port shouldn't be fixed but I'm too lazy for that
                        bitcoind_uri: "http://127.0.0.1:18443".to_owned(),
                        bitcoind_auth: bitcoincore_rpc::Auth::UserPass("public".to_owned(), "public".to_owned()),
                    })
                } else {
                    bail!("Failed to identify bitcoind configuration");
                }
            },
            Err(error) => Err(error).with_context(|| format!("Failed to check if bitcoind dir {} exists", bitcoind_dir.display()))?,
        }
    }
}

#[derive(serde::Deserialize)]
struct ConfigFile {
    bitcoind_uri: String,
    #[serde(default)]
    bitcoind_cookie_path: Option<std::path::PathBuf>,
    #[serde(default)]
    bitcoind_username: Option<String>,
    #[serde(default)]
    bitcoind_password: Option<String>,
}
