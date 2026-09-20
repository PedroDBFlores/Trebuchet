#pragma once
#include <string>

// Simple CLI argument parsing (can be replaced with CLI11 later)
class CliParser {
public:
    struct Config {
        std::string config_file;
        bool verbose = false;
    };

    static Config parse(int argc, char** argv);
};
