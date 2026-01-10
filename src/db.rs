use rusqlite::{Connection, Result};

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(conn)
}

pub fn db_info(conn: &Connection) -> Result<(i64, i64, i64)> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;

    let oldest: i64 = conn
        .query_row("SELECT MIN(timestamp) FROM events", [], |row| row.get(0))
        .unwrap_or(0);

    let newest: i64 = conn
        .query_row("SELECT MAX(timestamp) FROM events", [], |row| row.get(0))
        .unwrap_or(0);

    Ok((count, oldest, newest))
}
