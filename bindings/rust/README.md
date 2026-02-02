# OpenDAFF Rust Bindings

Rust bindings for the OpenDAFF library, providing idiomatic Rust access to directional audio data in DAFF format.

## Overview

OpenDAFF is a software package for directional audio content like directivities of sound sources (loudspeakers, musical instruments) and sound receivers (microphones, HRTFs/HRIRs). These Rust bindings provide a safe, native Rust interface to read and process DAFF files.

## Features

- **Idiomatic Rust API** with proper error handling using `Result<T, Error>`
- **Full content type support**: IR, MS, PS, MPS, and DFT
- **Memory safe** with automatic resource cleanup (RAII via Drop trait)
- **Type-safe** enums and structures
- **Zero-cost abstractions** over the C++ library
- **Thread-safe** (`Send` + `Sync`)

## Installation

### Prerequisites

1. **OpenDAFF C++ library**: Build and install the core DAFF library first
2. **Rust 1.70+**: Required for the bindings
3. **C++ compiler**: For building the C wrapper (gcc or clang)

### Building the C Wrapper

From the OpenDAFF root directory:

```bash
# Using justfile (recommended)
just build-rust

# Or using CMake directly
cmake -DOPENDAFF_WITH_RUST_BINDING=ON .
make
make install
```

### Installing the Rust Crate

```bash
cd bindings/rust
cargo build --release
cargo test
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
opendaff = { path = "/path/to/opendaff/bindings/rust" }
```

## Quick Start

```rust
use opendaff::{Reader, ContentType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new reader
    let mut reader = Reader::new()?;

    // Open a DAFF file
    reader.open_file("path/to/file.daff")?;

    // Get file properties
    let content_type = reader.content_type();
    let num_channels = reader.num_channels();
    let num_records = reader.num_records();

    println!("Content Type: {}", content_type);
    println!("Channels: {}", num_channels);
    println!("Records: {}", num_records);

    // Process based on content type
    match content_type {
        ContentType::ImpulseResponse => {
            let ir = reader.content_ir()?;
            println!("Filter Length: {} samples", ir.filter_length());
            println!("Sample Rate: {} Hz", ir.samplerate());

            // Get impulse response for front direction (phi=0, theta=0)
            let record_idx = ir.nearest_neighbour(0.0, 0.0);

            // Get filter coefficients for left channel
            let coeffs = ir.filter_coeffs(record_idx, 0)?;
            println!("Retrieved {} filter coefficients", coeffs.len());
        },
        _ => println!("Other content types..."),
    }

    Ok(())
}
```

## API Overview

### Reader

The main interface for reading DAFF files:

```rust
use opendaff::Reader;

let mut reader = Reader::new()?;
reader.open_file("file.daff")?;

// File properties
let content_type = reader.content_type();
let quantization = reader.quantization();
let num_channels = reader.num_channels();
let num_records = reader.num_records();
let (yaw, pitch, roll) = reader.orientation()?;

// Metadata
if reader.has_metadata("Description") {
    let desc = reader.metadata_string("Description")?;
    println!("Description: {}", desc);
}

// Explicit cleanup (optional, automatic on drop)
reader.close();
```

### Content Types

#### Impulse Response (IR)

```rust
let ir = reader.content_ir()?;

let filter_length = ir.filter_length();
let samplerate = ir.samplerate();

// Find nearest record for given angles (radians)
let record_idx = ir.nearest_neighbour(0.0, 0.0);

// Get record coordinates (data view)
let (alpha, beta) = ir.record_coords(record_idx)?;

// Get filter coefficients
let coeffs = ir.filter_coeffs(record_idx, channel)?;
```

#### Magnitude Spectrum (MS)

```rust
let ms = reader.content_ms()?;

let num_freqs = ms.num_frequencies();
let record_idx = ms.nearest_neighbour(phi, theta);
let magnitudes = ms.magnitudes(record_idx, channel)?;
```

#### Phase Spectrum (PS)

```rust
let ps = reader.content_ps()?;

let num_freqs = ps.num_frequencies();
let phases = ps.phases(record_idx, channel)?;
```

#### Magnitude-Phase Spectrum (MPS)

```rust
let mps = reader.content_mps()?;

let num_freqs = mps.num_frequencies();
let (magnitudes, phases) = mps.coefficients(record_idx, channel)?;
```

#### DFT Coefficients

```rust
let dft = reader.content_dft()?;

let num_coeffs = dft.num_dft_coeffs();
let is_symmetric = dft.is_symmetric();

// Returns interleaved real/imaginary: [real0, imag0, real1, imag1, ...]
let coeffs = dft.dft_coeffs(record_idx, channel)?;
```

## Coordinate System

OpenDAFF uses the OpenGL coordinate system with spherical views:

### Object View (phi, theta)

User-facing directional queries:

- **phi**: Azimuth angle (rotation around Y axis), range [0, 2π) radians
- **theta**: Elevation angle, range [-π/2, π/2] radians
- **Front**: (0, 0), **Up**: (0, π/2)

### Data View (alpha, beta)

Internal data storage coordinates:

- Used by content implementations
- Automatically converted from object view

All angles in the API are in **radians**.

## Error Handling

All fallible functions return `Result<T, Error>`:

```rust
use opendaff::{Reader, Error};

match reader.open_file("file.daff") {
    Ok(_) => println!("File opened successfully"),
    Err(e) => eprintln!("Failed to open file: {}", e),
}

// Or using the ? operator
let ir = reader.content_ir()?;
```

Common errors:

- File not found or cannot be opened
- Invalid file format
- Wrong content type requested
- Buffer size mismatch
- Invalid record or channel index

## Memory Management

The bindings use Rust's RAII pattern:

- Resources are automatically cleaned up when `Reader` goes out of scope
- `Drop` trait implementation ensures proper cleanup
- No manual memory management required
- Thread-safe: implements `Send` + `Sync`

```rust
{
    let mut reader = Reader::new()?;
    reader.open_file("file.daff")?;
    // Use reader...
} // Automatically cleaned up here
```

## Examples

### Basic Usage

```bash
cargo run --example basic_usage -- path/to/file.daff
```

See [examples/basic_usage.rs](examples/basic_usage.rs) for a complete example showing:

- Opening and reading DAFF files
- Handling different content types
- Querying directional data
- Accessing metadata

### Processing All Records

```rust
use opendaff::{Reader, ContentType};

let mut reader = Reader::new()?;
reader.open_file("file.daff")?;

if reader.content_type() == ContentType::ImpulseResponse {
    let ir = reader.content_ir()?;
    let num_records = reader.num_records();
    let num_channels = reader.num_channels();

    for record in 0..num_records {
        for channel in 0..num_channels {
            let coeffs = ir.filter_coeffs(record, channel)?;
            // Process coefficients...
        }
    }
}
```

### Spatial Audio Processing

```rust
use std::f64::consts::PI;

// Query multiple directions
let directions = [
    (0.0, 0.0),              // Front
    (PI/2.0, 0.0),           // Right
    (PI, 0.0),               // Back
    (3.0*PI/2.0, 0.0),       // Left
    (0.0, PI/2.0),           // Up
];

let ir = reader.content_ir()?;

for (phi, theta) in &directions {
    let record_idx = ir.nearest_neighbour(*phi, *theta);
    let coeffs = ir.filter_coeffs(record_idx, 0)?;
    println!("Direction ({:.2}, {:.2}): {} samples", phi, theta, coeffs.len());
}
```

## Testing

Run the test suite:

```bash
cd bindings/rust
cargo test
```

For integration tests with actual DAFF files, place test files in `testdata/` directory.

## Building from Source

### Standard Build

```bash
cd bindings/rust
cargo build --release
```

### Development Build with Debug Info

```bash
cargo build
```

### Running Tests

```bash
cargo test --all
```

### Generating Documentation

```bash
cargo doc --open
```

## Library Linking

The bindings link against:

1. `libdaffrustwrapper` - The C wrapper library
2. `libDAFF` - The core DAFF library
3. C++ standard library (`libstdc++` on Linux, `libc++` on macOS)

### Runtime Library Path

On Linux/macOS, you may need to set the library path:

```bash
# Linux
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=/usr/local/lib:$DYLD_LIBRARY_PATH
```

Or use rpath (configured in `build.rs`).

## Performance Notes

- Data retrieval functions allocate new Rust vectors
- The C++ library is zero-copy internally
- Rust vectors use the standard allocator
- Thread-safe for concurrent reads from different `Reader` instances

## Cross-Compilation

The bindings use FFI which complicates cross-compilation:

1. Build OpenDAFF for the target platform
2. Set `CARGO_BUILD_TARGET`
3. Configure appropriate linker in `.cargo/config.toml`
4. Ensure wrapper library is available for target

## Minimum Supported Rust Version (MSRV)

Rust 1.70 or higher is required.

## Contributing

Contributions are welcome! Please ensure:

- Code follows Rust conventions (`cargo fmt`, `cargo clippy`)
- Tests pass: `cargo test`
- Documentation is updated
- No unsafe code outside of FFI boundary

## License

OpenDAFF is distributed under the Apache License Version 2.0.

Copyright 2016-2018 Institute of Technical Acoustics (ITA), RWTH Aachen University

## Links

- [OpenDAFF Project](https://www.opendaff.org)
- [GitHub Repository](https://github.com/MeKo-Tech/opendaff)
- [File Format Specification](../../FILEFORMAT.md)
- [Main Documentation](../../README.md)
- [Go Bindings](../go/README.md)
- [Python Bindings](../python/README.md)

## Support

For issues and questions:

- Open an issue on [GitHub](https://github.com/MeKo-Tech/opendaff/issues)
- Consult the main OpenDAFF documentation
- Check the examples in `examples/`

## Architecture

The Rust bindings follow a three-layer architecture:

1. **C++ Core** (`libDAFF`) - The original OpenDAFF library
2. **C Wrapper** (`libdaffrustwrapper`) - Extern "C" interface for FFI
3. **Rust Wrapper** (`opendaff` crate) - Idiomatic Rust API

This design ensures:

- Memory safety through Rust's ownership system
- No manual memory management
- Type safety with Rust's type system
- Ergonomic API following Rust conventions
