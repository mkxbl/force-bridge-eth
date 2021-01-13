use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::Done;

#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EthToCkbRecord {
    pub id: i64,
    pub eth_lock_tx_hash: String,
    pub status: String,
    pub token_addr: Option<String>,
    pub sender_addr: Option<String>,
    pub locked_amount: Option<String>,
    pub bridge_fee: Option<String>,
    pub ckb_recipient_lockscript: Option<String>,
    pub sudt_extra_data: Option<String>,
    pub ckb_tx_hash: Option<String>,
    pub err_msg: Option<String>,
}

pub async fn create_eth_to_ckb_status_record(pool: &MySqlPool, tx_hash: String) -> Result<u64> {
    let id = sqlx::query(
        r#"
INSERT INTO eth_to_ckb ( eth_lock_tx_hash )
VALUES ( ? )
        "#,
    )
    .bind(tx_hash)
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(id)
}

pub async fn update_eth_to_ckb_status(pool: &MySqlPool, record: &EthToCkbRecord) -> Result<bool> {
    log::info!("update_eth_to_ckb_status, record: {:?}", record);
    let rows_affected = sqlx::query(
        r#"
UPDATE eth_to_ckb SET
    status = ?,
    token_addr = ?,
    sender_addr = ?,
    locked_amount = ?,
    bridge_fee = ?,
    ckb_recipient_lockscript = ?,
    sudt_extra_data = ?,
    ckb_tx_hash = ?,
    err_msg = ?
WHERE id = ?
        "#,
    )
    .bind(record.id)
    .bind(record.status.clone())
    .bind(record.token_addr.as_ref())
    .bind(record.sender_addr.as_ref())
    .bind(record.locked_amount.as_ref())
    .bind(record.bridge_fee.as_ref())
    .bind(record.ckb_recipient_lockscript.as_ref())
    .bind(record.sudt_extra_data.as_ref())
    .bind(record.ckb_tx_hash.as_ref())
    .bind(record.err_msg.as_ref())
    .execute(pool)
    .await?
    .rows_affected();
    Ok(rows_affected > 0)
}

pub async fn get_eth_to_ckb_status(
    pool: &MySqlPool,
    eth_lock_tx_hash: &str,
) -> Result<Option<EthToCkbRecord>> {
    Ok(sqlx::query_as::<_, EthToCkbRecord>(
        r#"
SELECT *
FROM eth_to_ckb
where eth_lock_tx_hash = ?
        "#,
    )
    .bind(eth_lock_tx_hash)
    .fetch_optional(pool)
    .await?)
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CrosschainHistory {
    pub id: u64,
    pub eth_tx_hash: Option<String>,
    pub ckb_tx_hash: Option<String>,
    pub status: String,
    pub sort: String,
    pub amount: String,
    pub token_addr: String,
}

pub async fn get_ckb_to_eth_crosschain_history(
    pool: &MySqlPool,
    eth_recipient_address: &str,
) -> Result<Vec<CrosschainHistory>> {
    Ok(sqlx::query_as::<_, CrosschainHistory>(
        r#"
SELECT id, eth_tx_hash, ckb_burn_tx_hash as ckb_tx_hash, status, 'ckb_to_eth' as sort, token_amount as amount, token_addr
FROM ckb_to_eth
where recipient_addr = ?
        "#,
    )
        .bind(eth_recipient_address)
        .fetch_all(pool)
        .await?)
}

pub async fn get_eth_to_ckb_crosschain_history(
    pool: &MySqlPool,
    ckb_recipient_lockscript: &str,
) -> Result<Vec<CrosschainHistory>> {
    Ok(sqlx::query_as::<_, CrosschainHistory>(
        r#"
SELECT id, eth_lock_tx_hash as eth_tx_hash, ckb_tx_hash, status, 'eth_to_ckb' as sort, locked_amount as amount, token_addr
FROM eth_to_ckb
where ckb_recipient_lockscript = ?
        "#,
    )
    .bind(ckb_recipient_lockscript)
    .fetch_all(pool)
    .await?)
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CkbToEthRecord {
    pub id: i64,
    pub ckb_burn_tx_hash: String,
    pub status: String,
    pub recipient_addr: Option<String>,
    pub token_addr: Option<String>,
    pub token_amount: Option<String>,
    pub fee: Option<String>,
    pub eth_tx_hash: Option<String>,
    pub err_msg: Option<String>,
}

pub async fn create_ckb_to_eth_status_record(pool: &MySqlPool, tx_hash: String) -> Result<i64> {
    let id = sqlx::query(
        r#"
INSERT INTO ckb_to_eth ( ckb_burn_tx_hash )
VALUES ( ? )
        "#,
    )
    .bind(tx_hash)
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(id as i64)
}

pub async fn update_ckb_to_eth_status(pool: &MySqlPool, record: &CkbToEthRecord) -> Result<bool> {
    log::info!("update_ckb_to_eth_status, record: {:?}", record);
    let rows_affected = sqlx::query(
        r#"
UPDATE ckb_to_eth SET
    status = ?,
    recipient_addr = ?,
    token_addr = ?,
    token_amount = ?,
    fee = ?,
    eth_tx_hash = ?,
    err_msg = ?
WHERE id = ?
        "#,
    )
    .bind(record.status.clone())
    .bind(record.recipient_addr.as_ref())
    .bind(record.token_addr.as_ref())
    .bind(record.token_amount.as_ref())
    .bind(record.fee.as_ref())
    .bind(record.eth_tx_hash.as_ref())
    .bind(record.err_msg.as_ref())
    .bind(record.id)
    .execute(pool)
    .await?
    .rows_affected();
    Ok(rows_affected > 0)
}

pub async fn get_ckb_to_eth_status(
    pool: &MySqlPool,
    ckb_burn_tx_hash: &str,
) -> Result<Option<CkbToEthRecord>> {
    Ok(sqlx::query_as::<_, CkbToEthRecord>(
        r#"
SELECT *
FROM ckb_to_eth
where ckb_burn_tx_hash = ?
        "#,
    )
    .bind(ckb_burn_tx_hash)
    .fetch_optional(pool)
    .await?)
}
