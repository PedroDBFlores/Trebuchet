#pragma once
#include <string>
#include <stdexcept>

// HttpMethod enum mirroring Rust's HttpMethod
enum class HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
};

// Convert HttpMethod to string
std::string to_string(HttpMethod method);

// Convert string to HttpMethod (throws on invalid input)
HttpMethod from_string(const std::string& method);
