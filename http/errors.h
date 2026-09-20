#pragma once
#include <string>
#include <stdexcept>
#include "../errors/response.h"

// HTTP-specific error types
namespace http {
    // Re-export base errors
    using ResponseError = ::ResponseError;
    using TimeoutError = ::TimeoutError;
    using ConnectionError = ::ConnectionError;
}
