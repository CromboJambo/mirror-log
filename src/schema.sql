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
