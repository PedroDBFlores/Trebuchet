#pragma once
#include <memory>
#include <string>
#include <chrono>
#include <curl/curl.h>
#include "target.h"
#include "result.h"
#include "../tester/executor.h"

// HttpExecutor mirrors Rust's HttpExecutor struct
// Implements TestExecutor<HttpTarget, HttpResult>
class HttpExecutor : public TestExecutor<HttpTarget, HttpResult> {
private:
    CURL* curl_handle;
    std::chrono::milliseconds default_timeout;

    // Helper to convert headers map to curl slist
    static struct curl_slist* headers_to_slist(const std::unordered_map<std::string, std::string>& headers);
    
    // Helper to free curl slist
    static void free_slist(struct curl_slist* list);

public:
    HttpExecutor(std::chrono::milliseconds default_timeout = std::chrono::seconds(30));
    ~HttpExecutor() override;

    // Execute an HTTP request against the target
    HttpResult execute(std::shared_ptr<HttpTarget> target) override;
    
    // Delete copy constructor and assignment operator (CURL* is not copyable)
    HttpExecutor(const HttpExecutor&) = delete;
    HttpExecutor& operator=(const HttpExecutor&) = delete;
};
