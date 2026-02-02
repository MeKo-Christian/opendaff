# Building OpenDAFF Rust Bindings

This document provides detailed instructions for building the Rust bindings for OpenDAFF.

## Prerequisites

Before building the Rust bindings, ensure you have:

1. **Rust toolchain** (1.70 or later)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **CMake** (3.10 or later)

   ```bash
   # Ubuntu/Debian
   sudo apt install cmake

   # macOS
   brew install cmake
   ```

3. **C++ compiler** (gcc or clang)

   ```bash
   # Ubuntu/Debian
   sudo apt install build-essential

   # macOS (Xcode Command Line Tools)
   xcode-select --install
   ```

4. **OpenDAFF C++ library** - Build the core library first

## Build Steps

### Method 1: Using justfile (Recommended)

From the OpenDAFF root directory:

```bash
# Build C wrapper and Rust bindings
just build-rust

# Test the bindings
just test-rust

# Format Rust code
just fmt-rust

# Run clippy linter
just clippy-rust

# Generate documentation
just docs-rust
```

### Method 2: Manual Build

#### Step 1: Build the C Wrapper

From the OpenDAFF root directory:

```bash
cmake -B build-rust -S . -DOPENDAFF_WITH_RUST_BINDING=ON
cmake --build build-rust -j $(nproc)
```

This creates `libdaffrustwrapper.so` (or `.dylib` on macOS, `.dll` on Windows).

#### Step 2: Build the Rust Crate

```bash
cd bindings/rust
cargo build --release
```

#### Step 3: Run Tests

```bash
cargo test
```

## Build Configuration

### Library Search Paths

The `build.rs` script configures the following library search paths:

1. `../../build` (relative to the Rust bindings directory)
2. `/usr/local/lib`
3. `/usr/lib`

If your libraries are installed elsewhere, you can set:

```bash
export LD_LIBRARY_PATH=/path/to/libs:$LD_LIBRARY_PATH  # Linux
export DYLD_LIBRARY_PATH=/path/to/libs:$DYLD_LIBRARY_PATH  # macOS
```

### Custom CMake Build Directory

If you used a different CMake build directory:

```bash
# Edit build.rs and change the build_dir path, or
# Set library path manually
export LD_LIBRARY_PATH=/path/to/your/build:$LD_LIBRARY_PATH
cd bindings/rust
cargo build --release
```

## Platform-Specific Notes

### Linux

```bash
# Install dependencies
sudo apt install cmake build-essential

# Build
just build-rust

# Set library path for runtime
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
```

### macOS

```bash
# Install dependencies
brew install cmake

# Build
just build-rust

# Set library path for runtime
export DYLD_LIBRARY_PATH=/usr/local/lib:$DYLD_LIBRARY_PATH
```

### Windows

Using MSVC:

```powershell
# Configure CMake with MSVC
cmake -B build-rust -S . -DOPENDAFF_WITH_RUST_BINDING=ON -G "Visual Studio 17 2022"
cmake --build build-rust --config Release

# Build Rust bindings
cd bindings\rust
cargo build --release
```

## Troubleshooting

### Error: Cannot find -ldaffrustwrapper

**Solution**: Build the C wrapper library first:

```bash
cmake -B build-rust -S . -DOPENDAFF_WITH_RUST_BINDING=ON
cmake --build build-rust
```

### Error: Cannot find -lDAFF

**Solution**: Build the core DAFF library:

```bash
cmake -B build -S .
cmake --build build
```

### Error: Library not found at runtime

**Solution**: Set the library path:

```bash
# Linux
export LD_LIBRARY_PATH=/path/to/libs:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=/path/to/libs:$DYLD_LIBRARY_PATH

# Or copy libraries to /usr/local/lib (requires sudo)
sudo cmake --install build-rust
```

### Error: Linking with `cc` failed

**Solution**: Ensure C++ standard library is available:

```bash
# Linux - install libstdc++
sudo apt install libstdc++6

# macOS - ensure Xcode Command Line Tools are installed
xcode-select --install
```

### Warning: Unused imports or variables

These are harmless warnings. To fix them:

```bash
cargo fmt
cargo clippy --fix --allow-dirty
```

## Running Examples

After building:

```bash
# Run basic usage example with a DAFF file
cargo run --example basic_usage -- /path/to/file.daff

# Build and install for system-wide use
cargo install --path .
```

## Development Workflow

```bash
# Check for compilation errors (fast)
cargo check

# Build for development (with debug symbols)
cargo build

# Build for release (optimized)
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_reader_creation

# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Generate and open documentation
cargo doc --open
```

## CI/CD Integration

For continuous integration, use:

```bash
# In your CI script
cmake -B build -S . -DOPENDAFF_WITH_RUST_BINDING=ON
cmake --build build
cd bindings/rust
cargo test --all
cargo clippy -- -D warnings
```

## Installing the Crate

### Local Installation

```bash
cd bindings/rust
cargo install --path .
```

### Using in Other Projects

Add to your `Cargo.toml`:

```toml
[dependencies]
opendaff = { path = "/path/to/opendaff/bindings/rust" }
```

Or if published to crates.io:

```toml
[dependencies]
opendaff = "1.8"
```

## Verifying the Build

```bash
# Check that libraries are found
ldd target/release/libopendaff.so  # Linux
otool -L target/release/libopendaff.dylib  # macOS

# Run tests
cargo test --release

# Check examples compile
cargo build --examples
```

## Clean Build

```bash
# Clean Rust artifacts
cargo clean

# Clean CMake build
rm -rf ../../build-rust

# Clean all
just clean-all
```

## Further Reading

- [Rust Bindings README](README.md)
- [OpenDAFF Main Documentation](../../README.md)
- [File Format Specification](../../FILEFORMAT.md)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
