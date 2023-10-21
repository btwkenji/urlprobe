use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(subcommand)]
    pub source: Source,
}

#[derive(Clap)]
pub enum Source {
    #[clap(about = "Fetch URLs from a list")]
    Urls(Vec<String>),

    #[clap(about = "Fetch URLs from a JSON file")]
    JsonFile(String),

    #[clap(about = "Fetch URLs from a CSV file")]
    CsvFile(String),

    #[clap(about = "Fetch URLs from a plain text file")]
    TxtFile(String),
}
