use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub timestamp: i64,
    pub source: String,
    pub content: String,
    pub meta: Option<String>,
}

impl Event {
    pub fn format_time(&self) -> String {
        let dt: DateTime<Utc> = Utc.timestamp_opt(self.timestamp, 0).unwrap();
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

pub fn recent(conn: &Connection, limit: i64) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, source, content, meta
         FROM events
         ORDER BY timestamp DESC
         LIMIT ?1",
    )?;

    let rows = stmt.query_map([limit], |row| {
        Ok(Event {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            source: row.get(2)?,
            content: row.get(3)?,
            meta: row.get(4)?,
        })
    })?;

    Ok(rows.filter_map(Result::ok).collect())
}

pub fn search(conn: &Connection, term: &str) -> Result<Vec<Event>> {
    let like = format!("%{}%", term);

    let mut stmt = conn.prepare(
        "SELECT id, timestamp, source, content, meta
         FROM events
         WHERE content LIKE ?1
         ORDER BY timestamp DESC",
    )?;

    let rows = stmt.query_map([like], |row| {
        Ok(Event {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            source: row.get(2)?,
            content: row.get(3)?,
            meta: row.get(4)?,
        })
    })?;

    Ok(rows.filter_map(Result::ok).collect())
}

pub fn by_source(conn: &Connection, source: &str, limit: Option<i64>) -> Result<Vec<Event>> {
    let query = if let Some(lim) = limit {
        format!(
            "SELECT id, timestamp, source, content, meta
             FROM events
             WHERE source = ?1
             ORDER BY timestamp DESC
             LIMIT {}",
            lim
        )
    } else {
        "SELECT id, timestamp, source, content, meta
         FROM events
         WHERE source = ?1
         ORDER BY timestamp DESC"
            .to_string()
    };

    let mut stmt = conn.prepare(&query)?;

    let rows = stmt.query_map([source], |row| {
        Ok(Event {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            source: row.get(2)?,
            content: row.get(3)?,
            meta: row.get(4)?,
        })
    })?;

    Ok(rows.filter_map(Result::ok).collect())
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Event> {
    conn.query_row(
        "SELECT id, timestamp, source, content, meta
         FROM events
         WHERE id = ?1",
        [id],
        |row| {
            Ok(Event {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                source: row.get(2)?,
                content: row.get(3)?,
                meta: row.get(4)?,
            })
        },
    )
}
