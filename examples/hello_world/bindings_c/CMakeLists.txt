cmake_minimum_required(VERSION 3.20)
project(hello_world_c_test LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Path to Rust cdylib output (passed via -DRUST_LIB_DIR=...)
if(NOT DEFINED RUST_LIB_DIR)
    message(FATAL_ERROR "RUST_LIB_DIR must be set to the directory containing the Rust cdylib")
endif()

include(FetchContent)
FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        v1.15.2
)
FetchContent_MakeAvailable(googletest)

enable_testing()

add_executable(hello_world_c_test main_test.cpp)

target_include_directories(hello_world_c_test PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(hello_world_c_test PRIVATE GTest::gtest_main)

if(UNIX AND NOT APPLE)
    target_link_libraries(hello_world_c_test PRIVATE dl)
endif()

# Copy the Rust cdylib next to the test exe so it can be found at runtime.
if(WIN32)
    set(_rust_lib "${RUST_LIB_DIR}/hello_world.dll")
elseif(APPLE)
    set(_rust_lib "${RUST_LIB_DIR}/libhello_world.dylib")
else()
    set(_rust_lib "${RUST_LIB_DIR}/libhello_world.so")
endif()
add_custom_command(TARGET hello_world_c_test POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different "${_rust_lib}" "$<TARGET_FILE_DIR:hello_world_c_test>")

# On Unix, set RPATH to $ORIGIN so the exe finds the .so next to it.
if(NOT WIN32)
    set_target_properties(hello_world_c_test PROPERTIES
        BUILD_RPATH "$ORIGIN"
        INSTALL_RPATH "$ORIGIN")
endif()

include(GoogleTest)
gtest_discover_tests(hello_world_c_test DISCOVERY_MODE PRE_TEST)
