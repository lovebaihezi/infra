#include "glog/logging.h"
#include "src/infra_core.hpp"
auto main(int _argc, const char *const argv[]) noexcept -> int {
    google::InitGoogleLogging(argv[0]);
    return 0;
}
