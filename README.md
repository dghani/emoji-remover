# emoji-remover

A fast, multi-threaded command-line tool to remove emojis from source code files. Built with Rust for performance and reliability.

## Features

- **Fast**: Multi-threaded processing using Rayon
- **Smart**: Respects `.gitignore` files by default
- **Flexible**: Process individual files or entire directories
- **Safe**: Dry-run mode to preview changes before applying
- **Clean**: Removes trailing spaces left after emoji removal
- **Zero Dependencies**: Single binary with no runtime dependencies

## Installation

### From Source

```bash
cargo install emoji-remover
```

### From Binary

Download the latest release from the [releases page](https://github.com/jamiepine/emoji-remover/releases).

## Usage

```bash
# Remove emojis from a single file
emoji-remover path/to/file.rs

# Remove emojis from an entire directory
emoji-remover path/to/project/

# Dry run - see what would be removed without making changes
emoji-remover --dry-run path/to/project/

# Process specific file extensions only
emoji-remover --extensions "rs,js,ts" path/to/project/

# Ignore .gitignore files
emoji-remover --no-ignore path/to/project/

# Quiet mode - only show files with changes
emoji-remover --quiet path/to/project/
```

## As a Git Pre-commit Hook

The repository includes an installation script for setting up emoji-remover as a pre-commit hook:

```bash
# Clone the repo
git clone https://github.com/jamiepine/emoji-remover.git
cd emoji-remover

# Build the tool
cargo build --release

# Install as pre-commit hook in your project
cd /path/to/your/project
/path/to/emoji-remover/install-pre-commit.sh
```

## Options

```
-d, --dry-run         Perform a dry run without modifying files
-e, --extensions      File extensions to process (comma-separated)
-q, --quiet          Quiet mode - only show files with changes
    --no-ignore      Don't respect .gitignore files
-h, --help           Print help
```

## Default File Extensions

By default, emoji-remover processes files with these extensions:
`rs, js, ts, jsx, tsx, py, java, c, cpp, h, hpp, go, rb, php, cs, swift, kt, scala, md, txt`

## Data Source

The emoji data used by this tool is sourced from [Emoji-List-Unicode](https://github.com/Tarikul-Islam-Anik/Emoji-List-Unicode), which provides a comprehensive list of emojis from the [Unicode official website](https://unicode.org/emoji/charts/full-emoji-list.html).

The tool includes:
- All standard emojis from `all-emoji.json`
- Emoji variations with skin tone modifiers from `full-emoji-modifiers.json`

## Build Process

The emoji data is processed at compile time using a Rust build script (`build.rs`). This approach:
- Embeds all emoji patterns directly into the binary
- Eliminates runtime file I/O for emoji data
- Results in a single, self-contained executable

## Performance

- Multi-threaded file processing using Rayon
- Efficient `.gitignore` handling via the `ignore` crate
- Zero-allocation string replacements where possible
- Typical processing speed: ~1000 files/second on modern hardware

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Emoji data from [Emoji-List-Unicode](https://github.com/Tarikul-Islam-Anik/Emoji-List-Unicode)
- Built with [Rust](https://www.rust-lang.org/) and the amazing Rust ecosystem