# GitHub Actions Workflows

This directory contains CI/CD workflows for the OpenDAFF project.

## Workflows

### CI (ci.yml)
[![CI](https://github.com/MeKo-Tech/opendaff/actions/workflows/ci.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/ci.yml)

Main continuous integration workflow that runs on every push and pull request:

- **Formatting Check**: Validates code formatting with treefmt
- **Build & Test Matrix**: Builds and tests on Ubuntu, macOS, and Windows
- **Build with Visualization**: Tests building DAFFViz on Linux and macOS
- **Python Bindings**: Tests Python bindings on all platforms with Python 3.8-3.12
- **Can Build Tools**: Verifies DAFFTool and DAFFViewer can be built

### Bindings (bindings.yml)
[![Bindings](https://github.com/MeKo-Tech/opendaff/actions/workflows/bindings.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/bindings.yml)

Specialized testing for language bindings:

- **Python Advanced**: Tests Python bindings with numpy and jupyter
- **C#**: Builds and tests C# wrapper on Windows
- **MATLAB**: Builds MEX files using MATLAB Actions
- **MATLAB Manual**: Verifies MATLAB binding source files exist

### Release (release.yml)
[![Release](https://github.com/MeKo-Tech/opendaff/actions/workflows/release.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/release.yml)

Creates release builds for all platforms:

- Triggered on version tags (v*.*.*)
- Builds optimized binaries for Linux, macOS, and Windows
- Runs tests before packaging
- Creates GitHub releases with downloadable artifacts

### Documentation (docs.yml)
[![Documentation](https://github.com/MeKo-Tech/opendaff/actions/workflows/docs.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/docs.yml)

Generates and deploys Doxygen documentation:

- Builds Doxygen HTML documentation
- Uploads documentation artifacts
- Deploys to GitHub Pages on master branch

## Local Testing

You can test workflows locally using [act](https://github.com/nektos/act):

```bash
# Install act
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run formatting check
act -j formatting

# Run build matrix (specific OS)
act -j build-and-test --matrix os:ubuntu-latest

# Run Python bindings test
act -j python-bindings
```

## Status Badges

Add these to your main README.md:

```markdown
[![CI](https://github.com/MeKo-Tech/opendaff/actions/workflows/ci.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/ci.yml)
[![Bindings](https://github.com/MeKo-Tech/opendaff/actions/workflows/bindings.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/bindings.yml)
[![Release](https://github.com/MeKo-Tech/opendaff/actions/workflows/release.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/release.yml)
[![Documentation](https://github.com/MeKo-Tech/opendaff/actions/workflows/docs.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/docs.yml)
```

## Workflow Dependencies

### Core CI Requirements
- **Ubuntu**: cmake, build-essential, libfftw3-dev, libsndfile1-dev
- **macOS**: cmake, fftw, libsndfile
- **Windows**: cmake (via Chocolatey)

### Visualization Requirements
- **Ubuntu**: qtbase5-dev, qttools5-dev, libvtk9-dev
- **macOS**: qt@5, vtk

### Python Bindings
- Python 3.8+ with setuptools and wheel

### C# Bindings
- .NET SDK 8.0
- MSBuild (Windows)

### MATLAB Bindings
- MATLAB R2023b or later

### Documentation
- doxygen
- graphviz

## Troubleshooting

### Formatting Failures
If the formatting check fails, run locally:
```bash
just fmt  # Auto-format all files
```

### Build Failures
Check the specific job logs in GitHub Actions. Common issues:
- Missing dependencies (see workflow file for required packages)
- CMake configuration errors (check CMakeLists.txt)
- Test failures (see test output in logs)

### Binding Build Failures
- **Python**: Ensure Python version compatibility
- **C#**: Windows-only, requires .NET SDK
- **MATLAB**: Requires MATLAB installation or MATLAB Actions setup

## Contributing

When adding new features:
1. Ensure formatting passes (`just fmt`)
2. Add tests if applicable
3. Update CMake configuration if needed
4. Verify all matrix builds pass on your PR
