use clap::{App, Arg};
use reqwest;
use std::error::Error;
use std::time::Instant;
pub mod read;
use read::{read_urls_from_csv, read_urls_from_json, read_urls_from_txt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("urlprobe")
        .version("0.1.0")
        .author("kenjitheman")
        .about("probe URLs for their status code and response time")
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .value_name("SOURCE")
                .help("specify the data source: json, csv, txt, or list")
                .takes_value(true)
                .required(true)
                .possible_values(&["json", "csv", "txt", "list"]),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILENAME")
                .help("specify the data source file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("urls")
                .short("u")
                .long("urls")
                .value_name("URLS")
                .help("specify the URLs to probe")
                .takes_value(true)
                .multiple(true)
                .use_delimiter(true),
        )
        .get_matches();

    let source = matches.value_of("source").unwrap();

    let urls = if let Some(urls) = matches.values_of("urls") {
        urls.map(String::from).collect::<Vec<String>>()
    } else {
        match source {
            "json" => {
                let filename = matches.value_of("file").unwrap_or("man.json");
                read_urls_from_json(&filename)?
            }
            "csv" => {
                let filename = matches.value_of("file").unwrap_or("man.csv");
                read_urls_from_csv(&filename)?
            }
            "txt" => {
                let filename = matches.value_of("file").unwrap_or("man.txt");
                read_urls_from_txt(&filename)?
            }
            _ => {
                eprintln!("[-] invalid source type");
                return Ok(());
            }
        }
    };

    if urls.is_empty() {
        println!("[-] no URLs found in the input");
        return Ok(());
    }

    for url in &urls {
        test_url(url).await?;
    }

    Ok(())
}

async fn test_url(url: &str) -> Result<(), reqwest::Error> {
    let start_time = Instant::now();
    let response = reqwest::get(url).await?;
    let elapsed_time = start_time.elapsed();

    if response.status().is_success() {
        println!("[+] URL: {}", url);
    } else {
        println!("[!] URL: {}", url);
    }

    println!("  -- [?] status: {:?}", response.status());
    println!("  -- [?] response time: {:?}", elapsed_time);

    Ok(())
}
