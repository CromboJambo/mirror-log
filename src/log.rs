use rusqlite::{Connection, Result, params};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Append a single event to the log
pub fn append(
    conn: &Connection,
    source: &str,
    content: &str,
    meta: Option<&str>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let ingested_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

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

    Ok(id)
}

/// Append multiple events from a batch in a single transaction
pub fn append_batch(
    conn: &Connection,
    source: &str,
    contents: &[&str],
    meta: Option<&str>,
) -> Result<Vec<String>> {
    let mut ids = Vec::new();
    let mut hasher = Sha256::new();

    // Wrap all inserts in a single transaction for atomicity
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(Box::new(e)))?;

    for content in contents {
        let id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let ingested_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Calculate content hash
        hasher.reset();
        hasher.update(content.as_bytes());
        let content_hash = format!("{:x}", hasher.finalize());

        tx.execute(
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

        ids.push(id);
    }

    tx.commit()
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(Box::new(e)))?;

    Ok(ids)
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

    for line in stdin.lock().lines() {
        let line = line?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            batch.push(trimmed.to_string());
        }

        // Execute batch when we reach the configured size
        if batch.len() >= batch_size {
            let contents: Vec<&str> = batch.iter().map(|s| s.as_str()).collect();
            match append_batch(conn, source, &contents, meta) {
                Ok(ids) => all_ids.extend(ids),
                Err(e) => {
                    // If we fail, flush the remaining batch and return error
                    if !batch.is_empty() {
                        let contents: Vec<&str> = batch.iter().map(|s| s.as_str()).collect();
                        if let Ok(ids) = append_batch(conn, source, &contents, meta) {
                            all_ids.extend(ids);
                        }
                    }
                    return Err(io::Error::new(io::ErrorKind::Other, e));
                }
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
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }
        }
    }

    Ok(all_ids)
}

/// Check if an event with the same content hash already exists
pub fn is_duplicate(conn: &Connection, content_hash: &str) -> Result<bool> {
    let exists: bool = conn.query_row(
        "SELECT 1 FROM events WHERE content_hash = ?1 LIMIT 1",
        [content_hash],
        |row| row.get(0),
    )?;

    Ok(exists)
}

/// Get statistics about the events
pub fn stats(conn: &Connection) -> Result<(i64, i64, i64, i64)> {
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;
    let unique: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT content_hash) FROM events",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    let oldest: i64 = conn
        .query_row("SELECT MIN(timestamp) FROM events", [], |row| row.get(0))
        .unwrap_or(0);
    let newest: i64 = conn
        .query_row("SELECT MAX(timestamp) FROM events", [], |row| row.get(0))
        .unwrap_or(0);

    Ok((total, unique, oldest, newest))
}
