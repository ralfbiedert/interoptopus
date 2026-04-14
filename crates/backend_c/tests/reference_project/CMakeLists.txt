cmake_minimum_required(VERSION 3.20)
project(reference_project_c_tests LANGUAGES CXX)

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

add_executable(reference_project_c_tests main_test.cpp)

target_include_directories(reference_project_c_tests PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(reference_project_c_tests PRIVATE GTest::gtest_main)

if(UNIX AND NOT APPLE)
    target_link_libraries(reference_project_c_tests PRIVATE dl)
endif()

# Copy the Rust cdylib next to the test exe so it can be found at runtime.
if(WIN32)
    set(_rust_lib "${RUST_LIB_DIR}/reference_project_c.dll")
elseif(APPLE)
    set(_rust_lib "${RUST_LIB_DIR}/libreference_project_c.dylib")
else()
    set(_rust_lib "${RUST_LIB_DIR}/libreference_project_c.so")
endif()
add_custom_command(TARGET reference_project_c_tests POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different "${_rust_lib}" "$<TARGET_FILE_DIR:reference_project_c_tests>")

# On Unix, set RPATH to $ORIGIN so the exe finds the .so next to it.
if(NOT WIN32)
    set_target_properties(reference_project_c_tests PROPERTIES
        BUILD_RPATH "$ORIGIN"
        INSTALL_RPATH "$ORIGIN")
endif()

include(GoogleTest)
gtest_discover_tests(reference_project_c_tests DISCOVERY_MODE PRE_TEST)
