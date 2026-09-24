#pragma once
#include <memory>

template <typename TargetType, typename ResultType>
class TestExecutor {
public:
    virtual ResultType execute(std::shared_ptr<TargetType> target) = 0;
    virtual ~TestExecutor() = default;
};
