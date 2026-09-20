#include <iostream>
#include <memory>
#include "cmd/parser.h"
#include "http/executor.h"
#include "http/target.h"
#include "http/method.h"
#include "http/status.h"

int main(int argc, char** argv) {
    try {
        // Parse CLI arguments
        auto cli_config = CliParser::parse(argc, argv);
        
        if (cli_config.verbose) {
            std::cout << "Trebuchet HTTP Load Tester\n";
            std::cout << "Config file: " << cli_config.config_file << "\n";
        }
        
        // Create HTTP executor
        HttpExecutor executor;
        
        // Create a sample HTTP target (in real usage, this would come from config file)
        auto target = std::make_shared<HttpTarget>(
            "test-1",
            std::chrono::seconds(10),
            3,
            "Test API",
            "https://httpbin.org/get",
            HttpMethod::GET,
            std::unordered_map<std::string, std::string>{},
            std::nullopt,
            HttpStatus(200)
        );
        
        if (cli_config.verbose) {
            std::cout << "Executing request to: " << target->url << "\n";
            std::cout << "Method: " << to_string(target->method) << "\n";
        }
        
        // Execute the request
        auto result = executor.execute(target);
        
        // Print results
        std::cout << "Status: " << result.status.to_string() << "\n";
        std::cout << "Latency: " << result.latency.count() << "ms\n";
        std::cout << "Body length: " << result.body.size() << " bytes\n";
        
        if (cli_config.verbose) {
            std::cout << "\nResponse body:\n" << result.body << "\n";
        }
        
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }
}
