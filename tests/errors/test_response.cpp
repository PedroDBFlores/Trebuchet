#include <catch2/catch_test_macros.hpp>
#include <string>
#include "errors/response.h"

TEST_CASE("Test ConnectionError", "[errors]") {
    ConnectionError response("failed to connect");
    std::string expected = "Connection error: failed to connect";
    std::string actual = response.what();
    REQUIRE(actual == expected);
}

TEST_CASE("TEST ResponseError", "[errors]") {
    ResponseError response("Not Found");
    std::string expected = "Not Found";
    std::string actual = response.what();
    REQUIRE(actual == expected);
}
