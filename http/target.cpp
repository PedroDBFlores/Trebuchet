#include "target.h"

HttpTarget::HttpTarget(std::string id, std::chrono::milliseconds timeout, uint32_t max_retries,
                       std::string name, std::string url, HttpMethod method,
                       std::unordered_map<std::string, std::string> headers,
                       std::optional<std::string> body, HttpStatus desired_status)
    : LoadTestTarget(std::move(id), timeout, max_retries),
      name(std::move(name)),
      url(std::move(url)),
      method(method),
      headers(std::move(headers)),
      body(std::move(body)),
      desired_status(std::move(desired_status)) {}
