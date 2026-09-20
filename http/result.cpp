#include "result.h"

HttpResult::HttpResult(std::chrono::milliseconds latency, HttpStatus status, std::string body)
    : LoadTestResult(latency), status(std::move(status)), body(std::move(body)) {}
