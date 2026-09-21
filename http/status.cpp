#include "status.h"
#include <string>

HttpStatus::HttpStatus(const uint16_t code) : code(code) {
    if (code >= 100 && code <= 199) {
        type = HttpStatusType::Informational;
    } else if (code >= 200 && code <= 299) {
        type = HttpStatusType::Success;
    } else if (code >= 300 && code <= 399) {
        type = HttpStatusType::Redirection;
    } else if (code >= 400 && code <= 499) {
        type = HttpStatusType::ClientError;
    } else if (code >= 500 && code <= 599) {
        type = HttpStatusType::ServerError;
    } else {
        type = HttpStatusType::Unknown;
    }
}

std::string HttpStatus::to_string() const {
    switch(type) {
        case HttpStatusType::Informational:
            return "Informational (" + std::to_string(code) + ")";
        case HttpStatusType::Success:
            return "Success (" + std::to_string(code) + ")";
        case HttpStatusType::Redirection:
            return "Redirect (" + std::to_string(code) + ")";
        case HttpStatusType::ClientError:
            return "Client Error (" + std::to_string(code) + ")";
        case HttpStatusType::ServerError:
            return "Server Error (" + std::to_string(code) + ")";
        case HttpStatusType::Unknown:
            return "Unknown (" + std::to_string(code) + ")";
        default:
            return "Unknown (" + std::to_string(code) + ")";
    }
}
