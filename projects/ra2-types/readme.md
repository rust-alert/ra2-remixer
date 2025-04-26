# RA2 MIX Library

A Rust library for reading and writing Red Alert 2 MIX archive files. Supports both encrypted and unencrypted MIX formats.

## Features

- Read and parse MIX files
- Extract files from MIX archives
- Support for encrypted MIX files
- Checksum calculation for filenames
- File operations (read/write)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ra2-mix = "0.0.0"
```

## Basic Usage

```rust,no_run
use ra2_mix::{MixPackage,MixError};
use std::path::Path;

fn main() -> Result<(), MixError> {
    // Load a MIX file
    let mix = MixPackage::load(Path::new("example.mix"))?;
    
    // Access files in the MIX archive
    for (filename, data) in mix.files.iter() {
        println!("Found file: {}", filename);
    }
    
    Ok(())
}
```

## API Documentation

See the [full API documentation](https://docs.rs/ra2-mix) for detailed usage.

## Examples

Check the `examples/` directory for complete usage examples:

1. `basic.rs` - Basic MIX file operations
2. `encrypted.rs` - Working with encrypted MIX files
3. `extract.rs` - Extracting files from MIX archives
