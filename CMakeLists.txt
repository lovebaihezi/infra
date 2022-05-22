cmake_minimum_required(VERSION 3.21)
project(infra)
set(CMAKE_CXX_STANDARD 23)

set(ARMADILLO_INCLUDE_DIR ${PROJECT_SOURCE_DIR}/cpp/third_party/armadillo/include)
set(ARMADILLO_LIBRARY cpp/third_party/armadillo/lib)
set(ENSMALLEN_INCLUDE_DIR ${PROJECT_SOURCE_DIR}/cpp/third_party/ensmallen/include)
set(CEREAL_INCLUDE_DIR ${PROJECT_SOURCE_DIR}/cpp/third_party/cereal/include)

set(BUILD_EXAMPLES OFF)
set(BUILD_TESTS OFF)
set(BUILD_DOCS OFF)
set(BUILD_CLI_EXECUTABLES OFF)
set(BUILD_PYTHON_BINGDING OFF)
set(BUILD_JULIA_BINDINGS OFF)
set(BUILD_SHARED_LIBS OFF)
set(FORCE_CXX11 ON)

include_directories(
  cpp/third_party/include
  ${CURL_INCLUDE_DIRS}
  ${OPENSSL_INCLUDE_DIR}
  ${PostgreSQL_INCLUDE_DIRS}
  ${ARMADILLO_INCLUDE_DIR}
  ${ENSMALLEN_INCLUDE_DIR}
  ${CEREAL_INCLUDE_DIR}
  cpp/include
  cpp/src
)

find_package(CURL REQUIRED)
find_package(Threads REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(Boost REQUIRED context)

find_package(PkgConfig REQUIRED)
pkg_search_module(GRPC REQUIRED grpc)
pkg_search_module(GRPCPP REQUIRED grpc++>=1.44.0)

find_package( OpenCV REQUIRED )
include_directories( ${OpenCV_INCLUDE_DIRS} )

add_subdirectory(cpp/third_party/glog)
add_subdirectory(cpp/third_party/googletest)
add_subdirectory(cpp/third_party/range-v3)
add_subdirectory(cpp/third_party/fmt)
#add_subdirectory(cpp/third_party/armadillo)
#add_subdirectory(cpp/third_party/ensmallen)
#add_subdirectory(cpp/third_party/cereal)
#add_subdirectory(cpp/third_party/mlpack)

#add_subdirectory(cpp/third_party/matplotplusplus)
#add_subdirectory(cpp/third_party/dcmtk)

set(LINKS glog::glog fmt ${OpenCV_LIBS} ${CURL_LIBRARIES} ${Boost_LIBRARIES} ${CMAKE_THREAD_LIBS_INIT} ${PostgreSQL_LIBRARIES} ${OPENSSL_LIBRARIES})

enable_testing()

include(GoogleTest)

add_executable(deepl cpp/src/deepl.main.cc)

add_executable(deepl.test cpp/tests/deepl.test.cc)
add_executable(medical.lab7.test cpp/tests/medical.lab7.test.cc)

gtest_discover_tests(deepl.test)
gtest_discover_tests(medical.lab7.test)

add_custom_target(symbols_generation COMMAND li_symbol_generator ${CMAKE_CURRENT_SOURCE_DIR})

# add_dependencies(deepl symbols_generation)

target_link_libraries(deepl PUBLIC ${LINKS})

target_link_libraries(deepl.test PUBLIC gtest_main gmock_main ${LINKS})
target_link_libraries(medical.lab7.test PUBLIC gtest_main gmock_main ${LINKS})
