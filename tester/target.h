#pragma once
#include <chrono>
#include <cstdint>
#include <string>

class LoadTestTarget {
public:
    std::string id;
    std::chrono::milliseconds timeout;
    uint32_t max_retries;

    LoadTestTarget(std::string id, std::chrono::milliseconds timeout, uint32_t max_retries);
    virtual ~LoadTestTarget() = default;
};
