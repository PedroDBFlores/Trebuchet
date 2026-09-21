#include "result.h"

LoadTestResult::LoadTestResult(const std::chrono::milliseconds latency)
    : latency(latency) {}
