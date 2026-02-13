use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn temp_db() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("mirror_log_large_file_test_");
    let random: u64 = rand::random();
    path.push(format!("large_file_test_{}.db", random));

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    path
}

fn get_binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mirror_log"))
}

#[cfg(test)]
mod large_file_tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::io::Write;

    #[test]
    fn test_1mb_file() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        // Create 1MB file
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        for i in 0..1000000 {
            writeln!(file, "Line {} with some content", i).expect("Failed to write to file");
        }

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_10mb_file() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        // Create 10MB file
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        for i in 0..10000000 {
            writeln!(file, "Line {} with some content", i).expect("Failed to write to file");
        }

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_50mb_file() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        // Create 50MB file
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        for i in 0..50000000 {
            writeln!(file, "Line {} with some content", i).expect("Failed to write to file");
        }

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_binary_file() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("bin");

        // Create binary file
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        for i in 0..1000 {
            file.write_all(&[i as u8]).expect("Failed to write to file");
        }

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_special_characters_file() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let special_content = r#"Special characters:
!@#$%^&*()_+-=[]{}|;':",./<>?\~`!@#$%^&*()_+-=[]{}|;':",./<>?\n
Newlines and tabs:\t\n\r
Unicode: 你好世界 🌍
Emoji: 🎉🎊🎈🎂🎁
Binary: \x00\x01\x02\x03
Control chars: \x07\x08\x0B\x0C\x0E
Quotes: " ' `
Backslash: \
Angle brackets: < >
Dollar sign: $
Plus sign: +
Minus sign: -
Asterisk: *
Slash: /
Question mark: ?
Exclamation mark: !
Equals sign: =
Pipe: |
Hash: #
At sign: @
Carat: ^
Underscore: _
Ampersand: &
Percent: %
Tilde: ~
Vertical bar: |
Colon: :
Semicolon: ;
Period: .
Comma: ,
Space:
Tab:
Form feed:
Carriage return:
Line feed:
Null byte:
Vertical tab:
#""#;

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", special_content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "special_chars",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_newlines_file() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let multiline_content =
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", multiline_content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "newlines",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_empty_file() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        // Write nothing

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_very_long_line() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let long_line = "A".repeat(100000); // 100KB line

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", long_line).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_file_exceeding_chunk_size() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        // Create file with content that will exceed chunk size
        let content = "A".repeat(10000); // 10KB content

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_file_with_json_meta() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let content = "Test content with JSON metadata";
        let meta =
            r#"{"key": "value", "number": 123, "array": [1, 2, 3], "nested": {"key": "value"}}"#;

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
                "--meta",
                meta,
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_file_with_malformed_json_meta() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let content = "Test content with malformed JSON";
        let meta = r#"{"key": "value", "invalid": "}}"#; // Malformed JSON

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
                "--meta",
                meta,
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_file_with_null_byte() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let content = "Test with null byte\x00 and more text";

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_file_with_very_long_source_name() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let content = "Test content";
        let long_source = "a".repeat(200); // Very long source name

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                &long_source,
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_file_with_very_long_content() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        let content = "A".repeat(1000000); // 1MB content

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_file_with_special_chars_in_path() {
        let db_path = temp_db();
        let file_path = temp_db();
        file_path.set_extension("txt");

        // Create file with special characters in content
        let content = "Test with special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?";

        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        writeln!(file, "{}", content).expect("Failed to write to file");

        let mut child = Command::new(get_binary_path())
            .args([
                "--db",
                db_path.to_str().unwrap(),
                "add-file",
                file_path.to_str().unwrap(),
                "--source",
                "test",
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let output = child
            .wait_with_output()
            .expect("Failed to wait for process");
        assert!(output.status.success());

        let conn = mirror_log::db::init_db(&db_path).expect("Failed to initialize DB");
        let (total, unique, _, _) = mirror_log::log::stats(&conn).expect("Failed to get stats");
        assert_eq!(total, 1);
        assert_eq!(unique, 1);

        fs::remove_file(&db_path).ok();
        fs::remove_file(&file_path).ok();
    }
}
