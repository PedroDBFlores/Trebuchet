#include "result.h"

LoadTestResult::LoadTestResult(std::chrono::milliseconds latency)
    : latency(latency) {}
