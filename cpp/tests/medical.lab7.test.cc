#include "medical.lab7.cc"
#include "gtest/gtest.h"
TEST(FunctionAvg, SafeAvgForGrayscale255NoOverflow) {
    auto data1 = ranges::views::ints(0, 256);
    auto result1 = Medical::avg(data1);
    EXPECT_EQ(result1, ranges::accumulate(data1, 0.) /
                           static_cast<double>(data1.size()));
    for (const auto v: ranges::views::ints(0, 10)) {
        fmt::print("{} ", v);
    }
}

// TEST(FunctionAvgmax, getMaxValueIndex) {
//     auto data1 = std::vector{1, 9, 27, 4, 28, 50, 24};
//     auto result1 = Medical::avgmax(data1);
//     EXPECT_EQ(result1, 5);
//     auto data2 = std::vector{23};
//     auto result2 = Medical::avgmax(data2);
//     EXPECT_EQ(result2, 0);
// }


