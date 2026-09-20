#pragma once
#include <chrono>
#include <cstdint>

class LoadTestResult {
public:
    std::chrono::milliseconds latency;

    LoadTestResult(std::chrono::milliseconds latency);
};
