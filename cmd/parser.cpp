#include "parser.h"
#include <iostream>
#include <string>

CliParser::Config CliParser::parse(int argc, char** argv) {
    Config config;
    
    for (int i = 1; i < argc; ++i) {
        std::string arg = argv[i];
        
        if (arg == "-c" || arg == "--config") {
            if (i + 1 < argc) {
                config.config_file = argv[++i];
            } else {
                std::cerr << "Error: --config requires a value\n";
                exit(1);
            }
        } else if (arg == "-v" || arg == "--verbose") {
            config.verbose = true;
        } else if (arg == "-h" || arg == "--help") {
            std::cout << "Usage: " << argv[0] << " [OPTIONS]\n"
                      << "Options:\n"
                      << "  -c, --config FILE  Configuration file path (required)\n"
                      << "  -v, --verbose      Verbose output\n"
                      << "  -h, --help         Show this help message\n";
            exit(0);
        }
    }
    
    if (config.config_file.empty()) {
        std::cerr << "Error: --config is required\n";
        exit(1);
    }
    
    return config;
}
