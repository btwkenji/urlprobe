use std::error::Error;
use std::fs;

pub fn read_urls_from_json(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let data: Result<serde_json::Value, _> = serde_json::from_str(&contents);
    match data {
        Ok(value) => {
            if let Some(urls) = value.get("urls") {
                if let Some(urls) = urls.as_array() {
                    let urls: Vec<String> = urls
                        .iter()
                        .filter_map(|u| u.as_str())
                        .map(String::from)
                        .collect();
                    Ok(urls)
                } else {
                    Err("Invalid JSON structure: 'urls' is not an array".into())
                }
            } else {
                Err("Invalid JSON structure: 'urls' key is missing".into())
            }
        }
        Err(e) => Err(e.into()),
    }
}

pub fn read_urls_from_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls = vec![];
    let file = fs::File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
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
    let urls: Vec<String> = file.lines().map(String::from).collect();
    Ok(urls)
}
