from conan import ConanFile
from conan.tools.cmake import CMakeToolchain, CMake, cmake_layout, CMakeDeps


class TrebuchetConan(ConanFile):
    name = "trebuchet"
    version = "0.1.0"
    settings = "os", "compiler", "build_type", "arch"

    # Dependencies - using system libcurl is fine, but we can also use conan's libcurl
    # For now, we'll rely on system libcurl found via find_package
    generators = "CMakeDeps", "CMakeToolchain"

    def layout(self):
        cmake_layout(self)

    def generate(self):
        deps = CMakeDeps(self)
        deps.generate()
        tc = CMakeToolchain(self)
        tc.generate()

    def build(self):
        cmake = CMake(self)
        cmake.configure()
        cmake.build()
