cmake_minimum_required(VERSION 3.15)
project(Trebuchet LANGUAGES CXX)

# C++20 standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# For each target, explicitly set the standard
target_compile_options(Trebuchet PRIVATE -std=c++20)
target_compile_options(http PRIVATE -std=c++20)
target_compile_options(tester PRIVATE -std=c++20)

# Find required packages
find_package(CURL REQUIRED)

# Project structure
set(SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})

# Tester module (base classes)
add_library(tester STATIC
    tester/target.cpp
    tester/result.cpp
    tester/config.cpp
    tester/executor.cpp
)
target_include_directories(tester PUBLIC
    ${SOURCE_DIR}
)

# Errors module
add_library(errors STATIC
    errors/response.cpp
)
target_include_directories(errors PUBLIC
    ${SOURCE_DIR}
)
target_link_libraries(errors PRIVATE tester)

# HTTP module (depends on tester, errors, and CURL)
add_library(http STATIC
    http/method.cpp
    http/status.cpp
    http/result.cpp
    http/target.cpp
    http/errors.cpp
    http/executor.cpp
)
target_include_directories(http PUBLIC
    ${SOURCE_DIR}
)
target_link_libraries(http PRIVATE tester errors CURL::libcurl)

# CMD module (depends on tester and http)
add_library(cmd STATIC
    cmd/parser.cpp
)
target_include_directories(cmd PUBLIC
    ${SOURCE_DIR}
)
target_link_libraries(cmd PRIVATE tester http)

# Main executable
add_executable(Trebuchet main.cpp)
target_link_libraries(Trebuchet PRIVATE cmd http tester errors CURL::libcurl)
target_include_directories(Trebuchet PRIVATE
    ${SOURCE_DIR}
)

# Enable testing
enable_testing()

# Print summary
message(STATUS "Configuration summary:")
message(STATUS "  - C++ standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "  - libcurl found: ${CURL_FOUND}")
message(STATUS "  - libcurl include dirs: ${CURL_INCLUDE_DIRS}")
message(STATUS "  - Build type: ${CMAKE_BUILD_TYPE}")
