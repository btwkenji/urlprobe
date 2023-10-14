mod args;

use clap::{App, Arg};

pub fn parse_args() -> clap::ArgMatches {
    App::new("URL Checker")
        .version("1.0")
        .author("Your Name")
        .about("Checks and tests URLs for status and response time")
        .arg(
            Arg::new("urls")
                .short('u')
                .long("urls")
                .value_name("URL")
                .about("URLs to test")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .value_name("JSON_FILE")
                .about("JSON file containing URLs")
                .takes_value(true),
        )
        .arg(
            Arg::new("csv")
                .short('c')
                .long("csv")
                .value_name("CSV_FILE")
                .about("CSV file containing URLs")
                .takes_value(true),
        )
        .arg(
            Arg::new("txt")
                .short('t')
                .long("txt")
                .value_name("TXT_FILE")
                .about("Text file containing URLs")
                .takes_value(true),
        )
        .get_matches()
}
