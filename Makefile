# Trebuchet Makefile - Wrapper for CMake build system
# Use: make tests   (for clean build + run tests)
#      make          (for normal build)
#      make clean   (for clean)

BUILD_DIR ?= build/Release

.PHONY: all clean tests

all: setup
	cd $(BUILD_DIR) && cmake --build .

clean:
	@if [ -d "$(BUILD_DIR)" ]; then cd $(BUILD_DIR) && cmake --build . --target clean 2>/dev/null || true; fi

# Setup Conan and CMake configuration
setup:
	mkdir -p $(BUILD_DIR)
	cd $(BUILD_DIR) && conan install /Users/pflores/workspace/Trebuchet --build=missing 2>/dev/null || true
	cd $(BUILD_DIR) && cmake /Users/pflores/workspace/Trebuchet -DCMAKE_BUILD_TYPE=Release -DCMAKE_TOOLCHAIN_FILE=generators/conan_toolchain.cmake 2>/dev/null || true

# Clean build + run tests (what you asked for)
tests: clean setup
	cd $(BUILD_DIR) && cmake --build . --target tests
	cd $(BUILD_DIR) && ./tests
