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

pub fn append_stdin(conn: &Connection, source: &str) -> Result<Vec<String>> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut ids = Vec::new();

    for line in stdin.lock().lines() {
        let line = line?;
        if !line.trim().is_empty() {
            let id = append(conn, source, &line, None)?;
            ids.push(id);
        }
    }

    Ok(ids)
}
