use reqwest;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://google.com",
        "https://github.com",
        "https://reddit.com",
        "https://rust-lang.org",
        "https://stackoverflow.com",
        "https://youtube.com",
        "https://amazon.com",
        "https://linkedin.com",
        "https://netflix.com",
        "https://ebay.com",
        "https://instagram.com",
        "https://bing.com",
        "https://wikipedia.org",
        "https://pinterest.com",
        "https://wordpress.com",
        "https://apple.com",
        "https://imdb.com",
        "https://adobe.com",
        "https://tumblr.com",
        "https://microsoft.com",
        "https://whatsapp.com",
        "https://espn.com",
        "https://craigslist.org",
        "https://paypal.com",
        "https://nytimes.com",
        "https://twitch.tv",
        "https://dropbox.com",
        "https://quora.com",
        "https://wikia.com",
        "https://live.com",
    ];

    for url in urls {
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
