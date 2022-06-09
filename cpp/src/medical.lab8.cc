#include "glog/logging.h"
#include <infra_core.hpp>
#include <opencv4/opencv2/opencv.hpp>

struct Todo {};
struct Unreachable {};
auto kmeans(cv::Mat&& mat, int32_t k) -> std::vector<std::vector<std::array<uchar, 25>>> {
    auto [rows, cols] = std::pair{mat.rows, mat.cols};
    throw Todo{};
    throw Unreachable{};
}

auto main(int argc, char* args[]) -> int {
    google::InitGoogleLogging(args[0]);
    CHECK(argc > 1) << "no enough args!";
    const auto str = std::string_view {args[1]};
    cv::Mat image = cv::imread(str.data());
    cv::cvtColor(image, image, cv::COLOR_RGB2GRAY);
    try {
        auto result = kmeans(std::move(image), 4);
        assert(result.size() == 4);
    } catch (Todo e) {
        LOG(FATAL) << "unimplement!";
    } catch (Unreachable code) {
        LOG(FATAL) << "Unreachable code!";
    }
    return 0;
}

