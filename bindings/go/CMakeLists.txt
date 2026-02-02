# Go bindings for OpenDAFF

cmake_minimum_required(VERSION 3.10)

# Build the C wrapper library for Go bindings
add_library(DAFFGoWrapper SHARED
    daff_go_wrapper.cpp
    daff_go_wrapper.h
)

target_link_libraries(DAFFGoWrapper
    DAFF
)

target_include_directories(DAFFGoWrapper PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_SOURCE_DIR}/include
)

set_target_properties(DAFFGoWrapper PROPERTIES
    OUTPUT_NAME "daffgowrapper"
    VERSION ${OPENDAFF_VERSION}
    SOVERSION ${OPENDAFF_VERSION_MAJOR}
)

# Install the wrapper library
install(TARGETS DAFFGoWrapper
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

# Install the header
install(FILES daff_go_wrapper.h
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/DAFF
)

# The actual Go build is handled by `go build` outside of CMake
# Users should run: cd bindings/go && go build
# Or use the justfile command: just build-go

message(STATUS "Go bindings C wrapper will be built")
message(STATUS "To build the Go package, run: cd bindings/go && go build")
