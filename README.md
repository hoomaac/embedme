# embedme

A command-line tool for embedding secret messages in PNG files. This project is implemented following the [PNGme Book tutorial](https://jrdngr.github.io/pngme_book/introduction.html) as a learning exercise in Rust programming.

## About

This implementation:
- Follows the PNG file format specification
- Handles chunk-level operations (reading, writing, and CRC validation)
- Provides a command-line interface for encoding/decoding hidden messages
- Demonstrates Rust concepts like error handling, type conversions, and safe memory management

## Usage

```bash
# Hide a message
embedme encode <PNG_FILE> <CHUNK_TYPE> <MESSAGE>

# Find a hidden message
embedme decode <PNG_FILE> <CHUNK_TYPE>

# Remove a hidden message
embedme remove <PNG_FILE> <CHUNK_TYPE>

# List all chunks
embedme print <PNG_FILE>
```

## Development

```bash
# Build the project
cargo build --release

# Run tests
cargo test
```

## Acknowledgments

This project was built by following the [PNGme Book](https://jrdngr.github.io/pngme_book/introduction.html), an excellent tutorial for learning Rust through a practical project. The original tutorial was created by Jordan Grace.

## License

MIT License