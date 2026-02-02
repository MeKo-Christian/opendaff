# OpenDAFF - A free, open source software package for directional audio data

[![CI](https://github.com/MeKo-Tech/opendaff/actions/workflows/ci.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/ci.yml)
[![Bindings](https://github.com/MeKo-Tech/opendaff/actions/workflows/bindings.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/bindings.yml)
[![Release](https://github.com/MeKo-Tech/opendaff/actions/workflows/release.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/release.yml)
[![Documentation](https://github.com/MeKo-Tech/opendaff/actions/workflows/docs.yml/badge.svg)](https://github.com/MeKo-Tech/opendaff/actions/workflows/docs.yml)

> **Note:** This is a fork of the original OpenDAFF repository maintained by the Institute of Technical Acoustics (ITA) at RWTH Aachen University. The original repository can be found at: [https://git.rwth-aachen.de/ita/OpenDAFF](https://git.rwth-aachen.de/ita/OpenDAFF)

OpenDAFF is a free and open source software package for directional audio content like directivities of sound sources (e.g. loudspeakers, musical instruments) and sound receivers (e.g. microphones, head-related transfer functions, HRIRs/HRTFs).
OpenDAFF enables a simple exchange, representation and efficient storage of such directional representations in form of a single DAFF file (\*.DAFF).
Under the term software package, OpenDAFF unites the DAFF file format conventions, a C++ software library for reading DAFF files, a C++ visualization library using VTK and QtWidgets and extensive Matlab scripts for writing DAFF files.
The package is complemented by tools like a graphical viewer application (called DAFFViewer) and a command-line utility (called DAFFTool).
Visit the OpenDAFF homepage: [http://www.opendaff.org](http://www.opendaff.org)

## License

Copyright 2016 - 2018 Institute of Technical Acoustics, RWTH Aachen University

Licensed under the Apache License, Version 2.0 (the "License");
you may not use the OpenDAFF software package except in compliance with the License.
You may obtain a copy of the License at

<http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

You should also have received a [copy of the License](LICENSE.md) with the OpenDAFF software package.

## Quick build guide for Linux developers

OpenDAFF uses CMake to generate project files for your desired development environment and platform. Install a current version of CMake according to the information found on [http://www.cmake.org](http://www.cmake.org)
In the source folder of OpenDAFF that contains the CMakeLists.txt, run

´´´sh
cmake .
make
make install
´´´

Now, a copy of the OpenDAFF static library _DAFF_ has been installed on your system. To activate further optional parts of OpenDAFF, see the OPENDAFF\_\* switches within the CMake configuration.

## OpenDAFF content

OpenDAFF comprises ...

1. two C++ programming libraries
   - _DAFF_: reader interface for DAFF files
   - _DAFFViz_: DAFF content visualization
2. two PC applications
   - _DAFFTool_: a command line tool to investigate file contents (metadata) and extract directional data
   - _DAFFViewer_: a graphical application to visualize DAFF directivities, show metadata and extract directional data
3. language bindings for the C++ library
   - _Matlab_: Matlab executable (mex) to read DAFF files
   - _Python_: Python C extension module with Pythonic wrapper
   - _C#_: C wrapper DLL with C# class interface
   - _Go_: CGO bindings with idiomatic Go API
4. a collection of Matlab scripts to generate or assemble DAFF content
5. some tests for validation.

## OpenDAFF dependencies

There is no dependency required for the DAFF library itself, however the additional visualization library
DAFFViz and the provided applications require third party projects. To build the entire OpenDAFF project,
the following is required:

- FFTW3
- SNDFILE
- Qt
- VTK with Qt Widgets
- Doxygen
- Matlab

## General build guide

To build all parts of the OpenDAFF project, the corresponding switches have to be activated in CMake. Have a look at the _OPENDAFF_ group or variables with this prefix in the configuration list.

OpenDAFF uses a "D" postfix to distinguish between libraries and binaries that are build with debug information, i.e. the DAFFD.lib (or DAFFD.dll or libDAFFD.so) refers to the debug version.

To find external packages, a common way is to first activate everything you desire to build in CMake. Hit configure. Then, when CMake looks for the dependencies, it will come up with errors if a package could not be found (vagely spoken: are not installed on your computer). Here, one can help and set the pathes that appear in CMake called _PACKAGE_\_DIR, for example if you downloaded and extracted a package to the file system. Also, have a look at the FindXXX.cmake in the OpenDAFF/cmake folder to extend the search paths for CMake.

### Dependency overview

- DAFFViz
  - DAFF
  - VTK
- DAFFTool
  - DAFF
  - FFTW3
  - SNDFILE
- DAFFViewer
  - DAFF
  - DAFFViz
  - FFTW3
  - SNDFILE
  - Qt
  - VTK with Qt Widgets

You may have to set the VTK_DIR appropriately to tell CMake where to look for the VTKConfig.cmake file.
The DAFFTool requires the FFTW3 library and libsndfile library. Again, you may have to tell CMake where to find the package.
The DAFFViewer also requires the Qt and VTK.

## Build guide for Visual Studio users on Windows

### Prerequisites

OpenDAFF uses CMake to generate project files for your desired development environment and platform.
Install a current version of CMake according to the information found on [http://www.cmake.org](http://www.cmake.org)

Let's assume you have donwloaded the source code of OpenDAFF to the folder _D:/dev/OpenDAFF_ and this README along with a file named CMakeLists.txt can be found here.

Start the CMake GUI and enter or browse to your source code folder: _D:/dev/OpenDAFF_
Now, provide a place where the project files shall be genereted, say _D:/dev/OpenDAFF/build/win32-x64.vc12_ to indicate platform (x64 in this case) and compiler version (Visual C++ Compiler version 12).
Hit the Configure button and select the desired Visual Studio compiler, build platform and environment version (i.e. Visual Studio 12 2013 Win64).
For starters, change the variable CMAKE*INSTALL_PREFIX to something like \_D:/dev/OpenDAFF/dist* to prevent installation of OpenDAFF into the system's installation path (where you probably don't have write access). Chose for example something like _D:/dev/OpenDAFF/dist/win32-x64.vc12_.
Now you can generate the project files and you will find the OpenDAFF.sln solution file in the target folder _D:/dev/OpenDAFF/build/win32-x64.vc12_, which you can opened with Visual Studio.

### Building DAFF library

In Visual Studio, you can now build the entire solution with Debug or Release mode. It will automatically install the headers and library into the CMAKE_INSTALL_PREFIX folder, if you have write permissions there.

### Building DAFFViz library

To build the visualization library _DAFFViz_, you will have to first build VTK with Qt support (QtWidgets). Find appropriate build/installation tutorials online.
Let's assume you have used CMake to build and install VTK-7.0 (from sources) into the folder _D:/dev/VTK/dist/win32-x64.vc12_. To tell CMake within the OpenDAFF generation process the required location, provide the path to the CMake-related files generated by VTK, which is - in this case example - located here: _D:/dev/VTK/dist/win32-x64.vc12/lib/cmake/vtk-7.0_
To do so, set the variable VTK*DIR to the respective folder \_D:/dev/VTK/dist/win32-x64.vc12/lib/cmake/vtk-7.0*, and CMake will resolve the dependencies automatically.

You can now activate the switch OPENDAFF*BUILD_DAFFVIZ to generate the DAFF visualization library \_DAFFViz*. Configure, Generate. Your Visual Studio project will now include this library.

If you want to build the applications, namely the DAFFViewer and the DAFFTool, you will have to resolve further third party dependencies, as stated above. You can find Windows binary packages on the websites of the projects. Again, tell CMake where to find the packages inside the CMake GUI window.
You can then activate the switches for the applications. Configure, Generate. Your Visual Studio project will now include the applications.

### Language bindings

#### Matlab binding

To generate the Matlab executable (mex) binary, it is easiest to use your preferred compiler within Matlab. You can execute the build_Matlab_DAFF.m script in bindings/matlab, which will generate a _DAFF_ mex file.

#### Python binding

Python bindings are provided in `bindings/python/`. Build with:

```sh
cd bindings/python
python setup.py build
python setup.py install
```

#### C# binding

C# bindings include a C wrapper DLL and C# class. Enable with `OPENDAFF_WITH_CSHARP_BINDING=ON` in CMake.

#### Go binding

Go bindings use CGO to interface with the C++ library through a C wrapper. Build with:

```sh
just build-go
# Or manually
cmake -DOPENDAFF_WITH_GO_BINDING=ON .
make
cd bindings/go && go build
```

For detailed Go documentation, see [bindings/go/README.md](bindings/go/README.md).
