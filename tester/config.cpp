#include "config.h"

LoadTestConfig::LoadTestConfig(std::string id, std::vector<std::shared_ptr<LoadTestTarget>> targets)
    : id(std::move(id)), targets(std::move(targets)) {}
