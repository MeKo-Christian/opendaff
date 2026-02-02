# Rust bindings for OpenDAFF

cmake_minimum_required(VERSION 3.10)

# Build the C wrapper library for Rust bindings
add_library(DAFFRustWrapper SHARED
    daff_rust_wrapper.cpp
    daff_rust_wrapper.h
)

target_link_libraries(DAFFRustWrapper
    DAFF
)

target_include_directories(DAFFRustWrapper PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/include
)

set_target_properties(DAFFRustWrapper PROPERTIES
    OUTPUT_NAME "daffrustwrapper"
    VERSION ${OPENDAFF_VERSION}
    SOVERSION ${OPENDAFF_VERSION_MAJOR}
)

# Install the wrapper library
install(TARGETS DAFFRustWrapper
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

# Install the header
install(FILES daff_rust_wrapper.h
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/DAFF
)

# The actual Rust build is handled by Cargo outside of CMake
# Users should run: cd bindings/rust && cargo build --release
# Or use the justfile command: just build-rust

message(STATUS "Rust bindings C wrapper will be built")
message(STATUS "To build the Rust crate, run: cd bindings/rust && cargo build --release")
