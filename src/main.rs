use anyhow::{Result, Context, anyhow, bail};
use bitcoin::{PrivateKey, Network};

mod config;
mod db;

fn load_private_key() -> Result<PrivateKey> {
    let data_dir = dirs::data_dir().ok_or(anyhow!("The user data directory was not identified"))?.join("pico-bitcoin-wallet");
    std::fs::create_dir_all(&data_dir).with_context(|| format!("Failed to create data dir at {}", data_dir.display()))?;
    let path = data_dir.join("private.key");
    match std::fs::read_to_string(&path) {
        Ok(key) => key.parse().context("Failed to parse private key"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let key = PrivateKey::new(secp256k1::SecretKey::new(&mut rand::thread_rng()), Network::Regtest);
            std::fs::write(&path, key.to_wif().as_bytes()).context("failed to save private key")?;
            Ok(key)
        },
        Err(error) => Err(anyhow!(error).context("failed to read private key")),
    }
}

fn scan() -> Result<()> {
    let db = db::Db::open();
    todo!()
}

fn address() -> Result<()> {
    todo!()
}

fn send(args: std::env::Args) -> Result<()> {
    let db = db::Db::open();
    todo!()
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next().ok_or_else(|| anyhow!("not even program name given"))?;
    let command = args.next().ok_or_else(|| anyhow!("missing command"))?;
    match &*command {
        "scan" => scan(),
        "address" => address(),
        "send" => send(args),
        _ => bail!("Unknown command: `{}`", command),
    }
}
