#include "response.h"

ResponseError::ResponseError(const std::string& message)
    : std::runtime_error(message) {}

TimeoutError::TimeoutError(uint32_t timeout_ms)
    : ResponseError("Timeout after " + std::to_string(timeout_ms) + "ms"),
      timeout_ms(timeout_ms) {}

ConnectionError::ConnectionError(const std::string& message)
    : ResponseError("Connection error: " + message) {}
