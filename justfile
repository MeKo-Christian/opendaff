# OpenDAFF justfile
# Development automation for OpenDAFF directional audio toolkit

set shell := ["bash", "-uc"]

# Default recipe - show available commands
default:
    @echo "OpenDAFF - Directional Audio File Format"
    @echo "=========================================="
    @echo ""
    @echo "Quick start:"
    @echo "  just build          # Build the core DAFF library"
    @echo "  just build-tests    # Build and run tests"
    @echo "  just dev            # Quick development build with tests"
    @echo "  just fmt            # Format all code"
    @echo ""
    @echo "All available commands:"
    @just --list

# Note: Install formatting tools manually
# clang-format: apt install clang-format (Linux) or brew install clang-format (macOS)
# treefmt: Download from https://github.com/numtide/treefmt/releases
# cmake-format: pip install cmake-format
# prettier: npm install -g prettier
# black: pip install black
# isort: pip install isort

# Format all code using treefmt
fmt:
    treefmt --allow-missing-formatter

# Check if code is formatted correctly
check-formatted:
    treefmt --allow-missing-formatter --fail-on-change

# Configure CMake build (basic DAFF library only)
configure BUILD_DIR="build":
    cmake -B {{ BUILD_DIR }} -S .

# Configure with all optional components enabled
configure-full BUILD_DIR="build":
    cmake -B {{ BUILD_DIR }} -S . \
        -DOPENDAFF_WITH_DAFFVIZ=ON \
        -DOPENDAFF_BUILD_DAFF_TOOL=ON \
        -DOPENDAFF_BUILD_DAFF_VIEWER=ON \
        -DOPENDAFF_BUILD_DAFF_TESTS=ON \
        -DOPENDAFF_BUILD_DAFF_BINDINGS_MATLAB=OFF \
        -DOPENDAFF_WITH_CSHARP_BINDING=ON \
        -DOPENDAFF_WITH_PYTHON_BINDING=ON \
        -DOPENDAFF_BUILD_DAFF_DOCUMENTATION=OFF

# Configure debug build
configure-debug BUILD_DIR="build-debug":
    cmake -B {{ BUILD_DIR }} -S . -DCMAKE_BUILD_TYPE=Debug

# Configure release build
configure-release BUILD_DIR="build-release":
    cmake -B {{ BUILD_DIR }} -S . -DCMAKE_BUILD_TYPE=Release

# Build the project (auto-configures if needed)
build BUILD_DIR="build":
    #!/usr/bin/env bash
    if [ ! -d "{{ BUILD_DIR }}" ]; then
        echo "Build directory {{ BUILD_DIR }} doesn't exist, configuring..."
        just configure {{ BUILD_DIR }}
    fi
    cmake --build {{ BUILD_DIR }} -j $(nproc)

# Build debug version
build-debug: (configure-debug "build-debug")
    @just build build-debug

# Build release version
build-release: (configure-release "build-release")
    @just build build-release

# Install to system or CMAKE_INSTALL_PREFIX
install BUILD_DIR="build":
    cmake --install {{ BUILD_DIR }}

# Install to custom prefix
install-prefix PREFIX BUILD_DIR="build":
    cmake -B {{ BUILD_DIR }} -S . -DCMAKE_INSTALL_PREFIX={{ PREFIX }}
    cmake --build {{ BUILD_DIR }}
    cmake --install {{ BUILD_DIR }}

# Clean build artifacts
clean BUILD_DIR="build":
    rm -rf {{ BUILD_DIR }}

# Clean all build directories
clean-all:
    rm -rf build build-* dist

# Build just the DAFF library (no optional components)
build-lib: configure
    @just build

# Build with visualization support
build-viz:
    cmake -B build-viz -S . -DOPENDAFF_WITH_DAFFVIZ=ON
    @just build build-viz

# Build DAFFTool only
build-tool:
    cmake -B build-tool -S . -DOPENDAFF_BUILD_DAFF_TOOL=ON
    @just build build-tool

# Build DAFFViewer only
build-viewer:
    cmake -B build-viewer -S . \
        -DOPENDAFF_WITH_DAFFVIZ=ON \
        -DOPENDAFF_BUILD_DAFF_VIEWER=ON
    @just build build-viewer

# Build tests
build-tests:
    cmake -B build-tests -S . -DOPENDAFF_BUILD_DAFF_TESTS=ON
    @just build build-tests

# Run tests (auto-builds if needed)
test BUILD_DIR="build-tests":
    #!/usr/bin/env bash
    if [ ! -d "{{ BUILD_DIR }}" ]; then
        echo "Tests not built yet, building..."
        just build-tests
    fi
    ctest --test-dir {{ BUILD_DIR }} --output-on-failure

# Build Python bindings
build-python:
    cd bindings/python && python setup.py build

# Install Python bindings
install-python:
    cd bindings/python && python setup.py install

# Build Python bindings (linking against existing DAFF library)
build-python-lib:
    cd bindings/python && python setup_with_lib.py build

# Build Go bindings (C wrapper via CMake + Go module)
build-go BUILD_DIR="build-go":
    cmake -B {{ BUILD_DIR }} -S . -DOPENDAFF_WITH_GO_BINDING=ON
    @just build {{ BUILD_DIR }}
    cd bindings/go && go build

# Test Go bindings
test-go:
    cd bindings/go && go test -v

# Install Go module locally
install-go:
    cd bindings/go && go install

# Format Go code
fmt-go:
    cd bindings/go && gofmt -w .

# Build Matlab bindings (requires Matlab with mex compiler)
build-matlab:
    #!/usr/bin/env bash
    cd bindings/matlab
    matlab -batch "run('build_Matlab_DAFF.m')"

# Generate Doxygen documentation
build-docs:
    cmake -B build-docs -S . -DOPENDAFF_BUILD_DAFF_DOCUMENTATION=ON
    @just build build-docs

# Open generated documentation in browser
view-docs BUILD_DIR="build-docs":
    xdg-open {{ BUILD_DIR }}/doc/html/index.html 2>/dev/null || open {{ BUILD_DIR }}/doc/html/index.html

# Build C# bindings
build-csharp BUILD_DIR="build-csharp":
    cmake -B {{ BUILD_DIR }} -S . -DOPENDAFF_WITH_CSHARP_BINDING=ON
    @just build {{ BUILD_DIR }}

# Run C# test
test-csharp BUILD_DIR="build-csharp":
    cd {{ BUILD_DIR }}/csharp && dotnet run --project DAFFTest.csproj

# Format C++ code with clang-format
fmt-cpp:
    find include src apps bindings/csharp bindings/python -name "*.cpp" -o -name "*.h" -o -name "*.c" | xargs clang-format -i

# Format CMake files
fmt-cmake:
    find . -name "CMakeLists.txt" -o -name "*.cmake" | xargs cmake-format -i

# Format Python files
fmt-python:
    isort bindings/python python
    black bindings/python python

# Check Python formatting
check-python:
    isort --check-only bindings/python python
    black --check bindings/python python

# Run all formatters
fmt-all: fmt

# Display project statistics
stats:
    @echo "=== OpenDAFF Project Statistics ==="
    @echo "C++ Headers:"
    @find include src apps -name "*.h" | wc -l
    @echo "C++ Sources:"
    @find src apps bindings -name "*.cpp" | wc -l
    @echo "C Files:"
    @find . -name "*.c" | wc -l
    @echo "CMake Files:"
    @find . -name "CMakeLists.txt" -o -name "*.cmake" | wc -l
    @echo "Python Files:"
    @find bindings/python python -name "*.py" 2>/dev/null | wc -l
    @echo "Matlab Files:"
    @find matlab bindings/matlab -name "*.m" 2>/dev/null | wc -l
    @echo "Total Lines of Code:"
    @find include src apps bindings -name "*.cpp" -o -name "*.h" -o -name "*.c" | xargs wc -l | tail -1

# Quick development build (DAFF + tests)
dev: configure-debug build-tests test

# Full CI-style check
ci: check-formatted build-tests test

# Show CMake configuration options
show-options:
    @echo "=== OpenDAFF CMake Options ==="
    @echo "OPENDAFF_BUILD_DAFFLIBS_SHARED    - Build as shared library"
    @echo "OPENDAFF_WITH_DAFFVIZ              - Build visualization library"
    @echo "OPENDAFF_BUILD_DAFF_TOOL           - Build DAFFTool CLI"
    @echo "OPENDAFF_BUILD_DAFF_VIEWER         - Build DAFFViewer GUI"
    @echo "OPENDAFF_BUILD_DAFF_TESTS          - Build test suite"
    @echo "OPENDAFF_BUILD_DAFF_BINDINGS_MATLAB - Build Matlab bindings"
    @echo "OPENDAFF_WITH_CSHARP_BINDING       - Build C# bindings"
    @echo "OPENDAFF_WITH_PYTHON_BINDING       - Build Python bindings"
    @echo "OPENDAFF_WITH_GO_BINDING           - Build Go bindings"
    @echo "OPENDAFF_BUILD_DAFF_DOCUMENTATION  - Generate Doxygen docs"

# Example: Run DAFFTool on a sample file
run-dafftool FILE BUILD_DIR="build-tool":
    {{ BUILD_DIR }}/apps/tool/DAFFTool {{ FILE }}

# Example: Run DAFFViewer
run-viewer FILE="" BUILD_DIR="build-viewer":
    {{ BUILD_DIR }}/apps/viewer/DAFFViewer {{ FILE }}
