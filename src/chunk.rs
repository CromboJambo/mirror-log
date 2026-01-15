use rusqlite::{Connection, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct Chunk {
    pub id: String,
    pub event_id: String,
    pub chunk_index: i64,
    pub content: String,
    pub start_offset: i64,
    pub end_offset: i64,
}

/// Split content into chunks based on paragraphs or size
pub fn chunk_content(content: &str, max_chunk_size: usize) -> Vec<(usize, usize, String)> {
    let mut chunks = Vec::new();

    // Split on double newlines (paragraphs)
    let paragraphs: Vec<&str> = content.split("\n\n").collect();

    let mut current_chunk = String::new();
    let mut chunk_start = 0;
    let mut current_pos = 0;

    for para in paragraphs {
        let para_len = para.len() + 2; // +2 for the \n\n we split on

        // If adding this paragraph exceeds max size and we have content, save chunk
        if !current_chunk.is_empty() && current_chunk.len() + para_len > max_chunk_size {
            chunks.push((chunk_start, current_pos, current_chunk.trim().to_string()));
            current_chunk.clear();
            chunk_start = current_pos;
        }

        if !current_chunk.is_empty() {
            current_chunk.push_str("\n\n");
        }
        current_chunk.push_str(para);
        current_pos += para_len;
    }

    // Don't forget the last chunk
    if !current_chunk.is_empty() {
        chunks.push((chunk_start, current_pos, current_chunk.trim().to_string()));
    }

    // If content has no paragraphs, split by size
    if chunks.is_empty() && !content.is_empty() {
        let mut start = 0;
        while start < content.len() {
            let end = (start + max_chunk_size).min(content.len());
            let chunk_text = content[start..end].to_string();
            chunks.push((start, end, chunk_text));
            start = end;
        }
    }

    chunks
}

/// Create chunks for an event
pub fn create_chunks(
    conn: &Connection,
    event_id: &str,
    content: &str,
    timestamp: i64,
    max_chunk_size: usize,
) -> Result<usize> {
    let chunks = chunk_content(content, max_chunk_size);

    let mut count = 0;
    for (idx, (start, end, chunk_content)) in chunks.iter().enumerate() {
        let chunk_id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO chunks (id, event_id, chunk_index, content, start_offset, end_offset, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &chunk_id,
                event_id,
                idx as i64,
                chunk_content,
                *start as i64,
                *end as i64,
                timestamp,
            ),
        )?;
        count += 1;
    }

    Ok(count)
}

/// Search chunks
pub fn search_chunks(conn: &Connection, term: &str, limit: Option<i64>) -> Result<Vec<Chunk>> {
    let like = format!("%{}%", term);

    let query = if let Some(lim) = limit {
        format!(
            "SELECT id, event_id, chunk_index, content, start_offset, end_offset
             FROM chunks
             WHERE content LIKE ?1
             ORDER BY timestamp DESC
             LIMIT {}",
            lim
        )
    } else {
        "SELECT id, event_id, chunk_index, content, start_offset, end_offset
         FROM chunks
         WHERE content LIKE ?1
         ORDER BY timestamp DESC"
            .to_string()
    };

    let mut stmt = conn.prepare(&query)?;

    let rows = stmt.query_map([like], |row| {
        Ok(Chunk {
            id: row.get(0)?,
            event_id: row.get(1)?,
            chunk_index: row.get(2)?,
            content: row.get(3)?,
            start_offset: row.get(4)?,
            end_offset: row.get(5)?,
        })
    })?;

    Ok(rows.filter_map(Result::ok).collect())
}
