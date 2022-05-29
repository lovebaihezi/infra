#include "glog/logging.h"
#include <infra_core.hpp>
#include <opencv4/opencv2/opencv.hpp>

constexpr uint32_t AVG_AMOUNT = UINT32_MAX / 255;

namespace Medical {
template <> auto avg(auto &&views) noexcept -> double;
// template <> auto avgmax(auto &&views) noexcept -> size_t;
auto w(const auto &probabilities, auto k) noexcept -> double;
auto mu(const auto &probabilities, auto k) noexcept -> double;
auto threshold(const cv::Mat &image) noexcept -> uchar;
} // namespace Medical

template <> auto Medical::avg(auto &&views) noexcept -> double {
    double sum = 0;
    for (auto v : views | ranges::views::chunk(AVG_AMOUNT)) {
        sum += ranges::accumulate(v, 0.) / static_cast<double>(v.size());
    }
    return sum;
}

auto Medical::w(const auto &probabilities, auto k) noexcept -> double {
    const auto v =
        ranges::accumulate(probabilities | ranges::views::take(k), 0.);
    return v;
}

auto Medical::mu(const auto &probabilities, auto k) noexcept -> double {
    const auto v = ranges::accumulate(
        probabilities | ranges::views::enumerate |
            ranges::views::transform([](auto &&i_v) {
                return i_v.first * static_cast<double>(i_v.second);
            }) |
            ranges::views::take(k),
        0.);
    return v;
};

auto cal(const auto &probabilities, uint32_t k, double mu_t) -> double {
    const auto w_k = Medical::w(probabilities, k);
    const auto friction = mu_t * w_k - Medical::mu(probabilities, k);
    if (friction == 0) {
        return 0;
    }
    const auto v = std::pow(friction, 2.) / (w_k * (1. - w_k));
    return v;
};

auto Medical::threshold(const cv::Mat &image) noexcept -> uchar {
    const auto pixel_amount = image.rows * image.cols;
    // each grayscale level amount
    auto amount = std::array<uint32_t, 256>{};
    std::for_each(image.begin<uchar>(), image.end<uchar>(),
                  [&amount](auto c) { amount[c] += 1; });
    // each grayscale level probability
    auto probabilities = std::array<double, 256>{};
    for (auto i{0}; i < amount.size(); i += 1) {
        probabilities[i] =
            static_cast<double>(amount[i]) / static_cast<double>(pixel_amount);
    }
    const auto mu_t =
        ranges::accumulate(probabilities | ranges::views::enumerate |
                               ranges::views::transform([](auto &&i_v) {
                                   return static_cast<double>(i_v.first) *
                                          static_cast<double>(i_v.second);
                               }),
                           .0);
    auto x = ranges::views::iota(1, 256) |
             ranges::views::transform([&probabilities, mu_t](auto i) {
                 return cal(probabilities, i, mu_t);
             }) |
             ranges::to<std::vector>();
    auto k =
        static_cast<uchar>(std::max_element(x.begin(), x.end()) - x.begin());
    return k;
}

void lab7(int argc, const char *const argv[]) noexcept {
    CHECK(argc >= 3) << "no enough args!";
    const std::string_view str{argv[1]};
    cv::Mat image = cv::imread(str.data(), 1);
    cv::cvtColor(image, image, cv::COLOR_RGB2GRAY);
    CHECK(!!image.data) << "opencv can not read input image data!";
    const auto k = Medical::threshold(image);
    std::for_each(image.begin<uchar>(), image.end<uchar>(), [k](auto &v) {
        if (v >= k) {
            v = 255;
        } else {
            v = 0;
        }
    });
    cv::imwrite(argv[2], image);
}
