#include "glog/logging.h"
#include <infra_core.hpp>
#include <medical.lab7.cc>

auto main(int argc, const char *const argv[]) noexcept -> int {
    google::InitGoogleLogging(argv[0]);
    lab7(argc, argv);
    return 0;
}
