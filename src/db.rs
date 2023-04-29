use rusqlite::{Connection, ToSql};
use anyhow::{Result, anyhow, Context};
use core::convert::TryInto;

const CREATE_TABLES: &str = r#"
BEGIN;
CREATE TABLE IF NOT EXISTS txos (txid BLOB, idx INTEGER, amount_sat INTEGER, spent_status INTEGER, PRIMARY KEY(txid, idx));
CREATE TABLE IF NOT EXISTS last_block (block_height INTEGER);
INSERT INTO last_block (block_height) SELECT 0 WHERE NOT EXISTS (SELECT * FROM last_block);
COMMIT;
"#;

pub struct Db(Connection);

impl Db {
    pub fn open() -> Result<Self> {
        let data_dir = dirs::data_dir().ok_or(anyhow!("The user data directory was not identified"))?.join("pico-bitcoin-wallet");
        std::fs::create_dir_all(&data_dir).with_context(|| format!("Failed to create data dir at {}", data_dir.display()))?;
        let path = data_dir.join("data.db");
        let connection = Connection::open(&path).with_context(|| format!("failed to open database at {}", path.display()))?;
        connection.execute_batch(CREATE_TABLES).context("failed to prepare the database tables")?;
        Ok(Db(connection))
    }

    pub fn get_last_height(&mut self) -> Result<u64> {
        let (height,) = self.0.query_row("SELECT block_height FROM  last_block", [], |row| row.try_into()).context("failed to query last block height")?;
        Ok(height)
    }

    pub fn store_txos<'a>(&mut self, txos: impl Iterator<Item=Result<(impl std::borrow::Borrow<bitcoin::OutPoint>, u64)>>, last_height: u64) -> Result<()> {
        use bitcoin::hashes::Hash;

        let transaction = self.0.transaction().context("failed to begin database transaction")?;
        for txo in txos {
            let (prev_input, amount) = txo?;
            let prev_input = prev_input.borrow();
            let params = [&(prev_input.txid.as_byte_array() as &[_]) as &dyn ToSql, &prev_input.vout, &amount];
            transaction.execute("INSERT INTO txos VALUES (?, ?, ?, 0)", &params)
                .with_context(|| format!("failed to insert txout {}:{} into the database", prev_input.txid, prev_input.vout))?;
        }
        let params = [&last_height as &dyn ToSql];
        transaction.execute("UPDATE last_block SET block_height = ?", &params)
            .context("failed to update last block in the database")?;
        transaction.commit().context("failed to commit database transaction")
    }

    pub fn iter_unspent(&mut self) -> Result<Utxos<'_>> {
        let prepared = self.0.prepare("SELECT txid, idx, amount_sat FROM txos WHERE spent_status = 0")
            .context("failed to prepare query statement")?;
        Ok(Utxos(prepared))
    }

    pub fn set_spent(&mut self, txo: &bitcoin::OutPoint) -> Result<usize> {
        use bitcoin::hashes::Hash;

        let params = [&(txo.txid.as_byte_array() as &[_]) as &dyn ToSql, &txo.vout];
        self.0.execute("UPDATE txos SET spent_status = 1 WHERE txid = ? AND idx = ?", &params)
            .with_context(|| format!("failed to mark txo {}:{} as spent", txo.txid, txo.vout))
    }
}

pub struct Utxos<'a>(rusqlite::Statement<'a>);

impl<'a> Utxos<'a> {
    pub fn iter(&mut self) -> Result<impl Iterator<Item=Result<(bitcoin::OutPoint, bitcoin::Amount)>> + '_> {
        use bitcoin::hashes::Hash;

        let iter = self.0
            .query_map([], move |row| {
                let (txid, vout, amount): (Vec<u8>, u32, u64) = row.try_into()?;
                let txid = bitcoin::Txid::from_byte_array(txid.try_into().unwrap());
                let txo = bitcoin::OutPoint {
                    txid,
                    vout,
                };
                Ok((txo, bitcoin::Amount::from_sat(amount)))
            })
            .context("failed to select unspent txos")?
            .map(|result| result.context("failed to convert SQL value to Rust type"));
        Ok(iter)
    }
}
