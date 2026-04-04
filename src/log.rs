use rusqlite::{params, Connection, Error as SqlError, Result};
use sha2::{Digest, Sha256};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const BUSY_RETRY_ATTEMPTS: usize = 10;
const BUSY_RETRY_DELAY_MS: u64 = 10;

#[derive(Debug, Clone, Copy)]
pub struct IntegrityReport {
    pub total_events: i64,
    pub missing_or_invalid_hashes: i64,
    pub hash_mismatches: i64,
    pub orphan_chunks: i64,
}

#[derive(Debug, Clone)]
pub struct AppendReceipt {
    pub id: String,
    pub timestamp: i64,
    pub ingested_at: i64,
    pub content_hash: String,
}

fn unix_now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Append a single event to the log
pub fn append(
    conn: &Connection,
    source: &str,
    content: &str,
    meta: Option<&str>,
) -> Result<String> {
    Ok(append_with_receipt(conn, source, content, meta)?.id)
}

/// Append a single event and return the canonical persistence receipt.
pub fn append_with_receipt(
    conn: &Connection,
    source: &str,
    content: &str,
    meta: Option<&str>,
) -> Result<AppendReceipt> {
    with_busy_retry(|| {
        let tx = conn.unchecked_transaction()?;
        let receipt = append_with_receipt_in_tx(&tx, source, content, meta)?;
        tx.commit()?;
        Ok(receipt)
    })
}

pub(crate) fn append_with_receipt_in_tx(
    conn: &Connection,
    source: &str,
    content: &str,
    meta: Option<&str>,
) -> Result<AppendReceipt> {
    let id = Uuid::new_v4().to_string();
    let now = unix_now_secs();
    let last_ts: Option<i64> =
        conn.query_row("SELECT MAX(timestamp) FROM events", [], |row| row.get(0))?;
    let timestamp = match last_ts {
        Some(last) if now <= last => last + 1,
        _ => now,
    };
    let ingested_at = timestamp;

    // Calculate content hash for deduplication
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let content_hash = format!("{:x}", hasher.finalize());

    conn.execute(
        "INSERT INTO events (id, timestamp, source, content, meta, ingested_at, content_hash)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            timestamp,
            source,
            content,
            meta,
            ingested_at,
            content_hash
        ],
    )?;

    Ok(AppendReceipt {
        id,
        timestamp,
        ingested_at,
        content_hash,
    })
}

/// Append multiple events from a batch in a single transaction
pub fn append_batch(
    conn: &Connection,
    source: &str,
    contents: &[&str],
    meta: Option<&str>,
) -> Result<Vec<String>> {
    Ok(append_batch_with_receipts(conn, source, contents, meta)?
        .into_iter()
        .map(|receipt| receipt.id)
        .collect())
}

/// Append multiple events and return a receipt for each inserted event.
pub fn append_batch_with_receipts(
    conn: &Connection,
    source: &str,
    contents: &[&str],
    meta: Option<&str>,
) -> Result<Vec<AppendReceipt>> {
    with_busy_retry(|| {
        let tx = conn.unchecked_transaction()?;
        let receipts = append_batch_with_receipts_in_tx(&tx, source, contents, meta)?;
        tx.commit()?;
        Ok(receipts)
    })
}

pub(crate) fn append_batch_with_receipts_in_tx(
    conn: &Connection,
    source: &str,
    contents: &[&str],
    meta: Option<&str>,
) -> Result<Vec<AppendReceipt>> {
    let mut receipts = Vec::with_capacity(contents.len());

    let now = unix_now_secs();
    let last_ts: Option<i64> =
        conn.query_row("SELECT MAX(timestamp) FROM events", [], |row| row.get(0))?;
    let mut timestamp = match last_ts {
        Some(last) if now <= last => last + 1,
        _ => now,
    };

    for content in contents {
        let receipt = append_with_receipt_at_timestamp(conn, source, content, meta, timestamp)?;
        receipts.push(receipt);
        timestamp += 1;
    }

    Ok(receipts)
}

fn append_with_receipt_at_timestamp(
    conn: &Connection,
    source: &str,
    content: &str,
    meta: Option<&str>,
    timestamp: i64,
) -> Result<AppendReceipt> {
    let id = Uuid::new_v4().to_string();
    let ingested_at = timestamp;

    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let content_hash = format!("{:x}", hasher.finalize());

    conn.execute(
        "INSERT INTO events (id, timestamp, source, content, meta, ingested_at, content_hash)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            timestamp,
            source,
            content,
            meta,
            ingested_at,
            content_hash
        ],
    )?;

    Ok(AppendReceipt {
        id,
        timestamp,
        ingested_at,
        content_hash,
    })
}

fn with_busy_retry<T, F>(mut operation: F) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    let mut attempts = 0;

    loop {
        match operation() {
            Ok(value) => return Ok(value),
            Err(err) if is_busy_error(&err) && attempts < BUSY_RETRY_ATTEMPTS => {
                attempts += 1;
                thread::sleep(std::time::Duration::from_millis(BUSY_RETRY_DELAY_MS));
            }
            Err(err) => return Err(err),
        }
    }
}

fn is_busy_error(err: &SqlError) -> bool {
    matches!(
        err,
        SqlError::SqliteFailure(code, _)
            if matches!(
                code.code,
                rusqlite::ffi::ErrorCode::DatabaseBusy | rusqlite::ffi::ErrorCode::DatabaseLocked
            )
    )
}

/// Append events from stdin with configurable batch size
pub fn append_stdin(
    conn: &Connection,
    source: &str,
    meta: Option<&str>,
    batch_size: usize,
) -> std::io::Result<Vec<String>> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut all_ids = Vec::new();
    let mut batch: Vec<String> = Vec::new();
    let effective_batch_size = batch_size.max(1);

    for line in stdin.lock().lines() {
        let line = line?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            batch.push(trimmed.to_string());
        }

        // Execute batch when we reach the configured size
        if batch.len() >= effective_batch_size {
            let contents: Vec<&str> = batch.iter().map(|s| s.as_str()).collect();
            match append_batch(conn, source, &contents, meta) {
                Ok(ids) => all_ids.extend(ids),
                Err(e) => return Err(io::Error::other(e)),
            }
            batch.clear();
        }
    }

    // Don't forget the last batch if it's non-empty
    if !batch.is_empty() {
        let contents: Vec<&str> = batch.iter().map(|s| s.as_str()).collect();
        match append_batch(conn, source, &contents, meta) {
            Ok(ids) => all_ids.extend(ids),
            Err(e) => {
                return Err(io::Error::other(e));
            }
        }
    }

    Ok(all_ids)
}

/// Check if an event with the same content hash already exists
pub fn is_duplicate(conn: &Connection, content_hash: &str) -> Result<bool> {
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM events WHERE content_hash = ?1)",
        [content_hash],
        |row| row.get(0),
    )?;

    Ok(exists)
}

/// Get statistics about the events
pub fn stats(conn: &Connection) -> Result<(i64, i64, i64, i64)> {
    let total: i64 = conn.query_row(
        "SELECT COUNT(*)
         FROM events
         WHERE NOT EXISTS (
             SELECT 1 FROM shadow_state s WHERE s.event_id = events.id
         )",
        [],
        |row| row.get(0),
    )?;
    let unique: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT content_hash)
             FROM events
             WHERE NOT EXISTS (
                 SELECT 1 FROM shadow_state s WHERE s.event_id = events.id
             )",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    let oldest: i64 = conn
        .query_row(
            "SELECT MIN(timestamp)
             FROM events
             WHERE NOT EXISTS (
                 SELECT 1 FROM shadow_state s WHERE s.event_id = events.id
             )",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    let newest: i64 = conn
        .query_row(
            "SELECT MAX(timestamp)
             FROM events
             WHERE NOT EXISTS (
                 SELECT 1 FROM shadow_state s WHERE s.event_id = events.id
             )",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok((total, unique, oldest, newest))
}

/// Verify core integrity invariants for stored events and chunks.
pub fn verify_integrity(conn: &Connection) -> Result<IntegrityReport> {
    let total_events: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;

    let missing_or_invalid_hashes: i64 = conn.query_row(
        "SELECT COUNT(*) FROM events WHERE content_hash IS NULL OR length(content_hash) != 64",
        [],
        |row| row.get(0),
    )?;

    let mut hash_mismatches = 0_i64;
    let mut stmt = conn.prepare("SELECT content, content_hash FROM events")?;
    let rows = stmt.query_map([], |row| {
        let content: String = row.get(0)?;
        let content_hash: Option<String> = row.get(1)?;
        Ok((content, content_hash))
    })?;

    for row in rows {
        let (content, stored_hash) = row?;
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let computed = format!("{:x}", hasher.finalize());

        if stored_hash.as_deref() != Some(computed.as_str()) {
            hash_mismatches += 1;
        }
    }

    let orphan_chunks: i64 = conn.query_row(
        "SELECT COUNT(*) FROM chunks c LEFT JOIN events e ON e.id = c.event_id WHERE e.id IS NULL",
        [],
        |row| row.get(0),
    )?;

    Ok(IntegrityReport {
        total_events,
        missing_or_invalid_hashes,
        hash_mismatches,
        orphan_chunks,
    })
}
