use rusqlite::{Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

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

    conn.execute(
        "INSERT INTO events (id, timestamp, source, content, meta)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (&id, timestamp, source, content, meta),
    )?;

    Ok(id)
}

pub fn append_stdin(conn: &Connection, source: &str) -> std::io::Result<Vec<String>> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut ids = Vec::new();

    // Wrap all inserts in a single transaction for speed
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    for line in stdin.lock().lines() {
        let line = line?;
        if !line.trim().is_empty() {
            let id = append(&tx, source, &line, None)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            ids.push(id);
        }
    }

    tx.commit()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(ids)
}
