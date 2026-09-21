#include "result.h"

HttpResult::HttpResult(const std::chrono::milliseconds latency, HttpStatus status, std::string body)
    : LoadTestResult(latency), status(std::move(status)), body(std::move(body)) {}
