# OpenDAFF Go Bindings

Go bindings for the OpenDAFF library, providing idiomatic Go access to directional audio data in DAFF format.

## Overview

OpenDAFF is a software package for directional audio content like directivities of sound sources (loudspeakers, musical instruments) and sound receivers (microphones, HRTFs/HRIRs). These Go bindings provide a native Go interface to read and process DAFF files.

## Features

- **Idiomatic Go API** with proper error handling and resource management
- **Full content type support**: IR, MS, PS, MPS, and DFT
- **Zero-copy data access** where possible
- **Automatic resource cleanup** with finalizers (but explicit `Close()` recommended)
- **Type-safe** enums and constants

## Installation

### Prerequisites

1. **OpenDAFF C++ library**: Build and install the core DAFF library first
2. **Go 1.21+**: Required for the bindings
3. **C++ compiler**: For CGO compilation (gcc or clang)

### Building the C Wrapper

From the OpenDAFF root directory:

```bash
# Using justfile (recommended)
just build-go

# Or using CMake directly
cmake -DOPENDAFF_WITH_GO_BINDING=ON .
make
make install
```

### Installing the Go Package

```bash
cd bindings/go
go build
go install
```

Or use it in your project:

```bash
go get github.com/MeKo-Tech/opendaff-go
```

## Quick Start

```go
package main

import (
    "fmt"
    "log"

    "github.com/MeKo-Tech/opendaff-go"
)

func main() {
    // Create a new reader
    reader, err := daff.NewReader()
    if err != nil {
        log.Fatal(err)
    }
    defer reader.Close()

    // Open a DAFF file
    if err := reader.OpenFile("path/to/file.daff"); err != nil {
        log.Fatal(err)
    }
    defer reader.CloseFile()

    // Get file properties
    contentType := reader.GetContentType()
    numChannels := reader.GetNumChannels()
    numRecords := reader.GetNumRecords()

    fmt.Printf("Content Type: %s\n", contentType)
    fmt.Printf("Channels: %d\n", numChannels)
    fmt.Printf("Records: %d\n", numRecords)

    // Process based on content type
    switch contentType {
    case daff.ContentTypeIR:
        processImpulseResponse(reader)
    case daff.ContentTypeMS:
        processMagnitudeSpectrum(reader)
    // ... handle other types
    }
}

func processImpulseResponse(reader *daff.Reader) error {
    ir, err := reader.GetContentIR()
    if err != nil {
        return err
    }

    filterLength := ir.GetFilterLength()
    samplerate := ir.GetSamplerate()
    fmt.Printf("Filter Length: %d samples at %d Hz\n", filterLength, samplerate)

    // Get impulse response for front direction (phi=0, theta=0)
    recordIndex := ir.GetNearestNeighbour(0, 0)

    // Get filter coefficients for left channel
    coeffs, err := ir.GetFilterCoeffs(recordIndex, 0)
    if err != nil {
        return err
    }

    fmt.Printf("Retrieved %d filter coefficients\n", len(coeffs))
    return nil
}
```

## API Overview

### Reader

```go
type Reader struct { /* ... */ }

func NewReader() (*Reader, error)
func (r *Reader) Close() error
func (r *Reader) OpenFile(filename string) error
func (r *Reader) CloseFile()
func (r *Reader) IsValid() bool

// File properties
func (r *Reader) GetContentType() ContentType
func (r *Reader) GetQuantization() Quantization
func (r *Reader) GetNumChannels() int
func (r *Reader) GetNumRecords() int
func (r *Reader) GetAlphaResolution() float32
func (r *Reader) GetBetaResolution() float32
func (r *Reader) GetOrientation() (yaw, pitch, roll float32, err error)

// Metadata
func (r *Reader) HasMetadata(key string) bool
func (r *Reader) GetMetadataString(key string) (string, error)
func (r *Reader) GetMetadataFloat(key string) (float32, error)
func (r *Reader) GetMetadataBool(key string) (bool, error)
```

### Content Types

#### Impulse Response (IR)

```go
type ContentIR struct { /* ... */ }

func (r *Reader) GetContentIR() (*ContentIR, error)
func (c *ContentIR) GetFilterLength() int
func (c *ContentIR) GetSamplerate() int
func (c *ContentIR) GetNearestNeighbour(phi, theta float64) int
func (c *ContentIR) GetRecordCoords(recordIndex int) (alpha, beta float64, err error)
func (c *ContentIR) GetFilterCoeffs(recordIndex, channel int) ([]float32, error)
```

#### Magnitude Spectrum (MS)

```go
type ContentMS struct { /* ... */ }

func (r *Reader) GetContentMS() (*ContentMS, error)
func (c *ContentMS) GetNumFrequencies() int
func (c *ContentMS) GetNearestNeighbour(phi, theta float64) int
func (c *ContentMS) GetMagnitudes(recordIndex, channel int) ([]float32, error)
```

#### Phase Spectrum (PS)

```go
type ContentPS struct { /* ... */ }

func (r *Reader) GetContentPS() (*ContentPS, error)
func (c *ContentPS) GetNumFrequencies() int
func (c *ContentPS) GetPhases(recordIndex, channel int) ([]float32, error)
```

#### Magnitude-Phase Spectrum (MPS)

```go
type ContentMPS struct { /* ... */ }

func (r *Reader) GetContentMPS() (*ContentMPS, error)
func (c *ContentMPS) GetNumFrequencies() int
func (c *ContentMPS) GetCoefficients(recordIndex, channel int) (magnitudes, phases []float32, err error)
```

#### DFT Coefficients

```go
type ContentDFT struct { /* ... */ }

func (r *Reader) GetContentDFT() (*ContentDFT, error)
func (c *ContentDFT) GetNumDFTCoeffs() int
func (c *ContentDFT) IsSymmetric() bool
func (c *ContentDFT) GetDFTCoeffs(recordIndex, channel int) ([]float32, error)  // Interleaved real/imag
```

## Coordinate System

OpenDAFF uses the OpenGL coordinate system with spherical views:

- **Object View (phi, theta)**: User-facing directional queries
  - `phi`: Azimuth angle (rotation around Y axis), range [0, 2π)
  - `theta`: Elevation angle, range [-π/2, π/2]
  - Front: (0, 0), Up: (0, π/2)

- **Data View (alpha, beta)**: Internal data storage
  - Used by content implementations
  - Automatically converted from object view

All angles in the API are in **radians**.

## Error Handling

All functions that can fail return an `error` as the last return value. Always check errors:

```go
ir, err := reader.GetContentIR()
if err != nil {
    log.Fatalf("Failed to get IR content: %v", err)
}
```

Common errors:

- File not found or cannot be opened
- Invalid file format
- Wrong content type requested
- Buffer size mismatch
- Invalid record or channel index

## Memory Management

- Call `reader.Close()` when done to release C++ resources
- Use `defer reader.Close()` immediately after creation
- Finalizers provide automatic cleanup but are not guaranteed to run immediately
- Slices returned by `Get*` methods are owned by Go and will be garbage collected

## Testing

Run the test suite:

```bash
cd bindings/go
go test -v
```

For integration tests with actual DAFF files, place test files in `testdata/` and uncomment the integration tests in `daff_test.go`.

## Examples

See `daff_test.go` for complete examples of:

- Opening and reading DAFF files
- Handling different content types
- Querying directional data
- Accessing metadata

## Building with CGO

The bindings use CGO to interface with the C++ library. Required CGO flags are set in `daff.go`:

```go
// #cgo CXXFLAGS: -I../../include -std=c++11
// #cgo LDFLAGS: -L../../build -lDAFF -lstdc++
```

You may need to adjust these paths based on your OpenDAFF installation:

```bash
# Set library path for runtime
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# Or on macOS
export DYLD_LIBRARY_PATH=/usr/local/lib:$DYLD_LIBRARY_PATH
```

## Cross-Compilation

CGO complicates cross-compilation. For building on a different platform:

1. Install a cross-compiler toolchain
2. Set `CGO_ENABLED=1`
3. Set appropriate `CC` and `CXX` variables
4. Ensure the OpenDAFF library is built for the target platform

## Performance Notes

- Data retrieval functions like `GetFilterCoeffs()` allocate new Go slices
- For performance-critical loops, reuse buffers when possible
- The C++ library handles thread-safety for read operations

## Contributing

Contributions are welcome! Please ensure:

- Code follows Go conventions (`gofmt`, `golint`)
- Tests pass and new features include tests
- Documentation is updated

## License

OpenDAFF is distributed under the Apache License Version 2.0.

Copyright 2016-2018 Institute of Technical Acoustics (ITA), RWTH Aachen University

## Links

- [OpenDAFF Project](https://www.opendaff.org)
- [GitHub Repository](https://github.com/MeKo-Tech/opendaff)
- [File Format Specification](../../FILEFORMAT.md)

## Support

For issues and questions:

- Open an issue on GitHub
- Consult the main OpenDAFF documentation
- Check the examples in `daff_test.go`
