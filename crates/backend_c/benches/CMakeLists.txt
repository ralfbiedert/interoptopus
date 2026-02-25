cmake_minimum_required(VERSION 3.20)
project(interoptopus_c_benchmarks LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Path to Rust cdylib output (passed via -DRUST_LIB_DIR=...)
if(NOT DEFINED RUST_LIB_DIR)
    message(FATAL_ERROR "RUST_LIB_DIR must be set to the directory containing the Rust cdylib")
endif()

include(FetchContent)
FetchContent_Declare(
  benchmark
  GIT_REPOSITORY https://github.com/google/benchmark.git
  GIT_TAG        v1.9.1
)
set(BENCHMARK_ENABLE_TESTING OFF CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(benchmark)

enable_testing()

add_executable(interoptopus_c_benchmarks main_bench.cpp)

target_include_directories(interoptopus_c_benchmarks PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/../tests/reference_project)
target_link_libraries(interoptopus_c_benchmarks PRIVATE benchmark::benchmark benchmark::benchmark_main)

if(UNIX AND NOT APPLE)
    target_link_libraries(interoptopus_c_benchmarks PRIVATE dl)
endif()

add_test(NAME benchmarks COMMAND interoptopus_c_benchmarks)

# Copy the Rust cdylib next to the benchmark exe so it can be found at runtime.
if(WIN32)
    set(_rust_lib "${RUST_LIB_DIR}/reference_project_c.dll")
elseif(APPLE)
    set(_rust_lib "${RUST_LIB_DIR}/libreference_project_c.dylib")
else()
    set(_rust_lib "${RUST_LIB_DIR}/libreference_project_c.so")
endif()
add_custom_command(TARGET interoptopus_c_benchmarks POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different "${_rust_lib}" "$<TARGET_FILE_DIR:interoptopus_c_benchmarks>")

# On Unix, set RPATH to $ORIGIN so the exe finds the .so next to it.
if(NOT WIN32)
    set_target_properties(interoptopus_c_benchmarks PROPERTIES
        BUILD_RPATH "$ORIGIN"
        INSTALL_RPATH "$ORIGIN")
endif()
