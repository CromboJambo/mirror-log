CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    source TEXT NOT NULL,
    content TEXT NOT NULL,
    meta TEXT
);

CREATE INDEX IF NOT EXISTS idx_events_time
ON events(timestamp);

CREATE INDEX IF NOT EXISTS idx_events_source
ON events(source);

-- Chunks: semantic sections of events for better search
CREATE TABLE IF NOT EXISTS chunks (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    content TEXT NOT NULL,
    start_offset INTEGER NOT NULL,
    end_offset INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id)
);

CREATE INDEX IF NOT EXISTS idx_chunks_event
ON chunks(event_id);

CREATE INDEX IF NOT EXISTS idx_chunks_time
ON chunks(timestamp);
