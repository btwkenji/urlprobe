use super::super::read;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile; // TODO: fix this

#[test]
fn test_read_urls_from_json() {
    let json_data = r#"["url1", "url2", "url3"]"#;
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "{}", json_data).unwrap();

    let file_path = temp_file.path().to_str().unwrap();
    let result = read::read_urls_from_json(file_path);

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls, vec!["url1", "url2", "url3"]);
}

#[test]
fn test_read_urls_from_csv() {
    let csv_data = "url1\nurl2\nurl3";
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "{}", csv_data).unwrap();

    let file_path = temp_file.path().to_str().unwrap();
    let result = read::read_urls_from_csv(file_path);

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls, vec!["url1", "url2", "url3"]);
}

#[test]
fn test_read_urls_from_txt() {
    let txt_data = "url1\nurl2\nurl3";
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "{}", txt_data).unwrap();

    let file_path = temp_file.path().to_str().unwrap();
    let result = read::read_urls_from_txt(file_path);

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls, vec!["url1", "url2", "url3"]);
}
