#include "glog/logging.h"
#include "src/infra_core.hpp"
auto main(int argc, const char *const argv[0]) noexcept -> int {
    google::InitGoogleLogging(argv[0]);
    return 0;
}
