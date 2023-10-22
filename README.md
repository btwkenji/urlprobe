# urlprobe

###

<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-plain.svg" height="200" alt="rust logo"  />
</div>

###

### that the tool is guarding or watching over URLs to ensure their availability and performance

## project structure:

```rust
.
├── LICENSE
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── lib.rs
│   ├── main.rs
│   └── read.rs
└── tests
    └── test.rs
```

## features

- a simple command-line tool in Rust which is checking url health and performance 
- provides a command-line interface for checking url health and performance

## installation

```sh
git clone https://github.com/kenjitheman/urlprobe
```

```sh
cargo build --release
```

```sh
cargo run --bin urlprobe
```

## usage

### options

- `-s, --source <SOURCE>`: Specify the data source (Required).
  - `<SOURCE>`: The data source type. Must be one of: `json`, `csv`, `txt`, or `list`

- `-f, --file <FILENAME>`: Specify the data source file
  - `<FILENAME>`: The filename to read data from

- `-u, --urls <URLS...>`: Specify the URLs to probe
  - `<URLS...>`: A list of URLs to probe

- `-h, --help`: Print help information

- `-V, --version`: Print the version information

### examples

- this command probes URLs from the JSON file `data.json`

```sh
cargo run --bin urlprobe -s json -f data.json
```

- this command probes the specified list of URLs

```sh
cargo run --bin urlprobe -s list -u https://example.com https://test.com
```

## contributing

- pull requests are welcome, for major changes, please open an issue first to
  discuss what you would like to change

- please make sure to update tests as appropriate

## license

- [MIT](https://choosealicense.com/licenses/mit/)
