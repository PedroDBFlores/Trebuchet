#pragma once
#include <string>
#include <chrono>
#include <unordered_map>
#include <optional>
#include "method.h"
#include "status.h"
#include "../tester/target.h"

// HttpTarget mirrors Rust's HttpTarget struct
// Extends LoadTestTarget with HTTP-specific configuration
class HttpTarget : public LoadTestTarget {
public:
    std::string name;
    std::string url;
    HttpMethod method;
    std::unordered_map<std::string, std::string> headers;
    std::optional<std::string> body;
    HttpStatus desired_status;

    HttpTarget(std::string id, std::chrono::milliseconds timeout, uint32_t max_retries,
               std::string name, std::string url, HttpMethod method,
               std::unordered_map<std::string, std::string> headers,
               std::optional<std::string> body, HttpStatus desired_status);
};
