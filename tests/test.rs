use urlprobe::read::{read_urls_from_csv, read_urls_from_json, read_urls_from_txt};

#[test]
fn test_read_urls_from_json() {
    let json_data = r#"
    {
        "urls": [
            "https://google.com",
            "https://github.com"
        ]
    }"#;

    let result = read_urls_from_json(json_data);

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://google.com");
    assert_eq!(urls[1], "https://github.com");
}

#[test]
fn test_read_urls_from_csv() {
    let csv_data = "https://google.com\nhttps://github.com";

    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path();
    std::fs::write(&file_path, csv_data).expect("Failed to write to temporary file");
    let result = read_urls_from_csv(&file_path.to_str().unwrap());

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://google.com");
    assert_eq!(urls[1], "https://github.com");
}

#[test]
fn test_read_urls_from_txt() {
    let txt_data = "https://google.com\nhttps://github.com";

    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let file_path = temp_file.path();
    std::fs::write(&file_path, txt_data).expect("Failed to write to temporary file");
    let result = read_urls_from_txt(&file_path.to_str().unwrap());

    assert!(result.is_ok());
    let urls = result.unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://google.com");
    assert_eq!(urls[1], "https://github.com");
}
