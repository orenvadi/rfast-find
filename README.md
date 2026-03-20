# rfast-find

A fast, parallelized `grep`/`find` implementation in Rust for learning purposes.

## Features

- **Parallel Search**: Utilizes `rayon` for efficient multi-threaded file processing.
- **Recursive Discovery**: Automatically crawls directories.
- **Highlighted Output**: Colorizes matches in the terminal.
- **Line Numbers**: Displays matching line numbers.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
rfast-find <TEXT_TO_SEARCH>
```

### Options

- `-i, --ignore-case`: Ignore case for text search (planned)
- `-n, --line-number`: Show line number
- `-h, --help`: Print help
- `-V, --version`: Print version

## License

Copyright (C) 2026 Nursultan Baktybekov

This project is licensed under the **GNU General Public License v2.0 or later**. See the [LICENSE](LICENSE) file for details.
