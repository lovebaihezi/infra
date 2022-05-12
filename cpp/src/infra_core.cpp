#include <infra_core.hpp>
export module core;

constexpr auto str = "hello world!";

export constexpr auto hello() -> const char * { return str; }
