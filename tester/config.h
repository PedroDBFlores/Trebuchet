#pragma once
#include <string>
#include <vector>
#include <memory>
#include "target.h"

class LoadTestConfig {
public:
    std::string id;
    std::vector<std::shared_ptr<LoadTestTarget>> targets;

    LoadTestConfig(std::string id, std::vector<std::shared_ptr<LoadTestTarget>> targets);
};
