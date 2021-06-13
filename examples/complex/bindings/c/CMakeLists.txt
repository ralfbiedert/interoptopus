
cmake_minimum_required(VERSION 3.14)
project(rust_playground C)

set(CMAKE_C_STANDARD 11)

include_directories(.)


add_executable(rust_playground
        app.c
        example_complex.h)

# It takes more fucking time figuring out how to link a DLL in C than to write the whole C exporter.
#target_link_libraries(rust_playground PUBLIC ../../../../target/debug/example_complex)
