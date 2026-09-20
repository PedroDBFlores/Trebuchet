#pragma once
#include <string>
#include "status.h"
#include "../tester/result.h"

// HttpResult mirrors Rust's HttpResult struct
// Extends LoadTestResult with HTTP-specific data
class HttpResult : public LoadTestResult {
public:
    HttpStatus status;
    std::string body;

    HttpResult(std::chrono::milliseconds latency, HttpStatus status, std::string body);
};
