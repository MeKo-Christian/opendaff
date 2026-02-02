# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

OpenDAFF is a free and open-source software package for directional audio content like directivities of sound sources (loudspeakers, musical instruments) and sound receivers (microphones, HRTFs/HRIRs). It enables exchange, representation, and efficient storage of directional data in the DAFF file format (\*.daff).

**Components:**

- **DAFF**: C++ reader library for DAFF files (no dependencies)
- **DAFFViz**: Visualization library using VTK and Qt
- **DAFFTool**: Command-line utility for file inspection and data extraction
- **DAFFViewer**: GUI application for visualization and data extraction
- Bindings for Matlab, Python, C#, and Go
- Matlab scripts for generating DAFF content

## Build System

The project uses **CMake** for cross-platform builds and **just** for task automation.

### Quick Start with just

The project includes a [justfile](justfile) for common development tasks:

```bash
# Show all available commands with quick start guide
just

# Build the core DAFF library (auto-configures if needed)
just build

# Build with tests and run them
just dev

# Format all code
just fmt

# Build and install to custom location
just install-prefix /usr/local

# Show all CMake options
just show-options
```

**Note:** The `just build` command automatically runs CMake configuration if the build directory doesn't exist, so you can start building immediately without manually running `just configure` first.

### Basic Build (Linux)

```bash
cmake .
make
make install
```

Or using just:

```bash
just configure
just build
just install
```

### Build Configuration

Enable optional components via CMake switches (prefix: `OPENDAFF_*`):

- `OPENDAFF_BUILD_DAFFLIBS_SHARED`: Build as shared library (default: OFF)
- `OPENDAFF_WITH_DAFFVIZ`: Build visualization library (requires VTK)
- `OPENDAFF_BUILD_DAFF_TOOL`: Build DAFFTool (requires FFTW3, SNDFILE)
- `OPENDAFF_BUILD_DAFF_VIEWER`: Build DAFFViewer (requires Qt, VTK, FFTW3, SNDFILE)
- `OPENDAFF_BUILD_DAFF_TESTS`: Build test suite
- `OPENDAFF_BUILD_DAFF_BINDINGS_MATLAB`: Build Matlab mex file
- `OPENDAFF_WITH_CSHARP_BINDING`: Build C# wrapper and bindings
- `OPENDAFF_WITH_PYTHON_BINDING`: Build Python C extension
- `OPENDAFF_WITH_GO_BINDING`: Build Go wrapper and bindings
- `OPENDAFF_BUILD_DAFF_DOCUMENTATION`: Generate Doxygen docs

**Just commands for specific builds:**

```bash
just build-viz        # Build with visualization
just build-tool       # Build DAFFTool
just build-viewer     # Build DAFFViewer
just build-tests      # Build tests
just build-python     # Build Python bindings
just build-go         # Build Go bindings
just build-matlab     # Build Matlab bindings (requires Matlab)
just build-docs       # Generate documentation
```

### Debug/Release Builds

Debug builds use "D" postfix: `DAFFD.lib` / `libDAFFD.so` / `DAFFD.dll`

```bash
just build-debug      # Configure and build debug version
just build-release    # Configure and build release version
```

## Architecture

### Core Library (`DAFF`)

**Headers:** [include/](include/)

- [DAFF.h](include/DAFF.h): Main header including all interfaces
- [DAFFReader.h](include/DAFFReader.h): Primary interface for reading DAFF files
- [DAFFContent.h](include/DAFFContent.h): Base interface for content data
- [DAFFMetadata.h](include/DAFFMetadata.h): Metadata interface
- [DAFFProperties.h](include/DAFFProperties.h): File properties interface
- [DAFFDefs.h](include/DAFFDefs.h): Constants and type definitions
- [DAFFUtils.h](include/DAFFUtils.h): Utility functions
- [DAFFSCTransform.h](include/DAFFSCTransform.h): Coordinate transformations

**Implementation:** [src/](src/)

- [DAFFReaderImpl.cpp](src/DAFFReaderImpl.cpp): Main reader implementation
- [DAFFMetadataImpl.cpp](src/DAFFMetadataImpl.cpp): Metadata handling
- [DAFFHeader.h](src/DAFFHeader.h): Internal file header structures

### Content Types

DAFF supports five content types (see [FILEFORMAT.md](FILEFORMAT.md)):

- **IR**: Impulse responses ([DAFFContentIR.h](include/DAFFContentIR.h))
- **MS**: Magnitude spectrum ([DAFFContentMS.h](include/DAFFContentMS.h))
- **PS**: Phase spectrum ([DAFFContentPS.h](include/DAFFContentPS.h))
- **MPS**: Magnitude-phase spectrum ([DAFFContentMPS.h](include/DAFFContentMPS.h))
- **DFT**: Discrete Fourier Transform coefficients ([DAFFContentDFT.h](include/DAFFContentDFT.h))

### Coordinate System

**OpenGL Cartesian system:**

- View direction: -Z axis
- Up direction: +Y axis
- Right-handed yaw-pitch-roll

**User/Object View (spherical):**

- Front: (phi, theta) = (0, 0)
- Up: (0, π/2)
- Phi: azimuthal rotation around +Y, range [0, 2π)
- Theta: elevation angle, range [-π/2, π/2]

**Data View (internal):**

- For equi-angular grids only
- Start: south pole (alpha, beta) = (0, 0)
- End: north pole (0, π)

### Visualization Library (`DAFFViz`)

**Location:** [include/daffviz/](include/daffviz/), [src/daffviz/](src/daffviz/)

**Key components:**

- `DAFFVizBalloonPlot`: 3D directivity visualization
- `DAFFVizCarpetPlot`: 2D carpet plots
- `DAFFVizSphere`, `DAFFVizGrid`, `DAFFVizArrow`: Scene graph elements
- Coordinate assistants for Cartesian/Spherical systems

**Dependencies:** VTK (with Qt Widgets)

### Applications

**DAFFTool:** [apps/tool/](apps/tool/)

- Command-line utility for metadata inspection and data extraction
- Dependencies: DAFF, FFTW3, SNDFILE

**DAFFViewer:** [apps/viewer/](apps/viewer/)

- Qt-based GUI application
- Uses DAFFViz for 3D/2D visualization
- Dependencies: DAFF, DAFFViz, Qt, VTK, FFTW3, SNDFILE

### Bindings

**Matlab:** [bindings/matlab/](bindings/matlab/)

- Build: Run [build_Matlab_DAFF.m](bindings/matlab/build_Matlab_DAFF.m) in Matlab
- Creates platform-specific mex file
- Writer scripts: [matlab/](matlab/) directory (e.g., [daffv17_write.m](matlab/daffv17_write.m))

**Python:** [bindings/python/](bindings/python/)

- Build: `python setup.py build` (compile) or `python setup_with_lib.py build` (link)
- Usage: Import [DAFF.py](python/DAFF.py) wrapper class
- Example: [python/daff_example.py](bindings/python/daff_example.py), [python/DAFF_example.ipynb](python/DAFF_example.ipynb)

**C#:** [bindings/csharp/](bindings/csharp/)

- Wrapper DLL + C# DAFF class
- Test project: [DAFFTest.csproj](bindings/csharp/DAFFTest.csproj)

**Go:** [bindings/go/](bindings/go/)

- Build: `just build-go` or manually with CMake + `go build`
- Uses CGO to call C wrapper around C++ library
- Idiomatic Go API with proper error handling
- Example: [bindings/go/daff_test.go](bindings/go/daff_test.go)
- Full documentation: [bindings/go/README.md](bindings/go/README.md)

## Testing

**Location:** [tests/](tests/)

Test subdirectories (enable with `OPENDAFF_BUILD_DAFF_TESTS`):

- `verification`: Core DAFF functionality tests
- `deserializertest`: File reading tests
- `daffviztest`: DAFFViz library tests
- `qttest`, `qtvtktest`, `qtdaffviztest`: Qt/VTK integration tests
- `tryout`: Experimental/development tests

## Dependencies

**Core Library (DAFF):** None

**Optional:**

- **FFTW3**: For IR↔DFT transformations (DAFFTransformerIR2DFT)
- **SNDFILE**: Audio file I/O (DAFFTool, DAFFViewer)
- **Qt5**: GUI (DAFFViewer, Components: Core, Widgets, Gui, Sql, Svg)
- **VTK** (with Qt Widgets): 3D visualization (DAFFViz, DAFFViewer)
- **Doxygen**: Documentation generation
- **Matlab**: Mex bindings

### CMake Package Resolution

If CMake can't find dependencies:

1. Set `<PACKAGE>_DIR` variables (e.g., `VTK_DIR` to `<vtk-install>/lib/cmake/vtk-7.0`)
2. Modify search paths in [cmake/](cmake/) `Find*.cmake` modules

## File Format

See [FILEFORMAT.md](FILEFORMAT.md) for complete binary format specification.

**Structure:**

1. File header (signature "FW", version 105)
2. File block table
3. Main header (content type, quantization, channels, records, orientation)
4. Content-specific header (varies by type: IR/MS/PS/MPS/DFT)
5. Record descriptor block (channel data offsets)
6. Binary data block
7. Metadata block

## Code Formatting

The project uses consistent code formatting enforced by configuration files.

### Automated Formatting

Use [treefmt](https://github.com/numtide/treefmt) for multi-language formatting:

```bash
just fmt              # Format all code
just check-formatted  # Check if code is formatted
```

### Formatters by Language

**C/C++:** `clang-format` (configured in [.clang-format](.clang-format))

- Style: Based on LLVM with 4-space tabs
- Line width: 120 characters
- Brace style: Functions on new line, control structures on same line

**CMake:** `cmake-format` (configured in [.cmake-format.yaml](.cmake-format.yaml))

- Line width: 100 characters
- Indent: 4 spaces

**Python:** `black` + `isort`

- Line width: 88 characters (black default)
- Import sorting with isort

**Markdown/YAML/JSON:** `prettier`

### Editor Configuration

[.editorconfig](.editorconfig) provides cross-editor settings:

- C/C++: Tabs (width 4)
- CMake/Python/C#: Spaces (4 spaces)
- Consistent line endings (LF on Unix, CRLF for .bat files)

## GitHub Organization

- Username: MeKo-Christian
- Organization: MeKo-Tech
