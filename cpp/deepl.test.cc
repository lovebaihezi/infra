#include "src/infra_core.hpp"
#include "gtest/gtest.h"
TEST(StdVariant, BasicUseage) {
    auto sum_type = std::variant<int, char>(char{0});
    ASSERT_FALSE(std::holds_alternative<int>(sum_type));
    ASSERT_TRUE(std::holds_alternative<char>(sum_type));
}
