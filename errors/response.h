#pragma once
#include <string>
#include <stdexcept>

// Base exception for response errors
class ResponseError : public std::runtime_error {
public:
    explicit ResponseError(const std::string& message);
};

// Timeout error
class TimeoutError : public ResponseError {
public:
    uint32_t timeout_ms;
    TimeoutError(uint32_t timeout_ms);
};

// Connection error
class ConnectionError : public ResponseError {
public:
    explicit ConnectionError(const std::string& message);
};
