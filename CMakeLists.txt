cmake_minimum_required(VERSION 3.15)

# Export compile commands for use with tools like clangd
# Set as CACHE before project() to place it in the source directory
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE BOOL "Generate compile_commands.json in source directory")

project(Trebuchet LANGUAGES CXX)

# C++20 standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Find required packages
find_package(CURL REQUIRED)

# Project structure
set(SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})

# Tester module (base classes)
file(GLOB TESTER_SOURCES "tester/*.cpp")
add_library(tester STATIC ${TESTER_SOURCES})
target_include_directories(tester PUBLIC ${SOURCE_DIR})

# Errors module
file(GLOB ERRORS_SOURCES "errors/*.cpp")
add_library(errors STATIC ${ERRORS_SOURCES})
target_include_directories(errors PUBLIC ${SOURCE_DIR})
target_link_libraries(errors PRIVATE tester)

# HTTP module (depends on tester, errors, and CURL)
file(GLOB HTTP_SOURCES "http/*.cpp")
add_library(http STATIC ${HTTP_SOURCES})
target_include_directories(http PUBLIC ${SOURCE_DIR})
target_link_libraries(http PRIVATE tester errors CURL::libcurl)

# CMD module (depends on tester and http)
file(GLOB CMD_SOURCES "cmd/*.cpp")
add_library(cmd STATIC ${CMD_SOURCES})
target_include_directories(cmd PUBLIC ${SOURCE_DIR})
target_link_libraries(cmd PRIVATE tester http)

# Main executable
file(GLOB MAIN_SOURCES "main.cpp")
add_executable(Trebuchet ${MAIN_SOURCES})
target_link_libraries(Trebuchet PRIVATE cmd http tester errors CURL::libcurl)
target_include_directories(Trebuchet PRIVATE ${SOURCE_DIR})

# Enable testing
enable_testing()

# Print summary
message(STATUS "Configuration summary:")
message(STATUS "  - C++ standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "  - libcurl found: ${CURL_FOUND}")
message(STATUS "  - libcurl include dirs: ${CURL_INCLUDE_DIRS}")
message(STATUS "  - Build type: ${CMAKE_BUILD_TYPE}")

# Tests
find_package(Catch2 3 REQUIRED)

# Collect all test source files from tests/ directory
file(GLOB TEST_SOURCES "tests/**/test*.cpp")

# Create test executable from all test files
add_executable(tests ${TEST_SOURCES})
target_link_libraries(tests PRIVATE Catch2::Catch2WithMain errors tester http CURL::libcurl)
target_include_directories(tests PRIVATE ${SOURCE_DIR})

# Register with CTest
add_test(NAME all-tests COMMAND tests)

# Copy compile_commands.json to project root for clangd
file(CREATE_LINK
    ${CMAKE_CURRENT_BINARY_DIR}/compile_commands.json
    ${CMAKE_CURRENT_SOURCE_DIR}/compile_commands.json
    SYMBOLIC
)
