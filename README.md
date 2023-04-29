# Insanely minimal Bitcoin wallet intended for demonstration of [Rust Bitcoin] ecosystem

**Absolutely DO NOT use with mainnet funds!!! No privacy - address reuse and other problems; no security review!**

This code is intended for demonstration only.
If you read it it should give you a better picture of [`rust-bitcoin`](https://docs.rs/bitcoin) and
associated crates.
Note that this is just basic introduction, not a comprehensive developer guide.
It doesn't teach about security or other important practices - it'd require whole book, maybe even multiple books!

The code has regtest hard-coded. This would be normally a bad practice if this wasn't a demo.

## Tutorial

The `master` branch contains only the skeleton which you can use to implement the wallet yourself.
Implementing your own wallet will give you much better insight into Bitcoin and its Rust library.
The skeleton provides configuration and database support.

### Usage

Install and run bitcoind in regtest mode.
[Cryptoanarchy Debian Repository](https://github.com/debian-cryptoanarchy/cryptoanarchy-deb-repo-builder) may help.
If you install `bitcoin-timechain-regtest` you don't need any further configuration.
Otherwise configure the wallet in `~/.config/pico-bitcoin-wallet/config.toml`.

Create a wallet in bitcoind and mine some coins (mine at least 101 blocks so that coins are mature).

You should implement these commands:

* `address` - returns the address of the wallet - create it from private key.
* `scan` - scans the blocks to find matching transactions, should also tell the user how many coins were found.
* `send` - creates a transaction that sends a given amount to the address provided in the input, bonus: implement BIP21

First version can be really very lazy if you like. E.g. require scan after sending transaction.

### Tips and possible gotchas

Things that were discovered during implementation of solution that seem important to mention:

* Note that the last block has number equal to block count, so the traditional `last_height..current_height` misses the last one.
* Private key pair for Taproot needs to be tweked before signing - call `.tap_tweak()` on it and then convert it using `to_inner`.
* To obtain a block from bitcoind you need to first get its hash using `get_block_hash`.
* `store_txos` takes an iterator, don't bother with being clever if you are low on time - just collect to `Vec` and pass it in.
* Don't bother with coin selection - just always spend all inputs, that'd be the best anyway if you actually reuse addresses.

## Solution

There's a `solution` branch that contains fully working minimal example.
You can use it if you decide to not write it yourself.

[Rust Bitcoin]: https://rust-bitcoin.org
