#include "target.h"

LoadTestTarget::LoadTestTarget(std::string id, std::chrono::milliseconds timeout, uint32_t max_retries)
    : id(std::move(id)), timeout(timeout), max_retries(max_retries) {}
