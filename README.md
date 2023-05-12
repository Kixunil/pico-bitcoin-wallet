# Insanely minimal Bitcoin wallet intended for demonstration of [Rust Bitcoin] ecosystem

**Absolutely DO NOT use with mainnet funds!!! No privacy - address reuse and other problems; no security review!**

TL;DR - run `cargo help` and read `main.rs`.

This code is intended for demonstration purposes only.
The aim is to give a basic introduction to [`rust-bitcoin`](https://docs.rs/bitcoin) and associated crates.
This is not a comprehensive developer guide.
We explicitly do not teach about security or other important practices - which would require a whole book, maybe even multiple books!

The code has regtest hard-coded, hard-coding the network should not be done in production code.

## Tutorial

The `master` branch contains only the skeleton which you can use to implement the wallet yourself.
Implementing your own wallet may give you some insight into Bitcoin and its Rust library.
The skeleton provides configuration and database support plus some minimal helper functions.


### Overview

0. Install/configure bitcoind regtest node (see below).
1. Configure `pico-bitcoin-wallet` (see below).
2. Read the code in `lib.rs`
3. Implement the `address` command.
4. Implement the `scan` command.
5. Implement the `send` command.


Solutions are provide per-step as git branches.

Happy hacking!

### Configure bitcoind

[Cryptoanarchy Debian Repository](https://github.com/debian-cryptoanarchy/cryptoanarchy-deb-repo-builder) may help.

Otherwise configure the wallet in `~/.config/pico-bitcoin-wallet/config.toml`.

Example minimal `bitcoin.conf`
```
chain=regtest
# Password for JSON-RPC connections
rpcpassword=pass
# Username for JSON-RPC connections
rpcuser=admin
```

### Configure pico-bitcoin-wallet

If you used `bitcoin-timechain-regtest` from `cryptoanarchy` you don't need any further configuration.
Otherwise create `~/.config/pico-bitcoin-wallet/config.toml` (run `pico-bitocin-wallet help` to get correct path for your system).

```toml
bitcoind_uri = "http://localhost:18443"
bitcoind_username = "admin"
bitcoind_password = "pass"
```

### bitcoin-cli and pico-bitcoin-wallet

Using `bitcoin-cli` you'll need to create a wallet and mine a bunch of blocks (more than 100).
Send bitcoin to the address output by your `address` command.
Scan the chain with your `scan` command and check the balance with the `balance` command.
Send bitcoin back to the wallet you loaded in `bitcoind`.


### Tips and possible gotchas

Things that were discovered during implementation of a solution that seem important to mention:

* Note that the last block has number equal to block count, so the traditional `last_height..current_height` misses the last one.
* Private key pair for Taproot needs to be tweaked before signing - call `.tap_tweak()` on it and then convert it using `to_inner`.
* To obtain a block from bitcoind you need to first get its hash using `get_block_hash`.
* `store_txos` takes an iterator, don't bother being clever if you are low on time - just collect to `Vec` and pass it in.
* Don't bother with coin selection, just think of a dumb algorithm (e.g. just spend all inputs or just spend the first one you find that is big enough).

## Solutions

There are the following solution branches:

- `solution-address`: Implements address.
- `solution-scan`: Implements scan.
- `solution-complete-taproot`: Implements a complete taproot solution.
- `solution-complete-segwit-v0`: Implements a complete p2wpkh solution.

[Rust Bitcoin]: https://rust-bitcoin.org
