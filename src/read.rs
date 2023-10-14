use serde_json;
use std::error::Error;
use std::fs;

pub fn read_urls_from_json(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string(file_path)?;
    let urls: Vec<String> = serde_json::from_str(&file)?;
    Ok(urls)
}

pub fn read_urls_from_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls = Vec::new();
    let mut rdr = csv::ReaderBuilder::new().from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        if let Some(url) = record.get(0) {
            urls.push(url.to_string());
        }
    }
    Ok(urls)
}

pub fn read_urls_from_txt(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string(file_path)?;
    let urls: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    Ok(urls)
}
