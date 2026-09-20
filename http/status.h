#pragma once
#include <string>
#include <cstdint>

// HttpStatusType categorizes HTTP status codes
enum class HttpStatusType {
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
    Unknown
};

// HttpStatus represents an HTTP status code with its category
class HttpStatus {
public:
    HttpStatusType type;
    uint16_t code;

    // Create from status code
    explicit HttpStatus(uint16_t code);

    // Convert to string representation
    std::string to_string() const;
};
