use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StagedEvent {
    pub id: String,
    pub source: String,
    pub content: String,
    pub meta: Option<String>,
    pub timestamp: i64,
}

impl StagedEvent {
    pub fn new(source: &str, content: &str, meta: Option<&str>) -> Self {
        let now = Utc::now().timestamp();
        let id = Uuid::new_v4().to_string();

        Self {
            id,
            source: source.to_string(),
            content: content.to_string(),
            meta: meta.map(|s| s.to_string()),
            timestamp: now,
        }
    }

    pub fn save_to_file(&self, staging_dir: &Path) -> Result<(), std::io::Error> {
        let filename = format!("{}.json", self.id);
        let path = staging_dir.join(filename);

        fs::create_dir_all(staging_dir)?;
        let json = serde_json::to_string_pretty(self)?;

        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(id: &str, staging_dir: &Path) -> Result<Self, std::io::Error> {
        let filename = format!("{}.json", id);
        let path = staging_dir.join(filename);

        let json = fs::read_to_string(path)?;
        let event: StagedEvent = serde_json::from_str(&json)?;
        Ok(event)
    }
}
