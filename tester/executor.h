#pragma once
#include <memory>
#include "target.h"
#include "result.h"

// Abstract base class for test executors (mirrors Rust's TestExecutor trait)
template <typename TargetType, typename ResultType>
class TestExecutor {
public:
    virtual ResultType execute(std::shared_ptr<TargetType> target) = 0;
    virtual ~TestExecutor() = default;
};
