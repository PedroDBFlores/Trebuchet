#include "target.h"

LoadTestTarget::LoadTestTarget(std::string id, const std::chrono::milliseconds timeout, const uint32_t max_retries)
    : id(std::move(id)), timeout(timeout), max_retries(max_retries) {}
