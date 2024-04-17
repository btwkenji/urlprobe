## Features

- A simple command-line tool in Rust which is checking url health and performance.
- Provides a command-line interface for checking url health and performance.

## Installation

```sh
git clone https://github.com/kenjitheman/urlprobe
```

```sh
cargo build --release
```

```sh
cargo run --bin urlprobe
```

## Usage

### Options

- `-s, --source <SOURCE>`: Specify the data source (Required).
  - `<SOURCE>`: The data source type. Must be one of: `json`, `csv`, `txt`, or `list`

- `-f, --file <FILENAME>`: Specify the data source file
  - `<FILENAME>`: The filename to read data from

- `-u, --urls <URLS...>`: Specify the URLs to probe
  - `<URLS...>`: A list of URLs to probe

- `-h, --help`: Print help information

- `-V, --version`: Print the version information

### Examples

- Json structure example

```json
{
  "urls": [
    "https://google.com",
    "https://github.com",
    "https://twitter.com",
    "https://youtube.com",
    "https://instagram.com",
    "https://wikipedia.org",
  ]
}
```

- This command probes URLs from the JSON file `data.json`

```sh
cargo run --bin urlprobe -s json -f data.json
```

- CSV structure example

```csv
urls
https://google.com
https://github.com
https://facebook.com
https://twitter.com
```

- This command probes URLs from the CSV file `data.csv`

```sh
cargo run --bin urlprobe -s csv -f data.csv
```

- TXT structure example

```txt
https://google.com
https://github.com
https://twitter.com
https://youtube.com
```

- This command probes URLs from the TXT file `data.txt`

```sh
cargo run --bin urlprobe -s txt -f data.txt
```

- Using command-line arguments
- This command probes the specified list of URLs

```sh
cargo run --bin urlprobe -s list -u https://example.com https://test.com
```

## Contributing

- Pull requests are welcome, for major changes, please open an issue first to
  discuss what you would like to change.

- Please make sure to update tests as appropriate.

## License

- [MIT](./LICENSE)
