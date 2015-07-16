# RA2 Remixer - Red Alert 2 Tool Collection

A collection of utilities for working with Red Alert 2 game files, particularly MIX archives.

## Features

- **MIX File Support**: Read, write, and modify RA2 MIX archives
- **Encryption/Decryption**: Handle RA2's custom encryption for MIX files
- **File Management**: Extract and patch files from/to MIX archives
- **CRC Calculation**: Calculate file IDs using RA2's CRC algorithm

## Installation

1. Ensure you have Rust installed (version 1.70.0 or higher)
2. Clone this repository
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

### Basic Commands

```sh
# Extract files from a MIX archive
ra2-mix extract input.mix output_directory

# Patch files into a MIX archive
ra2-mix patch input_directory output.mix
```

### Library Usage

```rust
use ra2_mix::XccPackage;

// Load a MIX file
let mut package = XccPackage::load(Path::new("input.mix"))?;

// Add a file to the package
package.add_file(Path::new("new_file.txt"))?;

// Save the modified package
package.save(Path::new("output.mix"))?;
```

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request

## License

MIT