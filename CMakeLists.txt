cmake_minimum_required(VERSION 3.20)
project(infra)
set(CMAKE_CXX_STANDARD 23)

find_package(CURL REQUIRED)
find_package(Threads REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(Boost REQUIRED context)

add_subdirectory(cpp/third_party/glog)
add_subdirectory(cpp/third_party/googletest)
add_subdirectory(cpp/third_party/range-v3)

set(FLAGS "-Wall -fmodules -fprebuilt-module-path=cpp/prebuilt --std=c++2b -fuse-ld=mold")
set(LINKS glog::glog tensorflow ${CURL_LIBRARIES} ${Boost_LIBRARIES} ${CMAKE_THREAD_LIBS_INIT} ${PostgreSQL_LIBRARIES} ${OPENSSL_LIBRARIES})
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${FLAGS}")

enable_testing()

include(GoogleTest)

include_directories(
  cpp/third_party/include
  cpp/src
  ${CURL_INCLUDE_DIRS}
  ${OPENSSL_INCLUDE_DIR}
  ${PostgreSQL_INCLUDE_DIRS}
)

add_executable(deepl cpp/deepl.main.cc)
add_executable(deepl.test cpp/deepl.test.cc)

gtest_discover_tests(deepl.test)

add_custom_target(symbols_generation COMMAND li_symbol_generator ${CMAKE_CURRENT_SOURCE_DIR})

add_dependencies(deepl symbols_generation)

target_link_libraries(deepl PUBLIC ${LINKS})

target_link_libraries(deepl.test PUBLIC gtest gtest_main gmock gmock_main ${LINKS})
