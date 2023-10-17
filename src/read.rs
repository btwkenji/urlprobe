use csv;
use serde_json;
use std::error::Error;
use std::fs;

pub fn read_urls_from_json(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let urls: Vec<String> = serde_json::from_str(&contents)?;
    Ok(urls)
}

pub fn read_urls_from_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls = vec![];
    let file = fs::File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        urls.push(record.get(0).unwrap().to_string());
    }

    Ok(urls)
}

pub fn read_urls_from_txt(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string(file_path)?;
    let urls: Vec<String> = file.lines().map(str::to_string).collect();
    Ok(urls)
}
