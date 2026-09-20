#include "method.h"
#include <string>
#include <stdexcept>

std::string to_string(HttpMethod method) {
    switch(method) {
        case HttpMethod::GET:    return "GET";
        case HttpMethod::POST:   return "POST";
        case HttpMethod::PUT:    return "PUT";
        case HttpMethod::DELETE: return "DELETE";
        default:
            throw std::runtime_error("Unknown HTTP method");
    }
}

HttpMethod from_string(const std::string& method) {
    if (method == "GET")     return HttpMethod::GET;
    if (method == "POST")    return HttpMethod::POST;
    if (method == "PUT")     return HttpMethod::PUT;
    if (method == "DELETE")  return HttpMethod::DELETE;
    throw std::runtime_error("Invalid HTTP method: " + method);
}
