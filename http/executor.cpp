#include "executor.h"
#include "target.h"
#include "result.h"
#include "method.h"
#include "status.h"
#include "errors.h"
#include <curl/curl.h>
#include <stdexcept>
#include <sstream>

// Helper to convert headers map to curl slist
struct curl_slist* HttpExecutor::headers_to_slist(const std::unordered_map<std::string, std::string>& headers) {
    struct curl_slist* list = nullptr;
    for (const auto& [key, value] : headers) {
        std::string header = key + ": " + value;
        list = curl_slist_append(list, header.c_str());
    }
    return list;
}

void HttpExecutor::free_slist(struct curl_slist* list) {
    if (list) {
        curl_slist_free_all(list);
    }
}

HttpExecutor::HttpExecutor(std::chrono::milliseconds default_timeout)
    : curl_handle(curl_easy_init()), default_timeout(default_timeout) {
    if (!curl_handle) {
        throw std::runtime_error("Failed to initialize libcurl");
    }
    // Set default timeout
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT_MS, static_cast<long>(default_timeout.count()));
    curl_easy_setopt(curl_handle, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl_handle, CURLOPT_SSL_VERIFYPEER, 1L);
}

HttpExecutor::~HttpExecutor() {
    if (curl_handle) {
        curl_easy_cleanup(curl_handle);
    }
}

HttpResult HttpExecutor::execute(std::shared_ptr<HttpTarget> target) {
    auto start_time = std::chrono::steady_clock::now();
    
    std::string response_body;
    long response_code = 0;
    CURLcode curl_result;
    
    // Set URL
    curl_easy_setopt(curl_handle, CURLOPT_URL, target->url.c_str());
    
    // Set timeout
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT_MS, 
                     static_cast<long>(target->timeout.count()));
    
    // Set HTTP method
    switch(target->method) {
        case HttpMethod::GET:
            curl_easy_setopt(curl_handle, CURLOPT_HTTPGET, 1L);
            break;
        case HttpMethod::POST: {
            curl_easy_setopt(curl_handle, CURLOPT_POST, 1L);
            if (target->body) {
                curl_easy_setopt(curl_handle, CURLOPT_POSTFIELDS, target->body->c_str());
            }
            break;
        }
        case HttpMethod::PUT:
            curl_easy_setopt(curl_handle, CURLOPT_CUSTOMREQUEST, "PUT");
            if (target->body) {
                curl_easy_setopt(curl_handle, CURLOPT_POSTFIELDS, target->body->c_str());
            }
            break;
        case HttpMethod::DELETE:
            curl_easy_setopt(curl_handle, CURLOPT_CUSTOMREQUEST, "DELETE");
            break;
    }
    
    // Set headers
    struct curl_slist* headers_list = headers_to_slist(target->headers);
    if (headers_list) {
        curl_easy_setopt(curl_handle, CURLOPT_HTTPHEADER, headers_list);
    }
    
    // Set up response handling
    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, 
        [](char* data, size_t size, size_t nmemb, void* userdata) -> size_t {
            std::string* body = static_cast<std::string*>(userdata);
            body->append(data, size * nmemb);
            return size * nmemb;
        });
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, &response_body);
    
    // Execute the request
    curl_result = curl_easy_perform(curl_handle);
    
    // Get response code
    curl_easy_getinfo(curl_handle, CURLINFO_RESPONSE_CODE, &response_code);
    
    // Cleanup headers
    free_slist(headers_list);
    
    auto end_time = std::chrono::steady_clock::now();
    auto latency = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);
    
    // Check for errors
    if (curl_result != CURLE_OK) {
        if (curl_result == CURLE_OPERATION_TIMEDOUT) {
            throw http::TimeoutError(static_cast<uint32_t>(target->timeout.count()));
        }
        throw http::ConnectionError(curl_easy_strerror(curl_result));
    }
    
    // Create status and result
    HttpStatus status(static_cast<uint16_t>(response_code));
    
    return HttpResult(latency, status, response_body);
}
