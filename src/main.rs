use anyhow::{anyhow, bail, Context, Result};
use bitcoin::{Amount, Network, PrivateKey};
use bitcoincore_rpc::Client;

mod config;
mod db;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next().ok_or_else(|| anyhow!("program name missing"))?;
    match args.next() {
        None => {
            println!("Command missing\n\n");
            help()
        }
        Some(command) => {
            match &*command {
                "scan" => scan(),
                "address" => address(),
                "balance" => balance(),
                "send" => send(args),
                "help" | "--help" |"-h" => help(),
                _ => bail!("Unknown command: `{}`", command),
            }
        }
    }
}

/// Prints an address associated with the private key loaded from file.
///
/// In a production wallet one would never reuse a single address like this but for demonstration
/// purposes it will suffice.
///
/// You can use a taproot address if you would like to play with taproot spends or alternatively you
/// can use a native segwit address. Note that the PSBT signing APIs are slightly different for each.
fn address() -> Result<()> {
    todo!("Implement this")
}

/// Scans the Bitcoin blockchain.
///
/// Requests blocks from `bitcoind`, starting at the current block height (`db.get_last_height`) and
/// stores relevant transaction information in the database.
///
/// Call this each time you use `bitcoin-cli generatetoaddress` to mine coins to your address.
///
///
fn scan() -> Result<()> {
    todo!("Implement scan once you have address working")
}

fn send(_args: std::env::Args) -> Result<()> {
    todo!("Implement send once you have scan working")
}

/// Prints the balance out of database, you must call `scan` first to populate the database.
fn balance() -> Result<()> {
    let mut db = db::Db::open()?;
    let mut total = Amount::ZERO;

    for result in db.iter_unspent()?.iter()? {
        let (_prev_out, amt) = result?;
        total += amt;
    }

    println!("Balance: {}", total);
    Ok(())
}

/// Prints help menu.
fn help() -> Result<()> {
    println!("");
    println!("Usage: pico-bitcoin-wallet COMMAND");
    println!("");
    println!("Commands:");
    println!("");
    println!(" address\t: Get the wallet address.");
    println!(" balance\t: Get the current balance.");
    println!(" scan\t\t: Scan all blocks looking for relevant transactions.");
    println!(" send\t\t: Send a given amount to the address provided.");
    println!(" help\t\t: Print this help menu.");
    println!("");

    let data_dir = db::data_dir()?;
    let config_file = config::config_file()?;

    println!("Some paths you might need:");
    println!("");
    println!("data directory: {}", data_dir.display());
    println!("configuration file: {}", config_file.display());
    println!("");

    Ok(())
}

///
/// Helper functions.
///

/// Loads a private key from file.
///
/// Creates a new private key if file is not found.
fn load_private_key() -> Result<PrivateKey> {
    let sk_path = db::private_key_file()?;

    match std::fs::read_to_string(&sk_path) {
        Ok(key) => key.parse().context("failed to parse private key"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let key = PrivateKey::new(secp256k1::SecretKey::new(&mut rand::thread_rng()), Network::Regtest);
            std::fs::write(&sk_path, key.to_wif().as_bytes()).context("failed to save private key")?;
            Ok(key)
        },
        Err(error) => Err(anyhow!(error).context("failed to read private key")),
    }
}

/// Gets an RPC client for `bitcoind`.
fn bitcoind_rpc_client() -> Result<Client> {
    let conf = config::load()?;
    let client = bitcoincore_rpc::Client::new(&conf.bitcoind_uri, conf.bitcoind_auth)
        .context("failed to connect to bitcoind")?;

    Ok(client)
}
