#include "glog/logging.h"
#include <infra_core.hpp>
#include <opencv4/opencv2/opencv.hpp>
namespace Medical {
template <> auto avg(auto &&views) noexcept -> double;
// template <> auto avgmax(auto &&views) noexcept -> size_t;
auto w(const auto &probabilities, auto k) noexcept -> double;
auto mu(const auto &probabilities, auto k) noexcept -> double;
auto threshold(const cv::Mat &image) noexcept -> uchar;
} // namespace Medical
void lab7(int argc, const char *const args[]) noexcept;
