use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc, Duration};
use serde_json;

#[derive(Debug, Clone)]
pub struct StagedEvent {
    pub id: String,
    pub source: String,
    pub content: String,
    pub meta: Option<String>,
    pub timestamp: i64,
}

impl StagedEvent {
    pub fn from_file(path: &Path) -> Result<Self, serde_json::Error> {
        let content = fs::read_to_string(path)?;
        let event: Self = serde_json::from_str(&content)?;
        Ok(event)
    }

    pub fn timestamp_utc(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.timestamp, 0).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub description: String,
    pub source_events: Vec<String>, // list of event IDs that triggered this pattern
}

pub fn detect_patterns(staging_dir: &Path) -> Result<Vec<Pattern>, Box<dyn std::error::Error>> {
    let mut patterns = Vec::new();

    // Load all staged events
    let entries = fs::read_dir(staging_dir)?;
    let mut events: Vec<StagedEvent> = Vec::new();

    for entry in entries {
        let path = entry?.path();
        if path.extension() == Some(std::ffi::OsStr::new("json")) {
            match StagedEvent::from_file(&path) {
                Ok(event) => events.push(event),
                Err(e) => eprintln!("Failed to parse staging event {}: {}", path.display(), e),
            }
        }
    }

    // Sort by timestamp
    events.sort_by_key(|e| e.timestamp);

    let one_week_ago = Utc::now() - Duration::weeks(1);

    // Pattern 1: Frequent shell commands (nushell-history)
    let mut shell_command_counts = std::collections::HashMap::new();
    for event in &events {
        if event.source == "nushell-history" && event.timestamp_utc() > one_week_ago {
            *shell_command_counts.entry(event.content.clone()).or_insert(0) += 1;
        }
    }

    for (command, count) in shell_command_counts.iter() {
        if *count >= 3 {
            let mut source_ids = Vec::new();
            for event in &events {
                if event.source == "nushell-history" && event.content == *command && event.timestamp_utc() > one_week_ago {
                    source_ids.push(event.id.clone());
                }
            }
            patterns.push(Pattern {
                description: format!("* You ran `{}` {} times in the last week — this suggests you rely on it for routine tasks.", command, count),
                source_events: source_ids,
            });
        }
    }

    // Pattern 2: Repeated dotfile edits (e.g., .config, .bashrc, .rustfmt.toml)
    let mut dotfile_edits = std::collections::HashMap::new();
    for event in &events {
        if event.source.starts_with("dotfile") && event.timestamp_utc() > one_week_ago {
            *dotfile_edits.entry(event.content.clone()).or_insert(0) += 1;
        }
    }

    for (content, count) in dotfile_edits.iter() {
        if *count >= 2 {
            let mut source_ids = Vec::new();
            for event in &events {
                if event.source.starts_with("dotfile") && event.content == *content && event.timestamp_utc() > one_week_ago {
                    source_ids.push(event.id.clone());
                }
            }
            patterns.push(Pattern {
                description: format!("* You edited a configuration file with content like \"{}\" {} times — this suggests iterative refinement of your workflow.", content, count),
                source_events: source_ids,
            });
        }
    }

    // Pattern 3: Sensitive content (e.g., passwords, keys)
    for event in &events {
        if event.content.contains("password") || event.content.contains("secret") || event.content.contains("key=") {
            patterns.push(Pattern {
                description: format!("* You entered sensitive data: \"{}\" — consider using a password manager.", event.content),
                source_events: vec![event.id.clone()],
            });
        }
    }

    Ok(patterns)
}
```

This `infer.rs` module:

- Reads all `.json` files from `staging/`
- Detects 3 patterns:
  1. Frequent shell commands (≥3 in last week)
  2. Repeated dotfile edits (≥2 in last week)
  3. Sensitive content (password, secret, key=)

Each pattern returns a Markdown-friendly description + list of source event IDs for traceability.

Next step: implement `mirror-log infer` CLI command to run this and output proposed reflections.
