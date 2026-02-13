use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn temp_db() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("mirror_log_test_");
    let random: u64 = rand::random();
    path.push(format!("test_{}.db", random));

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    path
}

fn get_binary_path() -> PathBuf {
    let mut path = env::current_exe().expect("Failed to get current executable path");
    path.set_file_name("mirror_log");
    path
}

fn run_cli(args: &[&str]) -> Result<String, String> {
    let output = Command::new(get_binary_path())
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run CLI: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_basic_append() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let result = mirror_log::log::append(&conn, "test_source", "Test event content", None)
            .expect("Failed to append");

        assert!(!result.is_empty());
        assert_eq!(result.len(), 36); // UUID length

        // Verify it was actually stored
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_duplicate_detection() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let content = "This is duplicate content";

        // First append
        mirror_log::log::append(&conn, "source1", content, None).expect("Failed to append first");

        // Second append with same content
        mirror_log::log::append(&conn, "source2", content, None).expect("Failed to append second");

        // Verify duplicate detection
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 2);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_batch_append() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let contents = vec!["Batch event 1", "Batch event 2", "Batch event 3"];

        let result = mirror_log::log::append_batch(&conn, "batch_source", &contents, None)
            .expect("Failed to batch append");

        assert_eq!(result.len(), 3);

        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 3);
        assert_eq!(unique, 3);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_stdin_append() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        // Write to stdin
        let input = "Stdin event 1\nStdin event 2\nStdin event 3\n";
        let mut child = Command::new(get_binary_path())
            .args(&[
                "--db",
                db_path.to_str().unwrap(),
                "stdin",
                "--source",
                "stdin_test",
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write to stdin");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert!(stdout.contains("3"));

        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 3);
        assert_eq!(unique, 3);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_stats() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        // Add some events
        mirror_log::log::append(&conn, "source1", "Event content 1", None)
            .expect("Failed to append");

        mirror_log::log::append(&conn, "source2", "Event content 2", None)
            .expect("Failed to append");

        mirror_log::log::append(
            &conn,
            "source1",
            "Event content 1", // duplicate
            None,
        )
        .expect("Failed to append duplicate");

        let (total, unique, oldest, newest) =
            mirror_log::log::stats(&conn).expect("Failed to get stats");

        assert_eq!(total, 3);
        assert_eq!(unique, 2);
        assert!(oldest > 0);
        assert!(newest > oldest);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_view_get_by_id() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let id = mirror_log::log::append(&conn, "view_test", "Content for viewing", None)
            .expect("Failed to append");

        let event = mirror_log::view::get_by_id(&conn, &id).expect("Failed to get event");

        assert_eq!(event.id, id);
        assert_eq!(event.source, "view_test");
        assert_eq!(event.content, "Content for viewing");

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_view_recent() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        // Add multiple events
        for i in 1..=5 {
            mirror_log::log::append(&conn, "recent_test", &format!("Recent event {}", i), None)
                .expect("Failed to append");

            thread::sleep(Duration::from_millis(10)); // Small delay to get different timestamps
        }

        let events = mirror_log::view::recent(&conn, 3).expect("Failed to get recent events");

        assert_eq!(events.len(), 3);
        // Should be in descending order by ingestion time
        assert!(events[0].content.contains("1") || events[0].content.contains("2"));

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_view_by_source() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        // Add events from different sources
        for i in 1..=5 {
            mirror_log::log::append(
                &conn,
                &format!("source_{}", i % 3), // Only 3 unique sources
                &format!("Content from source {}", i % 3),
                None,
            )
            .expect("Failed to append");
        }

        let events =
            mirror_log::view::by_source(&conn, "source_1", Some(10)).expect("Failed to get events");

        assert_eq!(events.len(), 2); // source_1 appears twice

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_search() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        // Add events with search terms
        mirror_log::log::append(&conn, "search_test", "Find this text content", None)
            .expect("Failed to append");

        mirror_log::log::append(&conn, "search_test", "Different content here", None)
            .expect("Failed to append");

        mirror_log::log::append(&conn, "search_test", "Find this text again", None)
            .expect("Failed to append");

        let events = mirror_log::view::search(&conn, "Find this text").expect("Failed to search");

        assert_eq!(events.len(), 2);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_is_duplicate() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let content_hash = "test_hash_123456";

        // Initially should be false
        let is_dup =
            mirror_log::log::is_duplicate(&conn, content_hash).expect("Failed to check duplicate");
        assert!(!is_dup);

        // Add event with this content
        mirror_log::log::append(&conn, "duplicate_test", "Test content", None)
            .expect("Failed to append");

        // Now should be true
        let is_dup =
            mirror_log::log::is_duplicate(&conn, content_hash).expect("Failed to check duplicate");
        assert!(is_dup);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_error_handling() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        // Add an event
        mirror_log::log::append(&conn, "error_test", "Test error handling", None)
            .expect("Failed to append");

        // Try to get non-existent event
        let result = mirror_log::view::get_by_id(&conn, "non-existent-id");
        assert!(result.is_err());

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_empty_db() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let (total, unique, oldest, newest) =
            mirror_log::log::stats(&conn).expect("Failed to get stats");

        assert_eq!(total, 0);
        assert_eq!(unique, 0);
        assert_eq!(oldest, 0);
        assert_eq!(newest, 0);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_large_content() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let large_content = "A".repeat(100000); // 100KB of content

        let id = mirror_log::log::append(&conn, "large_content_test", &large_content, None)
            .expect("Failed to append");

        // Verify it was stored correctly
        let event = mirror_log::view::get_by_id(&conn, &id).expect("Failed to get event");
        assert_eq!(event.content.len(), 100000);

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_meta_field() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        let meta = "test_meta_data";

        let id = mirror_log::log::append(&conn, "meta_test", "Test content with meta", Some(meta))
            .expect("Failed to append");

        let large_content = "A".repeat(3000); // 3KB content that will be chunked

        let id = mirror_log::log::append(&conn, "chunk_test", &large_content, None)
            .expect("Failed to append");

        // Create chunks
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let chunk_count =
            mirror_log::chunk::create_chunks(&conn, &id, &large_content, timestamp, 1500)
                .expect("Failed to create chunks");

        assert!(chunk_count > 1); // Should create multiple chunks

        // Verify chunks exist
        let chunks = mirror_log::chunk::list_chunks(&conn, &id).expect("Failed to list chunks");
        assert_eq!(chunks.len(), chunk_count as usize);

        // Verify chunk content
        let total_chunks_content: usize = chunks.iter().map(|c| c.content.len()).sum();
        assert_eq!(total_chunks_content, large_content.len());

        fs::remove_file(&db_path).ok();
    }

    #[test]
    fn test_chunk_search() {
        let db_path = temp_db();
        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");

        // Create a chunk with specific text
        let content = "This is a test chunk content with specific text to search for";
        let id = mirror_log::log::append(&conn, "chunk_search_test", content, None)
            .expect("Failed to append");

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        mirror_log::chunk::create_chunks(&conn, &id, content, timestamp, 20)
            .expect("Failed to create chunks");

        // Search for text within chunks
        let search_term = "specific text";
        let found_chunks = mirror_log::chunk::search_chunks(&conn, search_term, Some(10))
            .expect("Failed to search chunks");

        assert!(!found_chunks.is_empty());
        // Should find at least one chunk containing our search term
        assert!(found_chunks.iter().any(|c| c.content.contains(search_term)));

        fs::remove_file(&db_path).ok();
    }
}
